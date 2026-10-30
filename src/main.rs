//! Manual runner for recovered routines with native startup integration.
#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]
#[cfg(windows)]
mod desktop;
#[cfg(windows)]
mod image_codec;
mod native_host;
use mspaint_port::{bitmap, numeric, runtime::Runtime, status};
use std::{
    io::{self, Write},
    process::ExitCode,
};

const HELP: &str = "MS Paint Rust port\n\
Launch without arguments for the graphical editor.\n\n\
Usage: mspaint-port [IMAGE | --gui | --smoke-test | --interactive | startup | hresult CODE | decimal U32 | dib FILE]\n\
Numbers accept decimal or 0x-prefixed hexadecimal.\n\
dib accepts raw DIB headers or BMP files; output is the recovered size calculation.\n\
Use --interactive for a command prompt; help and quit are available there.";

fn number(value: &str) -> Result<u32, String> {
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16)
    } else {
        value.parse()
    }
    .map_err(|_| format!("invalid unsigned 32-bit number: {value}"))
}
fn command(runtime: &mut Runtime<native_host::NativeHost>, args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        Some("help" | "--help" | "-h") if args.len() == 1 => println!("{HELP}"),
        Some("startup") if args.len() == 1 => {
            let state = runtime.initialize().map_err(|e| e.to_string())?;
            println!(
                "Recovered startup cookie initialized: {:012x}; complement {:016x}",
                state.cookie, state.complement
            );
        }
        Some("hresult") if args.len() == 2 => {
            println!("0x{:08x}", status::hresult_from_win32(number(&args[1])?))
        }
        Some("decimal") if args.len() == 2 => {
            let mut bytes = [0; 10];
            let len =
                numeric::write_decimal(&mut bytes, number(&args[1])?).map_err(|e| e.to_string())?;
            println!(
                "{}",
                std::str::from_utf8(&bytes[..len]).map_err(|e| e.to_string())?
            );
        }
        Some("dib") if args.len() == 2 => {
            let bytes = std::fs::read(&args[1]).map_err(|e| format!("{}: {e}", args[1]))?;
            let header = if bytes.starts_with(b"BM") {
                bytes.get(14..).ok_or("truncated BMP file header")?
            } else {
                &bytes
            };
            println!(
                "Recovered DIB size: {} bytes",
                bitmap::dib_size(header).map_err(|e| e.to_string())?
            );
        }
        _ => return Err("unknown command or wrong argument count; use --help".into()),
    }
    Ok(())
}
fn run() -> Result<(), String> {
    let mut runtime = Runtime::new(native_host::NativeHost);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let image = args.len() == 1 && std::path::Path::new(&args[0]).is_file();
    if args.is_empty() || args == ["--gui"] || args == ["--smoke-test"] || image {
        runtime.initialize().map_err(|e| e.to_string())?;
        #[cfg(windows)]
        return desktop::run(
            if image {
                Some(args[0].clone().into())
            } else {
                None
            },
            args == ["--smoke-test"],
        );
        #[cfg(not(windows))]
        return Err("the desktop editor currently requires Windows".into());
    }
    if args == ["--interactive"] {
        println!("{HELP}");
        let stdin = io::stdin();
        loop {
            print!("mspaint-port> ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut line = String::new();
            if stdin.read_line(&mut line).map_err(|e| e.to_string())? == 0 {
                break;
            }
            let line = line.trim();
            if matches!(line, "quit" | "exit") {
                break;
            }
            if line.is_empty() {
                continue;
            }
            // Keep file paths (including spaces) as a single argument.
            let words = if let Some(path) = line.strip_prefix("dib ") {
                vec!["dib".into(), path.trim().trim_matches('"').into()]
            } else {
                line.split_whitespace().map(str::to_owned).collect()
            };
            if let Err(error) = command(&mut runtime, &words) {
                eprintln!("Error: {error}");
            }
        }
        Ok(())
    } else {
        command(&mut runtime, &args)
    }
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            #[cfg(windows)]
            if std::env::args().len() == 1
                || std::env::args().any(|s| s == "--gui")
                || std::env::args()
                    .nth(1)
                    .is_some_and(|s| std::path::Path::new(&s).is_file())
            {
                desktop::show_error(&error);
            }
            ExitCode::FAILURE
        }
    }
}
