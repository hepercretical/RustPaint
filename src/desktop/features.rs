//! Editor commands at the Windows boundary. Modal calls never hold App borrows.
use super::*;
use std::ffi::c_void;
#[repr(C)]
struct ChooseColor {
    size: u32,
    owner: Handle,
    instance: Handle,
    result: u32,
    custom: *mut u32,
    flags: u32,
    data: isize,
    hook: usize,
    template: *const u16,
}
#[repr(C)]
struct PrintDialog {
    size: u32,
    owner: Handle,
    mode: Handle,
    names: Handle,
    dc: Handle,
    flags: u32,
    from: u16,
    to: u16,
    min: u16,
    max: u16,
    copies: u16,
    instance: Handle,
    data: isize,
    print_hook: usize,
    setup_hook: usize,
    print_template: *const u16,
    setup_template: *const u16,
    print_handle: Handle,
    setup_handle: Handle,
}
#[repr(C)]
struct DocInfo {
    size: i32,
    name: *const u16,
    output: *const u16,
    kind: *const u16,
    flags: u32,
}
#[link(name = "user32")]
unsafe extern "system" {
    fn OpenClipboard(hwnd: Handle) -> i32;
    fn CloseClipboard() -> i32;
    fn EmptyClipboard() -> i32;
    fn SetClipboardData(format: u32, data: Handle) -> Handle;
    fn GetClipboardData(format: u32) -> Handle;
    fn EnableWindow(hwnd: Handle, enable: i32) -> i32;
    fn SetForegroundWindow(hwnd: Handle) -> i32;
    fn SetFocus(hwnd: Handle) -> Handle;
    fn GetWindowTextLengthW(hwnd: Handle) -> i32;
    fn GetWindowTextW(hwnd: Handle, text: *mut u16, max: i32) -> i32;
    fn IsDialogMessageW(hwnd: Handle, message: *mut Message) -> i32;
    fn SendMessageW(hwnd: Handle, message: u32, wparam: usize, lparam: isize) -> isize;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalAlloc(flags: u32, bytes: usize) -> Handle;
    fn GlobalLock(handle: Handle) -> *mut c_void;
    fn GlobalUnlock(handle: Handle) -> i32;
    fn GlobalFree(handle: Handle) -> Handle;
    fn GlobalSize(handle: Handle) -> usize;
}
#[link(name = "comdlg32")]
unsafe extern "system" {
    fn ChooseColorW(dialog: *mut ChooseColor) -> i32;
    fn PrintDlgW(dialog: *mut PrintDialog) -> i32;
}
#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(dc: Handle) -> Handle;
    fn DeleteDC(dc: Handle) -> i32;
    fn CreateDIBSection(
        dc: Handle,
        info: *const c_void,
        usage: u32,
        bits: *mut *mut c_void,
        section: Handle,
        offset: u32,
    ) -> Handle;
    fn CreateFontW(
        height: i32,
        width: i32,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strikeout: u32,
        charset: u32,
        output: u32,
        clip: u32,
        quality: u32,
        pitch: u32,
        name: *const u16,
    ) -> Handle;
    fn GdiFlush() -> i32;
    fn StartDocW(dc: Handle, doc: *const DocInfo) -> i32;
    fn StartPage(dc: Handle) -> i32;
    fn EndPage(dc: Handle) -> i32;
    fn EndDoc(dc: Handle) -> i32;
    fn AbortDoc(dc: Handle) -> i32;
    fn GetDeviceCaps(dc: Handle, index: i32) -> i32;
}
#[repr(C, align(4))]
struct Header([u8; 40]);
struct Clipboard;
impl Drop for Clipboard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
}
struct Global(Handle);
impl Drop for Global {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe {
                GlobalFree(self.0);
            }
        }
    }
}
struct Dc(Handle);
impl Drop for Dc {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.0);
        }
    }
}
struct Object(Handle);
impl Drop for Object {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe {
                DeleteObject(self.0);
            }
        }
    }
}
pub(super) unsafe fn copy(hwnd: Handle, image: &Canvas) -> Result<(), String> {
    unsafe {
        let bytes = image.bmp()?;
        let dib = &bytes[14..];
        let mut memory = Global(GlobalAlloc(0x42, dib.len()));
        if memory.0 == 0 {
            return Err("clipboard allocation failed".into());
        }
        let pointer = GlobalLock(memory.0);
        if pointer.is_null() {
            return Err("clipboard memory lock failed".into());
        }
        std::ptr::copy_nonoverlapping(dib.as_ptr(), pointer.cast(), dib.len());
        GlobalUnlock(memory.0);
        if OpenClipboard(hwnd) == 0 {
            return Err("clipboard is busy; try again".into());
        }
        let _clipboard = Clipboard;
        if EmptyClipboard() == 0 || SetClipboardData(8, memory.0) == 0 {
            return Err("could not copy image to clipboard".into());
        }
        memory.0 = 0;
        Ok(())
    }
}
pub(super) unsafe fn paste(hwnd: Handle) -> Result<Canvas, String> {
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return Err("clipboard is busy; try again".into());
        }
        let _clipboard = Clipboard;
        let handle = GetClipboardData(8);
        if handle == 0 {
            return Err("clipboard does not contain a DIB bitmap".into());
        }
        let size = GlobalSize(handle);
        if !(40..=128_000_000).contains(&size) {
            return Err("invalid clipboard image size".into());
        }
        let pointer = GlobalLock(handle);
        if pointer.is_null() {
            return Err("clipboard memory lock failed".into());
        }
        let bytes = std::slice::from_raw_parts(pointer.cast::<u8>(), size).to_vec();
        GlobalUnlock(handle);
        let word = |i| u32::from_le_bytes(bytes[i..i + 4].try_into().expect("checked DIB header"));
        let header = word(0) as usize;
        let bits = u16::from_le_bytes([bytes[14], bytes[15]]);
        let colors = word(32) as usize;
        let compression = word(16);
        let masks = if header == 40 && compression == 3 {
            12
        } else {
            0
        };
        let palette = if colors > 0 {
            colors.checked_mul(4).ok_or("clipboard palette overflow")?
        } else if bits <= 8 {
            (1usize << bits) * 4
        } else {
            0
        };
        let offset = 14usize
            .checked_add(header)
            .and_then(|n| n.checked_add(masks))
            .and_then(|n| n.checked_add(palette))
            .ok_or("clipboard size overflow")?;
        let mut bmp = Vec::with_capacity(size + 14);
        bmp.extend_from_slice(b"BM");
        bmp.extend_from_slice(&((size + 14) as u32).to_le_bytes());
        bmp.extend_from_slice(&[0; 4]);
        bmp.extend_from_slice(&(offset as u32).to_le_bytes());
        bmp.extend_from_slice(&bytes);
        Canvas::from_bmp(&bmp)
    }
}
struct Input {
    edits: [Handle; 2],
    done: bool,
    result: Option<[String; 2]>,
}
unsafe fn read_text(hwnd: Handle) -> String {
    unsafe {
        let n = GetWindowTextLengthW(hwnd).clamp(0, 65535);
        let mut text = vec![0; n as usize + 1];
        let copied = GetWindowTextW(hwnd, text.as_mut_ptr(), text.len() as i32);
        String::from_utf16_lossy(&text[..copied.max(0) as usize])
    }
}
unsafe extern "system" fn input_proc(
    hwnd: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    unsafe {
        if message == 0x81 {
            let create = &*(lparam as *const Create);
            SetWindowLongPtrW(hwnd, -21, create.parameter as isize);
            return 1;
        }
        if message == 0x82 {
            SetWindowLongPtrW(hwnd, -21, 0);
            return DefWindowProcW(hwnd, message, wparam, lparam);
        }
        let pointer = GetWindowLongPtrW(hwnd, -21) as *const RefCell<Input>;
        if pointer.is_null() {
            return DefWindowProcW(hwnd, message, wparam, lparam);
        }
        let cell = &*pointer;
        match message {
            0x111 if wparam & 0xffff == 1 => {
                let edits = cell.borrow().edits;
                let values = [read_text(edits[0]), read_text(edits[1])];
                let mut input = cell.borrow_mut();
                input.result = Some(values);
                input.done = true;
                0
            }
            0x111 if wparam & 0xffff == 2 => {
                cell.borrow_mut().done = true;
                0
            }
            0x10 => {
                cell.borrow_mut().done = true;
                0
            }
            _ => DefWindowProcW(hwnd, message, wparam, lparam),
        }
    }
}
unsafe fn prompt(
    owner: Handle,
    caption: &str,
    labels: [&str; 2],
    values: [&str; 2],
) -> Result<Option<[String; 2]>, String> {
    unsafe {
        let state = Box::new(RefCell::new(Input {
            edits: [0; 2],
            done: false,
            result: None,
        }));
        let instance = GetModuleHandleW(std::ptr::null());
        let class = wide("RustPaintInput");
        let wc = WindowClass {
            style: 3,
            procedure: Some(input_proc),
            class_extra: 0,
            window_extra: 0,
            instance,
            icon: 0,
            cursor: LoadCursorW(0, 32512usize as *const u16),
            background: 16,
            menu: std::ptr::null(),
            name: class.as_ptr(),
        };
        if RegisterClassW(&wc) == 0 && std::io::Error::last_os_error().raw_os_error() != Some(1410)
        {
            return Err("could not register input dialog".into());
        }
        let hwnd = CreateWindowExW(
            0x101,
            class.as_ptr(),
            wide(caption).as_ptr(),
            0x80c80000,
            180,
            160,
            500,
            248,
            owner,
            0,
            instance,
            (&*state as *const RefCell<Input>).cast_mut().cast(),
        );
        if hwnd == 0 {
            return Err("could not create input dialog".into());
        }
        let mut handles = [0; 2];
        for i in 0..2 {
            CreateWindowExW(
                0,
                wide("STATIC").as_ptr(),
                wide(labels[i]).as_ptr(),
                0x50000000,
                16,
                22 + i as i32 * 58,
                95,
                24,
                hwnd,
                0,
                instance,
                std::ptr::null_mut(),
            );
            handles[i] = CreateWindowExW(
                0x200,
                wide("EDIT").as_ptr(),
                wide(values[i]).as_ptr(),
                0x50810080,
                120,
                18 + i as i32 * 58,
                344,
                30,
                hwnd,
                100 + i as isize,
                instance,
                std::ptr::null_mut(),
            );
            SendMessageW(handles[i], 0x30, GetStockObject(17) as usize, 1);
            SendMessageW(handles[i], 0xc5, 65535, 0);
        }
        state.borrow_mut().edits = handles;
        for (id, name, x) in [(1, "OK", 270), (2, "Cancel", 372)] {
            let button = CreateWindowExW(
                0,
                wide("BUTTON").as_ptr(),
                wide(name).as_ptr(),
                0x50010000 | if id == 1 { 1 } else { 0 },
                x,
                148,
                92,
                30,
                hwnd,
                id,
                instance,
                std::ptr::null_mut(),
            );
            SendMessageW(button, 0x30, GetStockObject(17) as usize, 1);
        }
        EnableWindow(owner, 0);
        ShowWindow(hwnd, 5);
        SetFocus(handles[0]);
        SendMessageW(handles[0], 0xb1, 0, -1);
        let mut message = Message::default();
        while !state.borrow().done {
            let result = GetMessageW(&mut message, 0, 0, 0);
            if result <= 0 {
                if result == 0 {
                    PostQuitMessage(message.wparam as i32);
                }
                break;
            }
            if IsDialogMessageW(hwnd, &mut message) == 0 {
                TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
        DestroyWindow(hwnd);
        EnableWindow(owner, 1);
        SetForegroundWindow(owner);
        let result = state.borrow_mut().result.take();
        Ok(result)
    }
}
pub(super) unsafe fn text(
    hwnd: Handle,
    cell: &RefCell<App>,
    point: (i32, i32),
) -> Result<(), String> {
    unsafe {
        let Some(values) = prompt(hwnd, "Text", ["Text", "Font size"], ["", "24"])? else {
            return Ok(());
        };
        if values[0].is_empty() {
            return Ok(());
        }
        let size = values[1]
            .parse::<i32>()
            .ok()
            .filter(|n| (6..=300).contains(n))
            .ok_or("font size must be 6 through 300")?;
        let output = {
            let app = cell.borrow();
            raster_text(&app.canvas, &values[0], point, size, app.color)?
        };
        let mut app = cell.borrow_mut();
        app.canvas.replace_pixels(output)?;
        app.dirty = true;
        Ok(())
    }
}
unsafe fn raster_text(
    canvas: &Canvas,
    words: &str,
    point: (i32, i32),
    size: i32,
    color: u32,
) -> Result<Vec<u32>, String> {
    unsafe {
        let (header, pixels, width, height) = (
            Header(canvas.dib_header()),
            canvas.pixels(),
            canvas.width(),
            canvas.height(),
        );
        let dc = Dc(CreateCompatibleDC(0));
        if dc.0 == 0 {
            return Err("could not create text drawing context".into());
        }
        let mut pointer = std::ptr::null_mut();
        let bitmap = Object(CreateDIBSection(
            dc.0,
            header.0.as_ptr().cast(),
            0,
            &mut pointer,
            0,
            0,
        ));
        if bitmap.0 == 0 || pointer.is_null() {
            return Err("could not create text image".into());
        }
        std::ptr::copy_nonoverlapping(pixels.as_ptr(), pointer.cast::<u32>(), pixels.len());
        let previous = SelectObject(dc.0, bitmap.0);
        let font = Object(CreateFontW(
            -size,
            0,
            0,
            0,
            400,
            0,
            0,
            0,
            1,
            0,
            0,
            4,
            0,
            wide("Segoe UI").as_ptr(),
        ));
        if font.0 == 0 {
            SelectObject(dc.0, previous);
            return Err("could not create text font".into());
        }
        let old_font = SelectObject(dc.0, font.0);
        SetBkMode(dc.0, 1);
        SetTextColor(dc.0, native_color(color));
        let mut rect = Rect {
            left: point.0,
            top: point.1,
            right: width as i32,
            bottom: height as i32,
        };
        let text = wide(words);
        DrawTextW(dc.0, text.as_ptr(), -1, &mut rect, 0x10 | 0x800);
        GdiFlush();
        let output = std::slice::from_raw_parts(pointer.cast::<u32>(), pixels.len()).to_vec();
        SelectObject(dc.0, old_font);
        SelectObject(dc.0, previous);
        Ok(output)
    }
}
unsafe fn print(hwnd: Handle, canvas: &Canvas) -> Result<(), String> {
    unsafe {
        let mut dialog: PrintDialog = std::mem::zeroed();
        dialog.size = std::mem::size_of::<PrintDialog>() as u32;
        dialog.owner = hwnd;
        dialog.flags = 0x4010c;
        dialog.min = 1;
        dialog.max = 1;
        dialog.from = 1;
        dialog.to = 1;
        dialog.copies = 1;
        let result = PrintDlgW(&mut dialog);
        let _mode = Global(dialog.mode);
        let _names = Global(dialog.names);
        if result == 0 {
            let code = CommDlgExtendedError();
            return if code == 0 {
                Ok(())
            } else {
                Err(format!("Print dialog failed: {code:x}"))
            };
        }
        let dc = Dc(dialog.dc);
        let name = wide("Paint picture");
        let doc = DocInfo {
            size: std::mem::size_of::<DocInfo>() as i32,
            name: name.as_ptr(),
            output: std::ptr::null(),
            kind: std::ptr::null(),
            flags: 0,
        };
        if StartDocW(dc.0, &doc) <= 0 {
            return Err("printer could not start document".into());
        }
        if StartPage(dc.0) <= 0 {
            AbortDoc(dc.0);
            return Err("printer could not start page".into());
        }
        let (page_w, page_h) = (GetDeviceCaps(dc.0, 8), GetDeviceCaps(dc.0, 10));
        let scale =
            (page_w as f64 / canvas.width() as f64).min(page_h as f64 / canvas.height() as f64);
        let (w, h) = (
            (canvas.width() as f64 * scale) as i32,
            (canvas.height() as f64 * scale) as i32,
        );
        let header = Header(canvas.dib_header());
        let copied = StretchDIBits(
            dc.0,
            (page_w - w) / 2,
            (page_h - h) / 2,
            w,
            h,
            0,
            0,
            canvas.width() as i32,
            canvas.height() as i32,
            canvas.pixels().as_ptr().cast(),
            header.0.as_ptr().cast(),
            0,
            0xcc0020,
        );
        if copied <= 0 || EndPage(dc.0) <= 0 || EndDoc(dc.0) <= 0 {
            AbortDoc(dc.0);
            return Err("printer did not complete the picture".into());
        }
        Ok(())
    }
}
pub(super) unsafe fn perform(hwnd: Handle, cell: &RefCell<App>, id: u16) -> Result<(), String> {
    unsafe {
        match id {
            10 | 11 => {
                let image = {
                    let app = cell.borrow();
                    if let Some((a, b)) = app.selection {
                        app.canvas.region(a, b)?
                    } else {
                        Canvas::from_pixels(
                            app.canvas.width(),
                            app.canvas.height(),
                            app.canvas.pixels().to_vec(),
                        )?
                    }
                };
                copy(hwnd, &image)?;
                if id == 11 {
                    let mut app = cell.borrow_mut();
                    if let Some((a, b)) = app.selection {
                        app.canvas.erase_region(a, b);
                    } else {
                        app.canvas.clear();
                    }
                    app.dirty = true;
                }
            }
            12 => {
                let image = paste(hwnd)?;
                let mut app = cell.borrow_mut();
                let (a, b) = app.selection.unwrap_or(((0, 0), (0, 0)));
                let (x, y) = (a.0.min(b.0), a.1.min(b.1));
                app.canvas.paste_expanding(&image, x, y)?;
                app.selection = Some((
                    (x, y),
                    (x + image.width() as i32 - 1, y + image.height() as i32 - 1),
                ));
                app.dirty = true;
            }
            13 => {
                let mut app = cell.borrow_mut();
                app.tool = Tool::Select;
                app.selection = Some((
                    (0, 0),
                    (
                        app.canvas.width() as i32 - 1,
                        app.canvas.height() as i32 - 1,
                    ),
                ));
            }
            14 => {
                let mut app = cell.borrow_mut();
                let (a, b) = app
                    .selection
                    .ok_or("Drag a rectangular selection before choosing Crop")?;
                app.canvas.crop(a, b)?;
                app.selection = None;
                app.dirty = true;
            }
            15 | 16 => {
                let defaults = {
                    let app = cell.borrow();
                    [
                        app.canvas.width().to_string(),
                        app.canvas.height().to_string(),
                    ]
                };
                if let Some(values) = prompt(
                    hwnd,
                    if id == 15 {
                        "Resize picture"
                    } else {
                        "Canvas size"
                    },
                    ["Width (pixels)", "Height (pixels)"],
                    [&defaults[0], &defaults[1]],
                )? {
                    let w = values[0].parse().map_err(|_| "invalid width")?;
                    let h = values[1].parse().map_err(|_| "invalid height")?;
                    let mut app = cell.borrow_mut();
                    app.canvas.resize(w, h, id == 15)?;
                    app.selection = None;
                    app.dirty = true;
                }
            }
            17..=21 => {
                let mut app = cell.borrow_mut();
                match id {
                    17 => app.canvas.rotate_right(),
                    18 => app.canvas.rotate(3),
                    19 => app.canvas.rotate(2),
                    20 => app.canvas.flip(true),
                    21 => app.canvas.flip(false),
                    _ => {}
                }
                app.selection = None;
                app.dirty = true;
            }
            22 => {
                let mut app = cell.borrow_mut();
                if let Some((a, b)) = app.selection {
                    app.canvas.erase_region(a, b);
                    app.selection = None;
                    app.dirty = true;
                }
            }
            23 => {
                let mut custom = [0u32; 16];
                let color = cell.borrow().color;
                let mut dialog: ChooseColor = std::mem::zeroed();
                dialog.size = std::mem::size_of::<ChooseColor>() as u32;
                dialog.owner = hwnd;
                dialog.result = native_color(color);
                dialog.custom = custom.as_mut_ptr();
                dialog.flags = 3;
                if ChooseColorW(&mut dialog) != 0 {
                    cell.borrow_mut().color = native_color(dialog.result);
                }
            }
            24 => {
                let mut app = cell.borrow_mut();
                app.canvas.invert();
                app.dirty = true;
            }
            30..=36 => {
                let mut app = cell.borrow_mut();
                let center = viewport_center(&app);
                zoom_about(
                    &mut app,
                    [0, 25, 50, 100, 200, 400, 800][(id - 30) as usize],
                    center,
                );
            }
            51..=53 => {
                cell.borrow_mut().theme = match id {
                    51 => appearance::Theme::Light,
                    52 => appearance::Theme::Dark,
                    _ => appearance::Theme::System,
                };
            }
            54 => {
                let mut app = cell.borrow_mut();
                app.theme = if app.theme.dark(app.system_dark) {
                    appearance::Theme::Light
                } else {
                    appearance::Theme::Dark
                };
            }
            57 => {
                let mut app = cell.borrow_mut();
                app.toolbar_collapsed = !app.toolbar_collapsed;
            }
            58 => {
                let mut app = cell.borrow_mut();
                app.grid = !app.grid;
            }
            61 | 62 => {
                let mut app = cell.borrow_mut();
                let current = app.view.0 as u64 * 100 / app.canvas.width() as u64;
                let steps = [25, 50, 100, 200, 400, 800];
                let zoom = if id == 62 {
                    steps
                        .into_iter()
                        .find(|s| u64::from(*s) > current)
                        .unwrap_or(800)
                } else {
                    steps
                        .into_iter()
                        .rev()
                        .find(|s| u64::from(*s) < current)
                        .unwrap_or(25)
                };
                let center = viewport_center(&app);
                zoom_about(&mut app, zoom, center);
            }
            40 => {
                let image = {
                    let app = cell.borrow();
                    Canvas::from_pixels(
                        app.canvas.width(),
                        app.canvas.height(),
                        app.canvas.pixels().to_vec(),
                    )?
                };
                print(hwnd, &image)?;
            }
            _ => return Err("editor command not implemented".into()),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[cfg(target_pointer_width = "64")]
    fn windows_dialog_structures_match_x64_layout() {
        assert_eq!(std::mem::size_of::<ChooseColor>(), 72);
        assert_eq!(std::mem::size_of::<PrintDialog>(), 120);
        assert_eq!(std::mem::size_of::<DocInfo>(), 40);
    }
    #[test]
    fn native_unicode_text_rasterization_preserves_image_extent() {
        let canvas = Canvas::new(240, 80).unwrap();
        let pixels =
            unsafe { raster_text(&canvas, "Paint Ω & Rust", (8, 8), 24, 0x000000) }.unwrap();
        assert_eq!(pixels.len(), canvas.pixels().len());
        assert_eq!(pixels[0] & 0xffffff, 0xffffff);
        assert!(pixels.iter().any(|p| p & 0xffffff != 0xffffff));
    }
}
