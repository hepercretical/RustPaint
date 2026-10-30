// Rust's self-contained MinGW toolchain omits the GDI+ import library. Generate
// the small import table using bundled LLD; ship no third-party DLLs.
use std::{env, fs, path::PathBuf, process::Command};
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows")
        || env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("gnu")
    {
        return;
    }
    let output = Command::new(env::var_os("RUSTC").expect("RUSTC"))
        .args(["--print", "sysroot"])
        .output()
        .expect("query Rust sysroot");
    assert!(output.status.success(), "could not query Rust sysroot");
    let root = PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("UTF-8 sysroot")
            .trim(),
    );
    let target = env::var("TARGET").expect("TARGET");
    let tool = root
        .join("lib/rustlib")
        .join(&target)
        .join("bin/rust-lld.exe");
    let directory = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let definition = directory.join("gdiplus.def");
    fs::write(&definition,"LIBRARY gdiplus.dll\nEXPORTS\nGdiplusStartup\nGdiplusShutdown\nGdipCreateBitmapFromFile\nGdipCreateBitmapFromScan0\nGdipDisposeImage\nGdipGetImageWidth\nGdipGetImageHeight\nGdipBitmapLockBits\nGdipBitmapUnlockBits\nGdipSaveImageToFile\nGdipGetImageEncodersSize\nGdipGetImageEncoders\n").expect("write GDI+ definition");
    let status = Command::new(tool)
        .args([
            "-flavor",
            "link",
            "/lib",
            if target.starts_with("x86_64") {
                "/machine:x64"
            } else {
                "/machine:x86"
            },
        ])
        .arg(format!("/def:{}", definition.display()))
        .arg(format!("/out:{}", directory.join("libgdiplus.a").display()))
        .status()
        .expect("run bundled LLD");
    assert!(status.success(), "could not create GDI+ import library");
    let definition = directory.join("dwmapi.def");
    fs::write(
        &definition,
        "LIBRARY dwmapi.dll\nEXPORTS\nDwmSetWindowAttribute\n",
    )
    .expect("write DWM definition");
    let tool = root
        .join("lib/rustlib")
        .join(&target)
        .join("bin/rust-lld.exe");
    let status = Command::new(tool)
        .args([
            "-flavor",
            "link",
            "/lib",
            if target.starts_with("x86_64") {
                "/machine:x64"
            } else {
                "/machine:x86"
            },
        ])
        .arg(format!("/def:{}", definition.display()))
        .arg(format!("/out:{}", directory.join("libdwmapi.a").display()))
        .status()
        .expect("run bundled LLD");
    assert!(status.success(), "could not create DWM import library");
    println!("cargo:rustc-link-search=native={}", directory.display());
}
