//! Shared toolbar geometry, theme and chrome. Rendering and input use one layout.
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Theme {
    System,
    Light,
    Dark,
}
impl Theme {
    pub fn dark(self, system: bool) -> bool {
        match self {
            Self::System => system,
            Self::Light => false,
            Self::Dark => true,
        }
    }
}
#[derive(Clone, Copy)]
pub(super) struct Palette {
    pub workspace: u32,
    pub surface: u32,
    pub text: u32,
    pub muted: u32,
    pub line: u32,
    pub hover: u32,
    pub selected: u32,
    pub accent: u32,
}
impl Palette {
    pub fn new(dark: bool) -> Self {
        if dark {
            Self {
                workspace: 0x202020,
                surface: 0x2b2b2b,
                text: 0xf4f4f4,
                muted: 0xcccccc,
                line: 0x474747,
                hover: 0x3c3c3c,
                selected: 0x173e55,
                accent: 0x60cdff,
            }
        } else {
            Self {
                workspace: 0xe9e9e9,
                surface: 0xf9f9f9,
                text: 0x202020,
                muted: 0x5b5b5b,
                line: 0xdcdcdc,
                hover: 0xebebeb,
                selected: 0xe2f1fb,
                accent: 0x0067c0,
            }
        }
    }
}
#[derive(Clone)]
pub(super) struct Button {
    pub id: u16,
    pub label: &'static str,
    pub rect: Rect,
}
pub(super) struct Layout {
    pub height: i32,
    pub buttons: Vec<Button>,
    pub groups: Vec<(&'static str, Rect)>,
    pub colors: Vec<(u32, Rect)>,
    pub primary: Option<Rect>,
    pub secondary: Option<Rect>,
    pub size: Option<Rect>,
}
pub(super) fn contains(rect: Rect, x: i32, y: i32) -> bool {
    x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom
}
fn rect(x: i32, y: i32, w: i32, h: i32) -> Rect {
    Rect {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    }
}
impl Layout {
    pub fn new(width: i32, collapsed: bool) -> Self {
        let mut layout = Self {
            height: 44,
            buttons: Vec::new(),
            groups: Vec::new(),
            colors: Vec::new(),
            primary: None,
            secondary: None,
            size: None,
        };
        for (i, (id, label)) in [
            (1, "New"),
            (2, "Open"),
            (3, "Save"),
            (6, "Undo"),
            (7, "Redo"),
        ]
        .into_iter()
        .enumerate()
        {
            layout.add(id, label, rect(12 + i as i32 * 56, 7, 52, 30));
        }
        layout.add(54, "Theme", rect(width - 132, 7, 64, 30));
        layout.add(
            57,
            if collapsed { "Show" } else { "Hide" },
            rect(width - 64, 7, 52, 30),
        );
        if collapsed {
            return layout;
        }
        let (mut x, mut y) = (12, 48);
        for (group, w) in [
            ("Clipboard", 132),
            ("Image", 172),
            ("Tools", 204),
            ("Shapes", 128),
            ("Size", 92),
            ("Colors", 288),
        ] {
            if x + w > width - 12 && x > 12 {
                x = 12;
                y += 108;
            }
            layout.groups.push((group, rect(x, y, w, 98)));
            match group {
                "Clipboard" => {
                    layout.add(12, "Paste", rect(x + 4, y + 6, 54, 64));
                    layout.add(11, "Cut", rect(x + 62, y + 6, 66, 30));
                    layout.add(10, "Copy", rect(x + 62, y + 40, 66, 30));
                }
                "Image" => {
                    layout.add(108, "Select", rect(x + 4, y + 6, 58, 64));
                    for (i, (id, name)) in [(14, "Crop"), (15, "Resize"), (17, "Rotate")]
                        .into_iter()
                        .enumerate()
                    {
                        layout.add(id, name, rect(x + 66, y + 4 + i as i32 * 24, 102, 22));
                    }
                }
                "Tools" => {
                    for (i, (id, name)) in [
                        (100, "Pencil"),
                        (101, "Eraser"),
                        (105, "Fill"),
                        (106, "Picker"),
                        (109, "Text"),
                        (107, "Brush"),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        layout.add(
                            id,
                            name,
                            rect(
                                x + 4 + (i % 3) as i32 * 66,
                                y + 6 + (i / 3) as i32 * 34,
                                62,
                                30,
                            ),
                        );
                    }
                }
                "Shapes" => {
                    for (i, (id, name)) in [(102, "Line"), (103, "Rect."), (104, "Oval")]
                        .into_iter()
                        .enumerate()
                    {
                        layout.add(id, name, rect(x + 4 + i as i32 * 40, y + 6, 36, 64));
                    }
                }
                "Size" => {
                    layout.add(80, "−", rect(x + 4, y + 22, 25, 36));
                    layout.size = Some(rect(x + 30, y + 22, 32, 36));
                    layout.add(81, "+", rect(x + 63, y + 22, 25, 36));
                }
                "Colors" => {
                    layout.primary = Some(rect(x + 4, y + 8, 30, 30));
                    layout.secondary = Some(rect(x + 18, y + 24, 30, 30));
                    for (i, color) in COLORS.iter().enumerate() {
                        layout.colors.push((
                            *color,
                            rect(
                                x + 56 + (i % 10) as i32 * 22,
                                y + 5 + (i / 10) as i32 * 24,
                                19,
                                19,
                            ),
                        ));
                    }
                    layout.add(23, "Edit colors", rect(x + 56, y + 54, 216, 22));
                }
                _ => {}
            }
            x += w + 8;
            layout.height = y + 108;
        }
        layout
    }
    fn add(&mut self, id: u16, label: &'static str, rect: Rect) {
        self.buttons.push(Button { id, label, rect });
    }
    pub fn hit(&self, x: i32, y: i32) -> Option<u16> {
        self.buttons
            .iter()
            .find(|b| contains(b.rect, x, y))
            .map(|b| b.id)
    }
    pub fn color_at(&self, x: i32, y: i32) -> Option<u32> {
        self.colors
            .iter()
            .find(|(_, r)| contains(*r, x, y))
            .map(|(c, _)| *c)
    }
}
#[link(name = "gdi32")]
unsafe extern "system" {
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
    fn RoundRect(dc: Handle, l: i32, t: i32, r: i32, b: i32, ew: i32, eh: i32) -> i32;
    fn Ellipse(dc: Handle, l: i32, t: i32, r: i32, b: i32) -> i32;
    fn CreatePen(style: i32, width: i32, color: u32) -> Handle;
    fn MoveToEx(dc: Handle, x: i32, y: i32, old: *mut Point) -> i32;
    fn LineTo(dc: Handle, x: i32, y: i32) -> i32;
}
#[link(name = "user32")]
unsafe extern "system" {
    fn GetMenu(hwnd: Handle) -> Handle;
    fn GetSubMenu(menu: Handle, position: i32) -> Handle;
    fn CheckMenuItem(menu: Handle, id: u32, flags: u32) -> u32;
}
pub(super) unsafe fn font() -> Handle {
    unsafe {
        CreateFontW(
            -14,
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
            5,
            0,
            wide("Segoe UI").as_ptr(),
        )
    }
}
#[link(name = "advapi32")]
unsafe extern "system" {
    fn RegGetValueW(
        key: Handle,
        subkey: *const u16,
        value: *const u16,
        flags: u32,
        kind: *mut u32,
        data: *mut c_void,
        size: *mut u32,
    ) -> i32;
}
#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(hwnd: Handle, attribute: u32, value: *const c_void, size: u32) -> i32;
}
pub(super) fn system_dark() -> bool {
    let mut value = 1u32;
    let mut size = 4;
    // SAFETY: read-only registry query with initialized, correctly sized DWORD.
    unsafe {
        RegGetValueW(
            -2147483647,
            wide("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize").as_ptr(),
            wide("AppsUseLightTheme").as_ptr(),
            0x10,
            std::ptr::null_mut(),
            (&mut value as *mut u32).cast(),
            &mut size,
        ) == 0
            && value == 0
    }
}
pub(super) unsafe fn apply_theme(hwnd: Handle, cell: &RefCell<App>) {
    unsafe {
        let (dark, theme, collapsed, grid) = {
            let app = cell.borrow();
            (
                i32::from(app.theme.dark(app.system_dark)),
                app.theme,
                app.toolbar_collapsed,
                app.grid,
            )
        };
        // The documented Windows 11 immersive title-bar attribute; older builds can
        // decline it without affecting the editor's own themed surfaces.
        DwmSetWindowAttribute(hwnd, 20, (&dark as *const i32).cast(), 4);
        let view = GetSubMenu(GetMenu(hwnd), 3);
        if view != 0 {
            for (id, selected) in [
                (51, theme == Theme::Light),
                (52, theme == Theme::Dark),
                (53, theme == Theme::System),
                (57, !collapsed),
                (58, grid),
            ] {
                CheckMenuItem(view, id, if selected { 8 } else { 0 });
            }
        }
    }
}
unsafe fn rounded(dc: Handle, r: Rect, fill: u32, border: u32, circle: bool) {
    unsafe {
        let brush = CreateSolidBrush(native_color(fill));
        let pen = CreatePen(0, 1, native_color(border));
        if brush == 0 || pen == 0 {
            if brush != 0 {
                DeleteObject(brush);
            }
            if pen != 0 {
                DeleteObject(pen);
            }
            return;
        }
        let old_brush = SelectObject(dc, brush);
        let old_pen = SelectObject(dc, pen);
        if circle {
            Ellipse(dc, r.left, r.top, r.right, r.bottom);
        } else {
            RoundRect(dc, r.left, r.top, r.right, r.bottom, 8, 8);
        }
        SelectObject(dc, old_pen);
        SelectObject(dc, old_brush);
        DeleteObject(pen);
        DeleteObject(brush);
    }
}
unsafe fn text(dc: Handle, label: &str, mut r: Rect, color: u32) {
    unsafe {
        SetTextColor(dc, native_color(color));
        DrawTextW(dc, wide(label).as_ptr(), -1, &mut r, 0x25 | 0x8000);
    }
}
pub(super) unsafe fn toolbar(dc: Handle, app: &App, layout: &Layout) {
    unsafe {
        let palette = Palette::new(app.theme.dark(app.system_dark));
        for (name, r) in &layout.groups {
            let title = rect(r.left, r.bottom - 20, r.right - r.left, 18);
            text(dc, name, title, palette.muted);
            block(
                dc,
                rect(r.right + 3, r.top + 12, 1, 70),
                palette.line,
                false,
            );
        }
        for button in &layout.buttons {
            let selected =
                (100..110).contains(&button.id) && TOOLS[(button.id - 100) as usize].0 == app.tool;
            let hover = app.hover == Some(button.id);
            let fill = if selected {
                palette.selected
            } else if hover {
                palette.hover
            } else {
                palette.surface
            };
            rounded(
                dc,
                button.rect,
                fill,
                if selected {
                    palette.accent
                } else if hover {
                    palette.line
                } else {
                    palette.surface
                },
                false,
            );
            let glyph = match button.id {
                100 => Some("✎"),
                101 => Some("▱"),
                102 => Some("╱"),
                103 => Some("□"),
                104 => Some("○"),
                105 => Some("◈"),
                106 => Some("⌖"),
                107 => Some("〰"),
                108 => Some("▧"),
                109 => Some("A"),
                12 => Some("▣"),
                _ => None,
            };
            if let Some(glyph) = glyph {
                if button.rect.bottom - button.rect.top > 40 {
                    let mut icon = button.rect;
                    icon.bottom = icon.top + 32;
                    text(dc, glyph, icon, palette.text);
                    let mut caption = button.rect;
                    caption.top += 32;
                    text(dc, button.label, caption, palette.text);
                } else {
                    let mut icon = button.rect;
                    icon.right = icon.left + 18;
                    text(dc, glyph, icon, palette.text);
                    let mut caption = button.rect;
                    caption.left += 18;
                    text(dc, button.label, caption, palette.text);
                }
            } else {
                text(dc, button.label, button.rect, palette.text);
            }
        }
        if let Some(r) = layout.secondary {
            rounded(dc, r, app.secondary, palette.line, true);
        }
        if let Some(r) = layout.primary {
            rounded(dc, r, app.color, palette.accent, true);
        }
        for (color, r) in &layout.colors {
            rounded(
                dc,
                *r,
                *color,
                if *color == app.color {
                    palette.accent
                } else {
                    palette.line
                },
                true,
            );
        }
        if let Some(r) = layout.size {
            text(dc, &app.size.to_string(), r, palette.text);
        }
    }
}
pub(super) fn slider(width: i32, height: i32) -> Rect {
    rect(width - 218, height - 22, 122, 18)
}
pub(super) fn footer_hit(width: i32, height: i32, x: i32, y: i32) -> Option<u16> {
    if y < height - 28 {
        return None;
    }
    if contains(rect(width - 254, height - 25, 28, 24), x, y) {
        Some(61)
    } else if contains(rect(width - 90, height - 25, 28, 24), x, y) {
        Some(62)
    } else if contains(rect(width - 58, height - 25, 52, 24), x, y) {
        Some(30)
    } else {
        None
    }
}
pub(super) fn slider_zoom(r: Rect, x: i32) -> u32 {
    (25.0 * 2f64.powf(((x - r.left) as f64 / (r.right - r.left) as f64).clamp(0.0, 1.0) * 5.0))
        .round() as u32
}
pub(super) unsafe fn footer(dc: Handle, app: &App, width: i32, height: i32) {
    unsafe {
        let p = Palette::new(app.theme.dark(app.system_dark));
        let mut r = rect(12, height - 26, (width - 290).max(1), 24);
        SetTextColor(dc, native_color(p.text));
        let info = format!(
            "{} × {} px   ·   {:?}   ·   {}%",
            app.canvas.width(),
            app.canvas.height(),
            app.tool,
            app.view.0 as u64 * 100 / app.canvas.width() as u64
        );
        DrawTextW(dc, wide(&info).as_ptr(), -1, &mut r, 0x24 | 0x8000);
        for (id, label, x, w) in [
            (61, "−", width - 254, 28),
            (62, "+", width - 90, 28),
            (30, "Fit", width - 58, 52),
        ] {
            let r = rect(x, height - 25, w, 24);
            rounded(
                dc,
                r,
                if app.hover == Some(id) {
                    p.hover
                } else {
                    p.surface
                },
                p.surface,
                false,
            );
            text(dc, label, r, p.text);
        }
        let r = slider(width, height);
        block(
            dc,
            rect(r.left, r.top + 8, r.right - r.left, 3),
            p.line,
            false,
        );
        let zoom = (app.view.0 as f64 * 100.0 / app.canvas.width() as f64).clamp(25.0, 800.0);
        let x = r.left + ((zoom / 25.0).log2() / 5.0 * (r.right - r.left) as f64) as i32;
        rounded(dc, rect(x - 6, r.top + 2, 13, 15), p.accent, p.accent, true);
    }
}
pub(super) unsafe fn pixel_grid(dc: Handle, app: &App, client: Rect) {
    unsafe {
        let cell = app.view.0 as f64 / app.canvas.width() as f64;
        let cell_y = app.view.1 as f64 / app.canvas.height() as f64;
        if !app.grid || cell.min(cell_y) < 4.0 {
            return;
        }
        let p = Palette::new(app.theme.dark(app.system_dark));
        let pen = CreatePen(0, 1, native_color(p.line));
        if pen == 0 {
            return;
        }
        let old = SelectObject(dc, pen);
        let left = app.origin.0 - app.pan.0;
        let top = app.origin.1 - app.pan.1;
        for x in (app.pan.0 as f64 / cell).floor().max(0.0) as i32
            ..=app
                .canvas
                .width()
                .min(((client.right - left) as f64 / cell).ceil().max(0.0) as u32)
                as i32
        {
            let xx = left + (x as f64 * cell).round() as i32;
            MoveToEx(dc, xx, top, std::ptr::null_mut());
            LineTo(dc, xx, top + app.view.1);
        }
        for y in (app.pan.1 as f64 / cell_y).floor().max(0.0) as i32
            ..=app
                .canvas
                .height()
                .min(((client.bottom - top) as f64 / cell_y).ceil().max(0.0) as u32)
                as i32
        {
            let yy = top + (y as f64 * cell_y).round() as i32;
            MoveToEx(dc, left, yy, std::ptr::null_mut());
            LineTo(dc, left + app.view.0, yy);
        }
        SelectObject(dc, old);
        DeleteObject(pen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn groups_reflow_and_keep_every_button_in_the_toolbar() {
        for width in [600, 800, 940, 1280] {
            let l = Layout::new(width, false);
            assert_eq!(l.colors.len(), 20);
            assert_eq!(l.groups.len(), 6);
            for b in &l.buttons {
                assert!(
                    b.rect.left >= 0 && b.rect.right <= width,
                    "{} at {width}",
                    b.label
                );
                assert!(b.rect.bottom <= l.height);
                assert_eq!(l.hit(b.rect.left + 2, b.rect.top + 2), Some(b.id));
            }
        }
    }
    #[test]
    fn collapsed_toolbar_and_zoom_slider_have_consistent_hit_targets() {
        let l = Layout::new(800, true);
        assert_eq!(l.height, 44);
        assert!(l.colors.is_empty());
        assert!(l.buttons.iter().any(|b| b.id == 57));
        let r = slider(940, 880);
        assert_eq!(slider_zoom(r, r.left - 10), 25);
        assert_eq!(slider_zoom(r, r.right + 10), 800);
    }
    #[test]
    fn explicit_theme_overrides_system_appearance() {
        assert!(Theme::Dark.dark(false));
        assert!(!Theme::Light.dark(true));
        assert!(Theme::System.dark(true));
    }
}
