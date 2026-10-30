//! UI-thread-owned back buffer. Never present an uninitialized/partial frame.
use super::*;

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(dc: Handle) -> Handle;
    fn CreateCompatibleBitmap(dc: Handle, width: i32, height: i32) -> Handle;
    fn DeleteDC(dc: Handle) -> i32;
    fn BitBlt(
        destination: Handle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        source: Handle,
        source_x: i32,
        source_y: i32,
        operation: u32,
    ) -> i32;
}

#[derive(Default)]
pub(super) struct BackBuffer {
    dc: Handle,
    bitmap: Handle,
    previous: Handle,
    size: (i32, i32),
}

impl BackBuffer {
    // SAFETY: reference is a live display DC on the UI thread. The bitmap is
    // compatible with that DC, not the memory DC's default monochrome bitmap.
    pub unsafe fn prepare(&mut self, reference: Handle, width: i32, height: i32) -> Option<Handle> {
        unsafe {
            if width <= 0 || height <= 0 {
                return None;
            }
            if self.dc != 0 && self.size == (width, height) {
                return Some(self.dc);
            }
            let dc = CreateCompatibleDC(reference);
            if dc == 0 {
                return None;
            }
            let bitmap = CreateCompatibleBitmap(reference, width, height);
            if bitmap == 0 {
                DeleteDC(dc);
                return None;
            }
            let previous = SelectObject(dc, bitmap);
            if previous == 0 || previous == -1 {
                DeleteObject(bitmap);
                DeleteDC(dc);
                return None;
            }
            // Allocate first: a failed resize leaves the existing buffer owned.
            *self = Self {
                dc,
                bitmap,
                previous,
                size: (width, height),
            };
            Some(dc)
        }
    }

    // SAFETY: destination is a live DC; prepare and complete frame rendering
    // must precede this call. BeginPaint's destination clip remains in force.
    pub unsafe fn present(&self, destination: Handle, damage: Rect) -> bool {
        unsafe {
            self.dc != 0
                && BitBlt(
                    destination,
                    damage.left,
                    damage.top,
                    damage.right - damage.left,
                    damage.bottom - damage.top,
                    self.dc,
                    damage.left,
                    damage.top,
                    0x00cc0020, // SRCCOPY: publish the completed pixels together.
                ) != 0
        }
    }
}

impl Drop for BackBuffer {
    fn drop(&mut self) {
        // SAFETY: handles are exclusively owned by this UI-thread buffer. A
        // selected bitmap must be deselected before deleting it and its DC.
        unsafe {
            if self.dc != 0 {
                SelectObject(self.dc, self.previous);
                DeleteObject(self.bitmap);
                DeleteDC(self.dc);
            }
        }
    }
}
