//! Windows desktop host. All native handles and pointer lifetimes stay here.
use mspaint_port::canvas::{Canvas, Tool};
use std::{cell::RefCell, ffi::c_void, path::PathBuf};
mod appearance;
mod features;
mod rendering;
type Handle = isize;
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
struct MouseTracking {
    size: u32,
    flags: u32,
    hwnd: Handle,
    hover: u32,
}
#[repr(C)]
struct MinMax {
    reserved: Point,
    max_size: Point,
    max_position: Point,
    min_track: Point,
    max_track: Point,
}
#[repr(C)]
#[derive(Default)]
struct Message {
    hwnd: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    point: Point,
    private: u32,
}
#[repr(C)]
struct WindowClass {
    style: u32,
    procedure: Option<unsafe extern "system" fn(Handle, u32, usize, isize) -> isize>,
    class_extra: i32,
    window_extra: i32,
    instance: Handle,
    icon: Handle,
    cursor: Handle,
    background: Handle,
    menu: *const u16,
    name: *const u16,
}
#[repr(C)]
#[derive(Default)]
struct Paint {
    dc: Handle,
    erase: i32,
    rect: Rect,
    restore: i32,
    update: i32,
    reserved: [u8; 32],
}
#[repr(C)]
struct Create {
    parameter: *mut c_void,
    instance: Handle,
    menu: Handle,
    parent: Handle,
    cy: i32,
    cx: i32,
    y: i32,
    x: i32,
    style: i32,
    name: *const u16,
    class: *const u16,
    extended: u32,
}
#[repr(C)]
struct FileDialog {
    size: u32,
    owner: Handle,
    instance: Handle,
    filter: *const u16,
    custom_filter: *mut u16,
    max_custom: u32,
    index: u32,
    file: *mut u16,
    max_file: u32,
    title_file: *mut u16,
    max_title: u32,
    initial: *const u16,
    title: *const u16,
    flags: u32,
    offset: u16,
    extension: u16,
    default_extension: *const u16,
    data: isize,
    hook: usize,
    template: *const u16,
    reserved: *mut c_void,
    reserved_word: u32,
    flags_ex: u32,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn DrawFocusRect(dc: Handle, rect: *const Rect) -> i32;
    fn RegisterClassW(class: *const WindowClass) -> u16;
    fn CreateWindowExW(
        ex: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: Handle,
        menu: Handle,
        instance: Handle,
        param: *mut c_void,
    ) -> Handle;
    fn DefWindowProcW(hwnd: Handle, message: u32, wparam: usize, lparam: isize) -> isize;
    fn GetMessageW(message: *mut Message, hwnd: Handle, min: u32, max: u32) -> i32;
    fn TranslateMessage(message: *const Message) -> i32;
    fn DispatchMessageW(message: *const Message) -> isize;
    fn ShowWindow(hwnd: Handle, command: i32) -> i32;
    fn UpdateWindow(hwnd: Handle) -> i32;
    fn DestroyWindow(hwnd: Handle) -> i32;
    fn PostQuitMessage(code: i32);
    fn LoadCursorW(instance: Handle, name: *const u16) -> Handle;
    fn LoadIconW(instance: Handle, name: *const u16) -> Handle;
    fn SetWindowLongPtrW(hwnd: Handle, index: i32, value: isize) -> isize;
    fn GetWindowLongPtrW(hwnd: Handle, index: i32) -> isize;
    fn BeginPaint(hwnd: Handle, paint: *mut Paint) -> Handle;
    fn EndPaint(hwnd: Handle, paint: *const Paint) -> i32;
    fn GetClientRect(hwnd: Handle, rect: *mut Rect) -> i32;
    fn ScreenToClient(hwnd: Handle, point: *mut Point) -> i32;
    fn GetCursorPos(point: *mut Point) -> i32;
    fn SetCursor(cursor: Handle) -> Handle;
    fn TrackMouseEvent(event: *mut MouseTracking) -> i32;
    fn IsWindowVisible(hwnd: Handle) -> i32;
    fn FillRect(dc: Handle, rect: *const Rect, brush: Handle) -> i32;
    fn FrameRect(dc: Handle, rect: *const Rect, brush: Handle) -> i32;
    fn DrawTextW(dc: Handle, text: *const u16, count: i32, rect: *mut Rect, format: u32) -> i32;
    fn InvalidateRect(hwnd: Handle, rect: *const Rect, erase: i32) -> i32;
    fn SetCapture(hwnd: Handle) -> Handle;
    fn ReleaseCapture() -> i32;
    fn SetWindowTextW(hwnd: Handle, text: *const u16) -> i32;
    fn MessageBoxW(hwnd: Handle, text: *const u16, title: *const u16, kind: u32) -> i32;
    fn CreateMenu() -> Handle;
    fn CreatePopupMenu() -> Handle;
    fn AppendMenuW(menu: Handle, flags: u32, id: usize, label: *const u16) -> i32;
    fn SetMenu(hwnd: Handle, menu: Handle) -> i32;
    fn DestroyMenu(menu: Handle) -> i32;
    fn GetKeyState(key: i32) -> i16;
}
#[link(name = "gdi32")]
unsafe extern "system" {
    fn SaveDC(dc: Handle) -> i32;
    fn RestoreDC(dc: Handle, saved: i32) -> i32;
    fn IntersectClipRect(dc: Handle, left: i32, top: i32, right: i32, bottom: i32) -> i32;
    fn CreateSolidBrush(color: u32) -> Handle;
    fn DeleteObject(object: Handle) -> i32;
    fn SetBkMode(dc: Handle, mode: i32) -> i32;
    fn SetTextColor(dc: Handle, color: u32) -> u32;
    fn SelectObject(dc: Handle, object: Handle) -> Handle;
    fn GetStockObject(object: i32) -> Handle;
    fn StretchDIBits(
        dc: Handle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        sx: i32,
        sy: i32,
        sw: i32,
        sh: i32,
        bits: *const c_void,
        info: *const c_void,
        usage: u32,
        operation: u32,
    ) -> i32;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> Handle;
}
#[link(name = "shell32")]
unsafe extern "system" {
    fn DragAcceptFiles(hwnd: Handle, accept: i32);
    fn DragQueryFileW(drop: Handle, index: u32, name: *mut u16, length: u32) -> u32;
    fn DragFinish(drop: Handle);
}
#[link(name = "comdlg32")]
unsafe extern "system" {
    fn GetOpenFileNameW(dialog: *mut FileDialog) -> i32;
    fn GetSaveFileNameW(dialog: *mut FileDialog) -> i32;
    fn CommDlgExtendedError() -> u32;
}

const COLORS: [u32; 20] = [
    0x000000, 0x808080, 0x800000, 0xff0000, 0xff8000, 0xffff00, 0x008000, 0x00a2e8, 0x3f48cc,
    0xa349a4, 0xffffff, 0xc0c0c0, 0xb97a57, 0xffaec9, 0xffc90e, 0xefe4b0, 0xb5e61d, 0x99d9ea,
    0x7092be, 0xc8bfe7,
];
const TOOLS: [(Tool, &str); 10] = [
    (Tool::Pencil, "Pencil"),
    (Tool::Eraser, "Eraser"),
    (Tool::Fill, "Fill"),
    (Tool::Picker, "Pick color"),
    (Tool::Line, "Line"),
    (Tool::Rectangle, "Rectangle"),
    (Tool::Ellipse, "Ellipse"),
    (Tool::Brush, "Brush"),
    (Tool::Select, "Select"),
    (Tool::Text, "Text"),
];
struct App {
    back_buffer: rendering::BackBuffer,
    canvas: Canvas,
    color: u32,
    size: i32,
    tool: Tool,
    drawing: bool,
    start: (i32, i32),
    last: (i32, i32),
    dirty: bool,
    path: Option<PathBuf>,
    view: (i32, i32),
    selection: Option<((i32, i32), (i32, i32))>,
    zoom: u32,
    pan: (i32, i32),
    moving: Option<((i32, i32), (i32, i32))>,
    secondary: u32,
    stroke_color: u32,
    client: (i32, i32),
    origin: (i32, i32),
    theme: appearance::Theme,
    system_dark: bool,
    toolbar_collapsed: bool,
    hover: Option<u16>,
    grid: bool,
    zoom_drag: bool,
    constrain: bool,
}
fn layout(app: &mut App, width: i32, height: i32) {
    app.client = (width, height);
    let toolbar = appearance::Layout::new(width, app.toolbar_collapsed);
    let (available_w, available_h) = ((width - 40).max(1), (height - toolbar.height - 60).max(1));
    let scale = ((available_w as f64 / app.canvas.width() as f64)
        .min(available_h as f64 / app.canvas.height() as f64))
    .min(1.0);
    let scale = if app.zoom == 0 {
        scale
    } else {
        app.zoom as f64 / 100.0
    };
    app.view = (
        (app.canvas.width() as f64 * scale).round().max(1.0) as i32,
        (app.canvas.height() as f64 * scale).round().max(1.0) as i32,
    );
    app.pan.0 = app.pan.0.clamp(0, (app.view.0 - available_w).max(0));
    app.pan.1 = app.pan.1.clamp(0, (app.view.1 - available_h).max(0));
    app.origin = (
        20 + (available_w - app.view.0).max(0) / 2,
        toolbar.height + 20 + (available_h - app.view.1).max(0) / 2,
    );
}
fn image_point(app: &App, x: i32, y: i32) -> (i32, i32) {
    (
        ((x - app.origin.0 + app.pan.0) as f64 * app.canvas.width() as f64 / app.view.0 as f64)
            .floor() as i32,
        ((y - app.origin.1 + app.pan.1) as f64 * app.canvas.height() as f64 / app.view.1 as f64)
            .floor() as i32,
    )
}
fn zoom_about(app: &mut App, zoom: u32, anchor: (i32, i32)) {
    let before = (
        (anchor.0 - app.origin.0 + app.pan.0) as f64 * app.canvas.width() as f64
            / app.view.0 as f64,
        (anchor.1 - app.origin.1 + app.pan.1) as f64 * app.canvas.height() as f64
            / app.view.1 as f64,
    );
    app.zoom = zoom;
    let (w, h) = app.client;
    layout(app, w, h);
    app.pan = (
        (app.origin.0 as f64 + before.0 * app.view.0 as f64 / app.canvas.width() as f64
            - anchor.0 as f64)
            .round() as i32,
        (app.origin.1 as f64 + before.1 * app.view.1 as f64 / app.canvas.height() as f64
            - anchor.1 as f64)
            .round() as i32,
    );
    layout(app, w, h);
}
fn viewport_center(app: &App) -> (i32, i32) {
    (
        app.client.0 / 2,
        (appearance::Layout::new(app.client.0, app.toolbar_collapsed).height + app.client.1 - 28)
            / 2,
    )
}
enum Action {
    None,
    New,
    Open,
    Save(bool),
    Close,
    Feature(u16),
    Text((i32, i32)),
    OpenPath(PathBuf),
}
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}
fn native_color(rgb: u32) -> u32 {
    ((rgb & 0xff) << 16) | (rgb & 0xff00) | ((rgb >> 16) & 0xff)
}

// SAFETY: helpers below are called only with live USER32/GDI handles on the UI
// thread. String buffers live across each synchronous call. All image memory
// is owned by Canvas, and its dimensions cannot be mutated independently.
unsafe fn block(dc: Handle, rect: Rect, rgb: u32, border: bool) {
    unsafe {
        let brush = CreateSolidBrush(native_color(rgb));
        if brush != 0 {
            if border {
                FrameRect(dc, &rect, brush);
            } else {
                FillRect(dc, &rect, brush);
            }
            DeleteObject(brush);
        }
    }
}
unsafe fn paint(hwnd: Handle, app: &mut App) {
    unsafe {
        let mut ps = Paint::default();
        let dc = BeginPaint(hwnd, &mut ps);
        if dc == 0 {
            EndPaint(hwnd, &ps);
            return;
        }
        let mut client = Rect::default();
        GetClientRect(hwnd, &mut client);
        if client.right > 0 && client.bottom > 0 {
            if let Some(buffer) = app.back_buffer.prepare(dc, client.right, client.bottom) {
                render_frame(buffer, app, client);
                if !app.back_buffer.present(dc, ps.rect) {
                    render_frame(dc, app, client);
                }
            } else {
                // Allocation failure must still draw the document, rather than
                // copying an empty buffer or leaving the window unpainted.
                render_frame(dc, app, client);
            }
        }
        EndPaint(hwnd, &ps);
    }
}

// Compose the entire frame offscreen; no visible clearing between drawing steps.
// Save/restore inside this function keeps the reusable DC's clip and font stable.
unsafe fn render_frame(dc: Handle, app: &App, client: Rect) {
    unsafe {
        let toolbar = appearance::Layout::new(client.right, app.toolbar_collapsed);
        let palette = appearance::Palette::new(app.theme.dark(app.system_dark));
        block(dc, client, palette.workspace, false);
        block(
            dc,
            Rect {
                left: 0,
                top: 0,
                right: client.right,
                bottom: toolbar.height,
            },
            palette.surface,
            false,
        );
        let font = appearance::font();
        let old_font = SelectObject(dc, if font != 0 { font } else { GetStockObject(17) });
        SetBkMode(dc, 1);
        appearance::toolbar(dc, app, &toolbar);
        let w = app.canvas.width() as i32;
        let h = app.canvas.height() as i32;
        #[repr(C, align(4))]
        struct AlignedHeader([u8; 40]);
        let header = AlignedHeader(app.canvas.dib_header());
        let saved = SaveDC(dc);
        IntersectClipRect(dc, 0, toolbar.height + 8, client.right, client.bottom - 28);
        block(
            dc,
            Rect {
                left: app.origin.0 - app.pan.0 + 3,
                top: app.origin.1 - app.pan.1 + 3,
                right: app.origin.0 - app.pan.0 + app.view.0 + 4,
                bottom: app.origin.1 - app.pan.1 + app.view.1 + 4,
            },
            palette.line,
            false,
        );
        StretchDIBits(
            dc,
            app.origin.0 - app.pan.0,
            app.origin.1 - app.pan.1,
            app.view.0,
            app.view.1,
            0,
            0,
            w,
            h,
            app.canvas.pixels().as_ptr().cast(),
            header.0.as_ptr().cast(),
            0,
            0x00cc0020,
        );
        block(
            dc,
            Rect {
                left: app.origin.0 - app.pan.0 - 1,
                top: app.origin.1 - app.pan.1 - 1,
                right: app.origin.0 - app.pan.0 + app.view.0 + 1,
                bottom: app.origin.1 - app.pan.1 + app.view.1 + 1,
            },
            0x9ba4ad,
            true,
        );
        appearance::pixel_grid(dc, app, client);
        if let Some((a, b)) = app.selection.filter(|_| app.moving.is_none()) {
            let point = |(x, y): (i32, i32)| {
                (
                    app.origin.0 - app.pan.0 + (x as f64 * app.view.0 as f64 / w as f64) as i32,
                    app.origin.1 - app.pan.1 + (y as f64 * app.view.1 as f64 / h as f64) as i32,
                )
            };
            let a = point(a);
            let b = point(b);
            DrawFocusRect(
                dc,
                &Rect {
                    left: a.0.min(b.0),
                    top: a.1.min(b.1),
                    right: a.0.max(b.0) + 1,
                    bottom: a.1.max(b.1) + 1,
                },
            );
        }
        if saved != 0 {
            RestoreDC(dc, saved);
        }
        let status = Rect {
            left: 0,
            top: client.bottom - 26,
            right: client.right,
            bottom: client.bottom,
        };
        block(dc, status, palette.surface, false);
        appearance::footer(dc, app, client.right, client.bottom);
        SelectObject(dc, old_font);
        if font != 0 {
            DeleteObject(font);
        }
    }
}
fn down(app: &mut App, x: i32, y: i32) -> Action {
    let toolbar = appearance::Layout::new(app.client.0, app.toolbar_collapsed);
    if let Some(id) = toolbar.hit(x, y) {
        match id {
            100..=109 => {
                app.tool = TOOLS[(id - 100) as usize].0;
            }
            80 => app.size = (app.size - 1).max(1),
            81 => app.size = (app.size + 1).min(32),
            1 => return Action::New,
            2 => return Action::Open,
            3 => return Action::Save(false),
            6 => {
                app.canvas.undo();
                app.selection = None;
                app.dirty = true;
            }
            7 => {
                app.canvas.redo();
                app.selection = None;
                app.dirty = true;
            }
            _ => return Action::Feature(id),
        }
        return Action::None;
    }
    if let Some(color) = toolbar.color_at(x, y) {
        app.color = color;
        return Action::None;
    }
    if let Some(id) = appearance::footer_hit(app.client.0, app.client.1, x, y) {
        return Action::Feature(id);
    }
    let slider = appearance::slider(app.client.0, app.client.1);
    if appearance::contains(slider, x, y) {
        let zoom = appearance::slider_zoom(slider, x);
        let center = viewport_center(app);
        zoom_about(app, zoom, center);
        app.zoom_drag = true;
        return Action::None;
    }
    if y < toolbar.height + 8 || y >= app.client.1 - 28 {
        return Action::None;
    }
    let p = image_point(app, x, y);
    if app.canvas.get(p.0, p.1).is_none() {
        return Action::None;
    }
    if app.tool == Tool::Picker {
        app.color = app.canvas.get(p.0, p.1).unwrap_or(app.color);
        return Action::None;
    }
    if app.tool == Tool::Text {
        return Action::Text(p);
    }
    if app.tool == Tool::Select {
        if let Some((a, b)) = app.selection {
            if p.0 >= a.0.min(b.0)
                && p.0 <= a.0.max(b.0)
                && p.1 >= a.1.min(b.1)
                && p.1 <= a.1.max(b.1)
            {
                app.canvas.checkpoint();
                app.moving = Some((a, b));
                app.start = p;
                app.drawing = true;
                return Action::None;
            }
        }
        app.moving = None;
        app.selection = Some((p, p));
        app.start = p;
        app.last = p;
        app.drawing = true;
        return Action::None;
    }
    app.selection = None;
    app.stroke_color = app.color;
    app.canvas.checkpoint();
    app.start = p;
    app.last = p;
    app.dirty = true;
    app.canvas.draw(app.tool, p, p, app.color, app.size);
    app.drawing = app.tool != Tool::Fill;
    Action::None
}
fn movement(app: &mut App, x: i32, y: i32) {
    if !app.drawing {
        return;
    }
    let mut p = image_point(app, x, y);
    if app.constrain {
        let (dx, dy) = (p.0 - app.start.0, p.1 - app.start.1);
        if matches!(app.tool, Tool::Rectangle | Tool::Ellipse) {
            let n = dx.abs().max(dy.abs());
            p = (
                app.start.0 + n * if dx < 0 { -1 } else { 1 },
                app.start.1 + n * if dy < 0 { -1 } else { 1 },
            );
        } else if app.tool == Tool::Line {
            if (dy.abs() as f64) < (dx.abs() as f64 * 0.414) {
                p.1 = app.start.1;
            } else if (dx.abs() as f64) < (dy.abs() as f64 * 0.414) {
                p.0 = app.start.0;
            } else {
                let n = dx.abs().max(dy.abs());
                p = (app.start.0 + n * dx.signum(), app.start.1 + n * dy.signum());
            }
        }
    }
    if app.tool == Tool::Select {
        if let Some((a, b)) = app.moving {
            if let Ok(bounds) =
                app.canvas
                    .move_selection_preview(a, b, p.0 - app.start.0, p.1 - app.start.1)
            {
                app.selection = Some(bounds);
                app.dirty = true;
            }
            return;
        }
        let p = (
            p.0.clamp(0, app.canvas.width() as i32 - 1),
            p.1.clamp(0, app.canvas.height() as i32 - 1),
        );
        app.selection = Some((app.start, p));
        app.last = p;
        return;
    }
    let start = if matches!(app.tool, Tool::Pencil | Tool::Brush | Tool::Eraser) {
        app.last
    } else {
        app.canvas.restore_preview();
        app.start
    };
    app.canvas
        .draw(app.tool, start, p, app.stroke_color, app.size);
    app.last = p;
}
unsafe fn title(hwnd: Handle, cell: &RefCell<App>) {
    unsafe {
        let app = cell.borrow();
        let name = app
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Untitled".into());
        let text = wide(&format!(
            "{}{} — Paint (Rust)",
            if app.dirty { "*" } else { "" },
            name
        ));
        drop(app);
        SetWindowTextW(hwnd, text.as_ptr());
    }
}
unsafe fn error(hwnd: Handle, text: &str) {
    unsafe {
        MessageBoxW(hwnd, wide(text).as_ptr(), wide("Paint").as_ptr(), 0x10);
    }
}
unsafe fn file_path(hwnd: Handle, save: bool) -> Result<Option<PathBuf>, String> {
    unsafe {
        use std::os::windows::ffi::OsStringExt;
        let mut file = vec![0u16; 32768];
        let filter = wide("PNG image (*.png)\0*.png\0JPEG image (*.jpg;*.jpeg)\0*.jpg;*.jpeg\0Bitmap (*.bmp)\0*.bmp\0GIF image (*.gif)\0*.gif\0TIFF image (*.tif;*.tiff)\0*.tif;*.tiff\0All images\0*.png;*.jpg;*.jpeg;*.bmp;*.gif;*.tif;*.tiff\0\0");
        // All-zero pointer/function fields denote absent optional features.
        let mut dialog: FileDialog = std::mem::zeroed();
        dialog.size = std::mem::size_of::<FileDialog>() as u32;
        dialog.owner = hwnd;
        dialog.filter = filter.as_ptr();
        dialog.file = file.as_mut_ptr();
        dialog.max_file = file.len() as u32;
        dialog.default_extension = std::ptr::null();
        dialog.index = if save { 1 } else { 6 };
        dialog.flags = 0x00080000 | 0x00000008 | 0x00000800 | if save { 0x2 } else { 0x1000 };
        let success = if save {
            GetSaveFileNameW(&mut dialog)
        } else {
            GetOpenFileNameW(&mut dialog)
        };
        if success == 0 {
            let code = CommDlgExtendedError();
            return if code == 0 {
                Ok(None)
            } else {
                Err(format!("File dialog failed: 0x{code:x}"))
            };
        }
        let end = file.iter().position(|c| *c == 0).unwrap_or(file.len());
        let mut path: PathBuf = std::ffi::OsString::from_wide(&file[..end]).into();
        if save && path.extension().is_none() {
            let extension = ["png", "jpg", "bmp", "gif", "tif"]
                .get(dialog.index.saturating_sub(1) as usize)
                .copied()
                .unwrap_or("png");
            path.set_extension(extension);
            if path.exists()
                && MessageBoxW(
                    hwnd,
                    wide("This file already exists. Replace it?").as_ptr(),
                    wide("Save picture").as_ptr(),
                    4 | 0x20,
                ) != 6
            {
                return Ok(None);
            }
        }
        Ok(Some(path))
    }
}
unsafe fn save(hwnd: Handle, cell: &RefCell<App>, save_as: bool) -> Result<bool, String> {
    unsafe {
        let existing = if save_as {
            None
        } else {
            cell.borrow().path.clone()
        };
        let path = match existing {
            Some(path) => path,
            None => match file_path(hwnd, true)? {
                Some(path) => path,
                None => return Ok(false),
            },
        };
        // Create/truncate only after a selected destination and overwrite confirmation
        // from the standard file dialog, or an explicit Save to the current document.
        crate::image_codec::save(&path, &cell.borrow().canvas)?;
        let mut app = cell.borrow_mut();
        app.path = Some(path);
        app.dirty = false;
        drop(app);
        title(hwnd, cell);
        Ok(true)
    }
}
unsafe fn may_discard(hwnd: Handle, cell: &RefCell<App>) -> bool {
    unsafe {
        if !cell.borrow().dirty {
            return true;
        }
        match MessageBoxW(
            hwnd,
            wide("Save changes to this picture?").as_ptr(),
            wide("Paint").as_ptr(),
            3 | 0x20,
        ) {
            6 => match save(hwnd, cell, false) {
                Ok(saved) => saved,
                Err(e) => {
                    error(hwnd, &e);
                    false
                }
            },
            7 => true,
            _ => false,
        }
    }
}
unsafe fn action(hwnd: Handle, cell: &RefCell<App>, operation: Action) {
    unsafe {
        match operation {
            Action::OpenPath(path) => {
                if may_discard(hwnd, cell) {
                    match crate::image_codec::load(&path) {
                        Ok(canvas) => {
                            let mut app = cell.borrow_mut();
                            app.canvas = canvas;
                            app.path = Some(path);
                            app.dirty = false;
                            app.selection = None;
                            app.pan = (0, 0);
                        }
                        Err(e) => error(hwnd, &e),
                    }
                }
            }
            Action::Feature(id) => {
                if let Err(e) = features::perform(hwnd, cell, id) {
                    error(hwnd, &e);
                }
            }
            Action::Text(point) => {
                if let Err(e) = features::text(hwnd, cell, point) {
                    error(hwnd, &e);
                }
            }
            Action::None => {}
            Action::Save(save_as) => {
                if let Err(e) = save(hwnd, cell, save_as) {
                    error(hwnd, &e);
                }
            }
            Action::Close => {
                if may_discard(hwnd, cell) {
                    DestroyWindow(hwnd);
                }
                return;
            }
            Action::New => {
                if may_discard(hwnd, cell) {
                    match Canvas::new(800, 600) {
                        Ok(canvas) => {
                            let mut app = cell.borrow_mut();
                            app.canvas = canvas;
                            app.path = None;
                            app.dirty = false;
                            app.selection = None;
                            app.pan = (0, 0);
                        }
                        Err(e) => error(hwnd, &e),
                    }
                }
            }
            Action::Open => {
                if may_discard(hwnd, cell) {
                    match file_path(hwnd, false) {
                        Ok(Some(path)) => {
                            let result = crate::image_codec::load(&path);
                            match result {
                                Ok(canvas) => {
                                    let mut app = cell.borrow_mut();
                                    app.canvas = canvas;
                                    app.path = Some(path);
                                    app.dirty = false;
                                    app.selection = None;
                                    app.pan = (0, 0);
                                }
                                Err(e) => error(hwnd, &e),
                            }
                        }
                        Ok(None) => {}
                        Err(e) => error(hwnd, &e),
                    }
                }
            }
        }
        let mut client = Rect::default();
        GetClientRect(hwnd, &mut client);
        layout(&mut cell.borrow_mut(), client.right, client.bottom);
        appearance::apply_theme(hwnd, cell);
        title(hwnd, cell);
        InvalidateRect(hwnd, std::ptr::null(), 0);
    }
}
// SAFETY: USER32 invokes this callback on the creating UI thread. The boxed
// RefCell lives until the message loop finishes. No mutable borrow survives
// a modal dialog or other call that can reenter this procedure.
unsafe extern "system" fn procedure(
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
        if message == 0x24 {
            let limits = &mut *(lparam as *mut MinMax);
            limits.min_track = Point { x: 620, y: 500 };
            return 0;
        }
        let pointer = GetWindowLongPtrW(hwnd, -21) as *const RefCell<App>;
        if pointer.is_null() {
            return DefWindowProcW(hwnd, message, wparam, lparam);
        }
        let cell = &*pointer;
        match message {
            0x20 if lparam as u16 == 1 => {
                let mut point = Point::default();
                GetCursorPos(&mut point);
                ScreenToClient(hwnd, &mut point);
                let cursor = {
                    let app = cell.borrow();
                    let p = image_point(&app, point.x, point.y);
                    let toolbar = appearance::Layout::new(app.client.0, app.toolbar_collapsed);
                    if point.y >= toolbar.height + 8
                        && point.y < app.client.1 - 28
                        && app.canvas.get(p.0, p.1).is_some()
                    {
                        if app.tool == Tool::Text {
                            32513
                        } else if app.tool == Tool::Select
                            && app.selection.is_some_and(|(a, b)| {
                                p.0 >= a.0.min(b.0)
                                    && p.0 <= a.0.max(b.0)
                                    && p.1 >= a.1.min(b.1)
                                    && p.1 <= a.1.max(b.1)
                            })
                        {
                            32646
                        } else {
                            32515
                        }
                    } else {
                        32512
                    }
                };
                SetCursor(LoadCursorW(0, cursor as *const u16));
                1
            }
            0x1a => {
                cell.borrow_mut().system_dark = appearance::system_dark();
                appearance::apply_theme(hwnd, cell);
                InvalidateRect(hwnd, std::ptr::null(), 0);
                0
            }
            0x2a3 => {
                cell.borrow_mut().hover = None;
                InvalidateRect(hwnd, std::ptr::null(), 0);
                0
            }
            0x233 => {
                use std::os::windows::ffi::OsStringExt;
                let count = DragQueryFileW(wparam as isize, 0, std::ptr::null_mut(), 0);
                let mut path = vec![0u16; count as usize + 1];
                DragQueryFileW(wparam as isize, 0, path.as_mut_ptr(), path.len() as u32);
                DragFinish(wparam as isize);
                action(
                    hwnd,
                    cell,
                    Action::OpenPath(std::ffi::OsString::from_wide(&path[..count as usize]).into()),
                );
                0
            }
            0x20a => {
                let mut anchor = Point {
                    x: lparam as i16 as i32,
                    y: (lparam >> 16) as i16 as i32,
                };
                ScreenToClient(hwnd, &mut anchor);
                let delta = (wparam >> 16) as i16 as i32;
                let mut app = cell.borrow_mut();
                if GetKeyState(0x11) < 0 {
                    let current = if app.zoom == 0 {
                        app.view.0 as u32 * 100 / app.canvas.width()
                    } else {
                        app.zoom
                    };
                    let steps = [25, 50, 100, 200, 400, 800];
                    let zoom = if delta > 0 {
                        steps.into_iter().find(|s| *s > current).unwrap_or(800)
                    } else {
                        steps.into_iter().rev().find(|s| *s < current).unwrap_or(25)
                    };
                    zoom_about(&mut app, zoom, (anchor.x, anchor.y));
                } else if GetKeyState(0x10) < 0 {
                    app.pan.0 = (app.pan.0 - delta / 2).max(0);
                } else {
                    app.pan.1 = (app.pan.1 - delta / 2).max(0);
                }
                drop(app);
                action(hwnd, cell, Action::None);
                0
            }
            0x5 => {
                let mut client = Rect::default();
                GetClientRect(hwnd, &mut client);
                layout(&mut cell.borrow_mut(), client.right, client.bottom);
                InvalidateRect(hwnd, std::ptr::null(), 0);
                0
            }
            0xf => {
                paint(hwnd, &mut cell.borrow_mut());
                0
            }
            // WM_ERASEBKGND: the completed frame includes the background.
            0x14 => 1,
            0x201 | 0x204 => {
                let (operation, drawing) = {
                    let mut app = cell.borrow_mut();
                    let primary = app.color;
                    if message == 0x204 {
                        app.color = app.secondary;
                    }
                    let a = down(&mut app, lparam as i16 as i32, (lparam >> 16) as i16 as i32);
                    if message == 0x204 {
                        app.secondary = app.color;
                        app.color = primary;
                    }
                    (a, app.drawing || app.zoom_drag)
                };
                if drawing {
                    SetCapture(hwnd);
                }
                action(hwnd, cell, operation);
                0
            }
            0x200 => {
                let mut tracking = MouseTracking {
                    size: std::mem::size_of::<MouseTracking>() as u32,
                    flags: 2,
                    hwnd,
                    hover: 0,
                };
                TrackMouseEvent(&mut tracking);
                let (x, y) = (lparam as i16 as i32, (lparam >> 16) as i16 as i32);
                let repaint = {
                    let mut app = cell.borrow_mut();
                    let toolbar = appearance::Layout::new(app.client.0, app.toolbar_collapsed);
                    let hover = toolbar
                        .hit(x, y)
                        .or_else(|| appearance::footer_hit(app.client.0, app.client.1, x, y));
                    let changed = app.hover != hover;
                    app.hover = hover;
                    app.constrain = GetKeyState(0x10) < 0;
                    if app.zoom_drag {
                        let zoom = appearance::slider_zoom(
                            appearance::slider(app.client.0, app.client.1),
                            x,
                        );
                        let center = viewport_center(&app);
                        zoom_about(&mut app, zoom, center);
                    } else {
                        movement(&mut app, x, y);
                    }
                    changed || app.drawing || app.zoom_drag
                };
                if repaint {
                    InvalidateRect(hwnd, std::ptr::null(), 0);
                }
                0
            }
            0x202 | 0x205 => {
                {
                    let mut app = cell.borrow_mut();
                    app.constrain = GetKeyState(0x10) < 0;
                    movement(&mut app, lparam as i16 as i32, (lparam >> 16) as i16 as i32);
                    app.drawing = false;
                    app.moving = None;
                    app.zoom_drag = false;
                }
                ReleaseCapture();
                InvalidateRect(hwnd, std::ptr::null(), 0);
                0
            }
            0x215 => {
                cell.borrow_mut().drawing = false;
                cell.borrow_mut().moving = None;
                cell.borrow_mut().zoom_drag = false;
                0
            }
            0x111 => {
                let operation = match wparam & 0xffff {
                    1 => Action::New,
                    2 => Action::Open,
                    3 => Action::Save(false),
                    4 => Action::Save(true),
                    5 => Action::Close,
                    6 => {
                        let mut app = cell.borrow_mut();
                        app.canvas.undo();
                        app.dirty = true;
                        Action::None
                    }
                    7 => {
                        let mut app = cell.borrow_mut();
                        app.canvas.redo();
                        app.dirty = true;
                        Action::None
                    }
                    8 => {
                        let mut app = cell.borrow_mut();
                        app.canvas.clear();
                        app.dirty = true;
                        Action::None
                    }
                    id if (10..=62).contains(&id) => Action::Feature(id as u16),
                    _ => Action::None,
                };
                action(hwnd, cell, operation);
                0
            }
            0x100 if GetKeyState(0x11) < 0 => {
                let operation = match wparam {
                    0x4e => Action::New,
                    0x4f => Action::Open,
                    0x53 => Action::Save(GetKeyState(0x10) < 0),
                    0x5a => {
                        let mut app = cell.borrow_mut();
                        app.canvas.undo();
                        app.dirty = true;
                        Action::None
                    }
                    0x59 => {
                        let mut app = cell.borrow_mut();
                        app.canvas.redo();
                        app.dirty = true;
                        Action::None
                    }
                    0x43 => Action::Feature(10),
                    0x58 => Action::Feature(if GetKeyState(0x10) < 0 { 14 } else { 11 }),
                    0x56 => Action::Feature(12),
                    0x41 => Action::Feature(13),
                    0x45 => Action::Feature(16),
                    0x57 => Action::Feature(15),
                    0x50 => Action::Feature(40),
                    0x47 => Action::Feature(58),
                    _ => Action::None,
                };
                action(hwnd, cell, operation);
                0
            }
            0x100 if wparam == 0x2e => {
                action(hwnd, cell, Action::Feature(22));
                0
            }
            0x10 => {
                action(hwnd, cell, Action::Close);
                0
            }
            0x2 => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, message, wparam, lparam),
        }
    }
}

pub fn run(initial: Option<PathBuf>, smoke: bool) -> Result<(), String> {
    let canvas = if let Some(path) = initial.as_ref() {
        crate::image_codec::load(path)?
    } else {
        Canvas::new(800, 600)?
    };
    let state = Box::new(RefCell::new(App {
        back_buffer: rendering::BackBuffer::default(),
        canvas,
        color: 0,
        size: 2,
        tool: Tool::Pencil,
        drawing: false,
        start: (0, 0),
        last: (0, 0),
        dirty: false,
        path: initial,
        view: (800, 600),
        selection: None,
        zoom: 0,
        pan: (0, 0),
        moving: None,
        secondary: 0xffffff,
        stroke_color: 0,
        client: (940, 880),
        origin: (20, 146),
        theme: appearance::Theme::System,
        system_dark: false,
        toolbar_collapsed: false,
        hover: None,
        grid: false,
        zoom_drag: false,
        constrain: false,
    }));
    state.borrow_mut().system_dark = appearance::system_dark();
    // SAFETY: pointers reference live boxed state and UTF-16 buffers until the
    // window has been destroyed; message processing remains on this thread.
    unsafe {
        let instance = GetModuleHandleW(std::ptr::null());
        let class = wide("MSPaintRustEditor");
        let wc = WindowClass {
            style: 3,
            procedure: Some(procedure),
            class_extra: 0,
            window_extra: 0,
            instance,
            icon: LoadIconW(0, 32512usize as *const u16),
            cursor: LoadCursorW(0, 32515usize as *const u16),
            background: 0,
            menu: std::ptr::null(),
            name: class.as_ptr(),
        };
        if RegisterClassW(&wc) == 0 {
            return Err(format!(
                "RegisterClassW: {}",
                std::io::Error::last_os_error()
            ));
        }
        let hwnd = CreateWindowExW(
            0,
            class.as_ptr(),
            wide("Untitled — Paint (Rust)").as_ptr(),
            0x00cf0000,
            100,
            80,
            940,
            880,
            0,
            0,
            instance,
            (&*state as *const RefCell<App>).cast_mut().cast(),
        );
        if hwnd == 0 {
            return Err(format!(
                "CreateWindowExW: {}",
                std::io::Error::last_os_error()
            ));
        }
        let menu = CreateMenu();
        let file = CreatePopupMenu();
        let edit = CreatePopupMenu();
        let image = CreatePopupMenu();
        let view = CreatePopupMenu();
        for (id, name) in [
            (1, "&New\tCtrl+N"),
            (2, "&Open...\tCtrl+O"),
            (3, "&Save\tCtrl+S"),
            (4, "Save &as...\tCtrl+Shift+S"),
            (40, "&Print...\tCtrl+P"),
            (5, "E&xit"),
        ] {
            AppendMenuW(file, 0, id, wide(name).as_ptr());
        }
        for (id, name) in [
            (6, "&Undo\tCtrl+Z"),
            (7, "&Redo\tCtrl+Y"),
            (8, "Clear image"),
            (10, "&Copy\tCtrl+C"),
            (11, "Cu&t\tCtrl+X"),
            (12, "&Paste\tCtrl+V"),
            (13, "Select &all\tCtrl+A"),
            (22, "Delete selection\tDel"),
        ] {
            AppendMenuW(edit, 0, id, wide(name).as_ptr());
        }
        AppendMenuW(menu, 0x10, file as usize, wide("&File").as_ptr());
        AppendMenuW(menu, 0x10, edit as usize, wide("&Edit").as_ptr());
        for (id, name) in [
            (14, "&Crop to selection"),
            (15, "&Resize...\tCtrl+W"),
            (16, "Canvas &size...\tCtrl+E"),
            (17, "Rotate &right 90°"),
            (18, "Rotate &left 90°"),
            (19, "Rotate &180°"),
            (20, "Flip &horizontal"),
            (21, "Flip &vertical"),
            (24, "&Invert colors"),
            (23, "Edit &colors..."),
        ] {
            AppendMenuW(image, 0, id, wide(name).as_ptr());
        }
        for (id, name) in [
            (30, "Fit to &window"),
            (31, "&25%"),
            (32, "&50%"),
            (33, "&100%"),
            (34, "&200%"),
            (35, "&400%"),
            (36, "&800%"),
            (57, "Show/hide toolbar"),
            (58, "Pixel grid (400%+)\tCtrl+G"),
            (51, "Light theme"),
            (52, "Dark theme"),
            (53, "Use Windows theme"),
        ] {
            AppendMenuW(view, 0, id, wide(name).as_ptr());
        }
        AppendMenuW(menu, 0x10, image as usize, wide("&Image").as_ptr());
        AppendMenuW(menu, 0x10, view as usize, wide("&View").as_ptr());
        if SetMenu(hwnd, menu) == 0 {
            DestroyMenu(menu);
            DestroyWindow(hwnd);
            return Err("could not attach application menu".into());
        }
        ShowWindow(hwnd, 5);
        appearance::apply_theme(hwnd, &state);
        DragAcceptFiles(hwnd, 1);
        UpdateWindow(hwnd);
        title(hwnd, &state);
        if smoke {
            // Exercise both themes, compact toolbar and high-zoom grid through
            // real WM_PAINT handling, without touching user documents.
            for theme in [appearance::Theme::Light, appearance::Theme::Dark] {
                {
                    let mut app = state.borrow_mut();
                    app.theme = theme;
                    app.toolbar_collapsed = theme == appearance::Theme::Dark;
                    app.grid = true;
                    let (w, h) = app.client;
                    app.zoom = 400;
                    layout(&mut app, w, h);
                }
                appearance::apply_theme(hwnd, &state);
                InvalidateRect(hwnd, std::ptr::null(), 0);
                UpdateWindow(hwnd);
            }
            let mut rect = Rect::default();
            let valid = IsWindowVisible(hwnd) != 0
                && GetClientRect(hwnd, &mut rect) != 0
                && rect.right > 0
                && rect.bottom > 0;
            DestroyWindow(hwnd);
            if !valid {
                return Err("desktop window failed its creation/render check".into());
            }
            return Ok(());
        }
        let mut message = Message::default();
        loop {
            let result = GetMessageW(&mut message, 0, 0, 0);
            if result == 0 {
                break;
            }
            if result < 0 {
                DestroyWindow(hwnd);
                return Err(format!("GetMessageW: {}", std::io::Error::last_os_error()));
            }
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
    Ok(())
}
pub fn show_error(message: &str) {
    unsafe {
        error(0, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> App {
        App {
            back_buffer: rendering::BackBuffer::default(),
            canvas: Canvas::new(800, 600).unwrap(),
            color: 0,
            size: 2,
            tool: Tool::Pencil,
            drawing: false,
            start: (0, 0),
            last: (0, 0),
            dirty: false,
            path: None,
            view: (800, 600),
            selection: None,
            zoom: 0,
            pan: (0, 0),
            moving: None,
            secondary: 0xffffff,
            stroke_color: 0,
            client: (940, 880),
            origin: (20, 146),
            theme: appearance::Theme::System,
            system_dark: false,
            toolbar_collapsed: false,
            hover: None,
            grid: false,
            zoom_drag: false,
            constrain: false,
        }
    }
    #[test]
    fn native_window_creates_paints_and_closes() {
        run(None, true).unwrap();
    }

    #[link(name = "user32")]
    unsafe extern "system" {
        fn GetDC(hwnd: Handle) -> Handle;
        fn ReleaseDC(hwnd: Handle, dc: Handle) -> i32;
    }
    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn GetPixel(dc: Handle, x: i32, y: i32) -> u32;
    }
    struct DisplayDc(Handle);
    impl DisplayDc {
        fn new() -> Self {
            let dc = unsafe { GetDC(0) };
            assert_ne!(dc, 0);
            Self(dc)
        }
    }
    impl Drop for DisplayDc {
        fn drop(&mut self) {
            unsafe { ReleaseDC(0, self.0) };
        }
    }

    #[test]
    fn buffered_drawing_publishes_complete_color_frames_without_clearing_destination() {
        let display = DisplayDc::new();
        let mut app = app();
        app.zoom = 100;
        app.color = 0xe04020;
        layout(&mut app, 940, 880);
        let client = Rect {
            left: 0,
            top: 0,
            right: 940,
            bottom: 880,
        };
        let mut destination = rendering::BackBuffer::default();
        // All reads/writes below target owned offscreen bitmaps, not the desktop.
        unsafe {
            let target = destination.prepare(display.0, 940, 880).unwrap();
            block(target, client, 0xff00ff, false);
            let origin = app.origin;
            down(&mut app, origin.0 + 20, origin.1 + 20);
            for step in 0..48 {
                app.theme = if step % 2 == 0 {
                    appearance::Theme::Light
                } else {
                    appearance::Theme::Dark
                };
                let point = (origin.0 + 20 + step * 3, origin.1 + 20 + step);
                movement(&mut app, point.0, point.1);
                let dc = app.back_buffer.prepare(display.0, 940, 880).unwrap();
                let previous = GetPixel(target, point.0, point.1);
                render_frame(dc, &app, client);
                // Composition cannot clear the destination while it is running.
                assert_eq!(GetPixel(target, point.0, point.1), previous);
                assert_eq!(GetPixel(dc, point.0, point.1), native_color(app.color));
                assert_eq!(GetPixel(dc, origin.0 + 500, origin.1 + 400), 0xffffff);
                assert_eq!(
                    GetPixel(dc, 2, origin.1),
                    native_color(appearance::Palette::new(app.theme.dark(false)).workspace)
                );
                assert!(app.back_buffer.present(target, client));
                assert_eq!(GetPixel(target, point.0, point.1), native_color(app.color));
                assert_eq!(GetPixel(target, origin.0 + 500, origin.1 + 400), 0xffffff);
            }
        }
        assert!(app.canvas.undo());
        assert!(!app.canvas.undo());
    }

    #[test]
    fn back_buffer_reuses_storage_resizes_and_only_copies_the_damaged_region() {
        let display = DisplayDc::new();
        let mut source = rendering::BackBuffer::default();
        let mut destination = rendering::BackBuffer::default();
        unsafe {
            let dc = source.prepare(display.0, 80, 60).unwrap();
            assert_eq!(source.prepare(display.0, 80, 60), Some(dc));
            let target = destination.prepare(display.0, 80, 60).unwrap();
            let rect = Rect {
                left: 0,
                top: 0,
                right: 80,
                bottom: 60,
            };
            block(dc, rect, 0x12ab34, false);
            block(target, rect, 0xff00ff, false);
            assert!(source.present(
                target,
                Rect {
                    left: 10,
                    top: 15,
                    right: 40,
                    bottom: 45
                }
            ));
            assert_eq!(GetPixel(target, 10, 15), native_color(0x12ab34));
            assert_eq!(GetPixel(target, 39, 44), native_color(0x12ab34));
            assert_eq!(GetPixel(target, 9, 15), native_color(0xff00ff));
            assert_eq!(GetPixel(target, 40, 45), native_color(0xff00ff));
            assert!(source.prepare(display.0, 0, 0).is_none());
            assert_eq!(source.prepare(display.0, 80, 60), Some(dc));
            let resized = source.prepare(display.0, 120, 90).unwrap();
            let larger = Rect {
                right: 120,
                bottom: 90,
                ..rect
            };
            block(resized, larger, 0x56cd78, false);
            assert_eq!(GetPixel(resized, 119, 89), native_color(0x56cd78));
        }
    }
    #[test]
    fn canvas_centers_and_collapsing_toolbar_reclaims_workspace() {
        let mut app = app();
        app.zoom = 100;
        layout(&mut app, 1280, 1000);
        let top = appearance::Layout::new(1280, false).height + 20;
        assert_eq!(app.origin.0, 240);
        assert_eq!(
            app.origin.1 - top,
            (1000 - 40) - (app.origin.1 + app.view.1)
        );
        let old_top = app.origin.1;
        app.toolbar_collapsed = true;
        layout(&mut app, 1280, 1000);
        assert!(app.origin.1 < old_top);
        assert_eq!(image_point(&app, app.origin.0, app.origin.1), (0, 0));
    }
    #[test]
    fn zoom_keeps_the_cursor_anchor_over_the_same_image_pixel() {
        let mut app = app();
        layout(&mut app, 940, 880);
        let anchor = (app.client.0 / 2, app.origin.1 + app.view.1 / 2);
        let before = image_point(&app, anchor.0, anchor.1);
        zoom_about(&mut app, 400, anchor);
        let after = image_point(&app, anchor.0, anchor.1);
        assert!(before.0.abs_diff(after.0) <= 1 && before.1.abs_diff(after.1) <= 1);
    }
    #[test]
    fn shift_line_constraint_snaps_to_horizontal() {
        let mut app = app();
        app.zoom = 100;
        app.toolbar_collapsed = true;
        layout(&mut app, 940, 1000);
        let origin = app.origin;
        app.tool = Tool::Line;
        app.constrain = true;
        down(&mut app, origin.0 + 20, origin.1 + 20);
        movement(&mut app, origin.0 + 120, origin.1 + 23);
        assert_eq!(app.canvas.get(120, 20), Some(0));
        assert_eq!(app.canvas.get(120, 23), Some(0xffffff));
    }
    #[test]
    #[cfg(target_pointer_width = "64")]
    fn ffi_records_match_the_windows_x64_abi() {
        assert_eq!(std::mem::size_of::<WindowClass>(), 72);
        assert_eq!(std::mem::size_of::<Message>(), 48);
        assert_eq!(std::mem::size_of::<Paint>(), 72);
        assert_eq!(std::mem::size_of::<Create>(), 80);
        assert_eq!(std::mem::size_of::<FileDialog>(), 152);
    }
    #[test]
    fn toolbar_changes_tools_and_colors_without_editing_the_document() {
        let mut app = app();
        let toolbar = appearance::Layout::new(app.client.0, false);
        let r = toolbar.buttons.iter().find(|b| b.id == 101).unwrap().rect;
        down(&mut app, r.left + 2, r.top + 2);
        assert_eq!(app.tool, Tool::Eraser);
        let r = toolbar.colors[3].1;
        down(&mut app, r.left + 2, r.top + 2);
        assert_eq!(app.color, 0xff0000);
        assert!(!app.dirty);
        let r = toolbar.buttons.iter().find(|b| b.id == 81).unwrap().rect;
        down(&mut app, r.left + 2, r.top + 2);
        assert_eq!(app.size, 3);
    }
    #[test]
    fn scaled_canvas_input_and_shape_preview_keep_one_undo_record() {
        let mut app = app();
        layout(&mut app, 940, 880);
        let origin = app.origin;
        let scale = app.view.0 as f64 / app.canvas.width() as f64;
        let delta = (200.0 * scale).round() as i32;
        let target = image_point(&app, origin.0 + delta, origin.1 + delta);
        assert!(target.0.abs_diff(200) <= 1 && target.1.abs_diff(200) <= 1);
        app.tool = Tool::Line;
        down(&mut app, origin.0, origin.1);
        movement(&mut app, origin.0 + delta, origin.1 + delta);
        assert_eq!(app.canvas.get(target.0, target.1), Some(0));
        movement(&mut app, origin.0, origin.1 + delta);
        assert_eq!(app.canvas.get(target.0, target.1), Some(0xffffff));
        assert_eq!(app.canvas.get(0, target.1), Some(0));
        assert!(app.canvas.undo());
        assert!(!app.canvas.undo());
    }
}
