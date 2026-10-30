//! Owned, short-lived GDI+ codec boundary. No original Paint DLLs are required.
use mspaint_port::canvas::Canvas;
use std::{ffi::c_void, os::windows::ffi::OsStrExt, path::Path};
#[repr(C)]
struct Startup {
    version: u32,
    debug: usize,
    suppress_thread: i32,
    suppress_codecs: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Guid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}
#[repr(C)]
struct Codec {
    clsid: Guid,
    format: Guid,
    name: *const u16,
    dll: *const u16,
    description: *const u16,
    extensions: *const u16,
    mime: *const u16,
    flags: u32,
    version: u32,
    sig_count: u32,
    sig_size: u32,
    sig_pattern: *const u8,
    sig_mask: *const u8,
}
#[repr(C)]
struct Region {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
#[repr(C)]
#[derive(Default)]
struct Bits {
    width: u32,
    height: u32,
    stride: i32,
    format: i32,
    scan: *mut u8,
    reserved: usize,
}
#[link(name = "gdiplus")]
unsafe extern "system" {
    fn GdiplusStartup(token: *mut usize, input: *const Startup, output: *mut c_void) -> i32;
    fn GdiplusShutdown(token: usize);
    fn GdipCreateBitmapFromFile(path: *const u16, bitmap: *mut *mut c_void) -> i32;
    fn GdipCreateBitmapFromScan0(
        w: i32,
        h: i32,
        stride: i32,
        format: i32,
        pixels: *mut u8,
        bitmap: *mut *mut c_void,
    ) -> i32;
    fn GdipDisposeImage(image: *mut c_void) -> i32;
    fn GdipGetImageWidth(image: *mut c_void, width: *mut u32) -> i32;
    fn GdipGetImageHeight(image: *mut c_void, height: *mut u32) -> i32;
    fn GdipBitmapLockBits(
        image: *mut c_void,
        region: *const Region,
        flags: u32,
        format: i32,
        bits: *mut Bits,
    ) -> i32;
    fn GdipBitmapUnlockBits(image: *mut c_void, bits: *mut Bits) -> i32;
    fn GdipSaveImageToFile(
        image: *mut c_void,
        path: *const u16,
        encoder: *const Guid,
        parameters: *const c_void,
    ) -> i32;
    fn GdipGetImageEncodersSize(count: *mut u32, size: *mut u32) -> i32;
    fn GdipGetImageEncoders(count: u32, size: u32, buffer: *mut Codec) -> i32;
}
fn check(status: i32) -> Result<(), String> {
    if status == 0 {
        Ok(())
    } else {
        Err(format!("Windows image codec failed (GDI+ status {status})"))
    }
}
struct Engine(usize);
impl Engine {
    fn start() -> Result<Self, String> {
        let mut token = 0;
        let input = Startup {
            version: 1,
            debug: 0,
            suppress_thread: 0,
            suppress_codecs: 0,
        };
        // SAFETY: initialized ABI input, writable token, no optional output requested.
        unsafe {
            check(GdiplusStartup(&mut token, &input, std::ptr::null_mut()))?;
        }
        Ok(Self(token))
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        unsafe {
            GdiplusShutdown(self.0);
        }
    }
}
struct Image(*mut c_void);
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            GdipDisposeImage(self.0);
        }
    }
}
fn path_wide(path: &Path) -> Vec<u16> {
    path.as_os_str().encode_wide().chain(Some(0)).collect()
}
pub fn load(path: &Path) -> Result<Canvas, String> {
    let _engine = Engine::start()?;
    let name = path_wide(path);
    let mut pointer = std::ptr::null_mut();
    // SAFETY: GDI+ owns the image and locked storage. Image is disposed before the
    // engine shuts down; every read is within the validated dimensions and stride.
    unsafe {
        check(GdipCreateBitmapFromFile(name.as_ptr(), &mut pointer))?;
        let image = Image(pointer);
        let (mut width, mut height) = (0, 0);
        check(GdipGetImageWidth(image.0, &mut width))?;
        check(GdipGetImageHeight(image.0, &mut height))?;
        let checked = Canvas::new(width, height)?;
        let mut pixels = Vec::with_capacity(checked.pixels().len());
        drop(checked);
        let region = Region {
            x: 0,
            y: 0,
            width: width as i32,
            height: height as i32,
        };
        let mut bits = Bits::default();
        check(GdipBitmapLockBits(image.0, &region, 1, 0x26200a, &mut bits))?;
        if bits.scan.is_null() || (bits.stride.unsigned_abs() as usize) < (width as usize * 4) {
            GdipBitmapUnlockBits(image.0, &mut bits);
            return Err("invalid decoded image stride".into());
        }
        for y in 0..height {
            let row = bits.scan.offset(y as isize * bits.stride as isize);
            for x in 0..width {
                let p = std::ptr::read_unaligned(row.add(x as usize * 4).cast::<u32>());
                let alpha = p >> 24;
                let blend = |shift: u32| -> u32 {
                    let channel = (p >> shift) & 255u32;
                    (channel * alpha + 255 * (255 - alpha) + 127) / 255
                };
                pixels.push((blend(16) << 16) | (blend(8) << 8) | blend(0));
            }
        }
        check(GdipBitmapUnlockBits(image.0, &mut bits))?;
        Canvas::from_pixels(width, height, pixels)
    }
}
pub fn save(path: &Path, canvas: &Canvas) -> Result<(), String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("png");
    let temporary = parent.join(format!(
        ".rust-paint-{}-{}.{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed),
        extension
    ));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| e.to_string())?;
    drop(file);
    let result = save_encoded(&temporary, canvas)
        .and_then(|()| std::fs::rename(&temporary, path).map_err(|e| e.to_string()));
    if result.is_err() {
        let _ = std::fs::remove_file(&temporary);
    }
    result
}
fn save_encoded(path: &Path, canvas: &Canvas) -> Result<(), String> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png")
        .to_ascii_lowercase();
    if extension == "bmp" {
        return std::fs::write(path, canvas.bmp()?).map_err(|e| e.to_string());
    }
    let mime = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "tif" | "tiff" => "image/tiff",
        _ => return Err("Choose a PNG, JPEG, BMP, GIF or TIFF filename".into()),
    };
    let _engine = Engine::start()?;
    let mut pixels: Vec<u32> = canvas.pixels().iter().map(|p| p | 0xff000000).collect();
    let mut pointer = std::ptr::null_mut();
    // SAFETY: pixel storage outlives the GDI+ image. The codec table is aligned and
    // sized from GDI+; all native resources are disposed before shutdown.
    unsafe {
        check(GdipCreateBitmapFromScan0(
            canvas.width() as i32,
            canvas.height() as i32,
            canvas.width() as i32 * 4,
            0x26200a,
            pixels.as_mut_ptr().cast(),
            &mut pointer,
        ))?;
        let image = Image(pointer);
        let (mut count, mut size) = (0, 0);
        check(GdipGetImageEncodersSize(&mut count, &mut size))?;
        if (size as usize) < (count as usize).saturating_mul(std::mem::size_of::<Codec>()) {
            return Err("invalid codec table size".into());
        }
        let mut buffer = vec![0u64; (size as usize).div_ceil(8)];
        check(GdipGetImageEncoders(
            count,
            size,
            buffer.as_mut_ptr().cast(),
        ))?;
        let codecs = std::slice::from_raw_parts(buffer.as_ptr().cast::<Codec>(), count as usize);
        let mut encoder = None;
        for codec in codecs {
            if codec.mime.is_null() {
                continue;
            }
            let mut units = Vec::new();
            for n in 0..128 {
                let unit = *codec.mime.add(n);
                if unit == 0 {
                    break;
                }
                units.push(unit);
            }
            if String::from_utf16_lossy(&units) == mime {
                encoder = Some(codec.clsid);
                break;
            }
        }
        let encoder = encoder.ok_or("requested image encoder is not available")?;
        let name = path_wide(path);
        check(GdipSaveImageToFile(
            image.0,
            name.as_ptr(),
            &encoder,
            std::ptr::null(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn png_roundtrip_uses_installed_windows_codec() {
        let path =
            std::env::temp_dir().join(format!("rust-paint-codec-{}.png", std::process::id()));
        let canvas =
            Canvas::from_pixels(2, 2, vec![0xff0000, 0x00ff00, 0x0000ff, 0xffffff]).unwrap();
        save(&path, &canvas).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.pixels(), canvas.pixels());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn formats_roundtrip_and_failed_save_keeps_existing_file() {
        let canvas = Canvas::from_pixels(16, 16, vec![0x334455; 256]).unwrap();
        for extension in ["jpg", "gif", "tif", "bmp"] {
            let path = std::env::temp_dir().join(format!(
                "rust-paint-formats-{}.{}",
                std::process::id(),
                extension
            ));
            save(&path, &canvas).unwrap();
            let loaded = load(&path).unwrap();
            assert_eq!((loaded.width(), loaded.height()), (16, 16));
            if extension == "jpg" {
                let color = loaded.pixels()[0];
                for shift in [0, 8, 16] {
                    assert!(
                        (((color >> shift) & 255) as i32 - ((0x334455u32 >> shift) & 255) as i32)
                            .abs()
                            < 8
                    );
                }
            } else if extension != "gif" {
                assert_eq!(loaded.pixels(), canvas.pixels());
            }
            save(&path, &canvas).unwrap();
            std::fs::remove_file(path).unwrap();
        }
        let path = std::env::temp_dir().join(format!(
            "rust-paint-preserve-{}.invalid",
            std::process::id()
        ));
        std::fs::write(&path, b"keep this file").unwrap();
        assert!(save(&path, &canvas).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"keep this file");
        std::fs::remove_file(path).unwrap();
    }
}
