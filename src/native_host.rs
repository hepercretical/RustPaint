//! Native boundary for the executable. The safe library contains no FFI.
use mspaint_port::runtime::Host;
pub struct NativeHost;

#[cfg(windows)]
mod windows {
    #[repr(C)]
    #[derive(Default)]
    pub struct FileTime {
        pub low: u32,
        pub high: u32,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn GetSystemTimeAsFileTime(time: *mut FileTime);
        pub fn GetCurrentThreadId() -> u32;
        pub fn GetCurrentProcessId() -> u32;
        pub fn QueryPerformanceCounter(counter: *mut i64) -> i32;
    }
}

#[cfg(windows)]
impl Host for NativeHost {
    fn file_time(&mut self) -> Result<u64, String> {
        let mut value = windows::FileTime::default();
        // SAFETY: correctly aligned writable FILETIME with the system ABI.
        unsafe { windows::GetSystemTimeAsFileTime(&mut value) };
        Ok(u64::from(value.low) | (u64::from(value.high) << 32))
    }
    fn thread_id(&mut self) -> Result<u32, String> {
        // SAFETY: no arguments; no borrowed memory or handles.
        Ok(unsafe { windows::GetCurrentThreadId() })
    }
    fn process_id(&mut self) -> Result<u32, String> {
        // SAFETY: no arguments; no borrowed memory or handles.
        Ok(unsafe { windows::GetCurrentProcessId() })
    }
    fn performance_counter(&mut self) -> Result<u64, String> {
        let mut value = 0i64;
        // SAFETY: writable aligned LARGE_INTEGER-sized storage for this call.
        if unsafe { windows::QueryPerformanceCounter(&mut value) } == 0 {
            return Err(format!(
                "QueryPerformanceCounter failed: {}",
                std::io::Error::last_os_error()
            ));
        }
        Ok(value as u64)
    }
}

#[cfg(not(windows))]
impl Host for NativeHost {
    fn file_time(&mut self) -> Result<u64, String> {
        Err("native startup requires Windows".into())
    }
    fn thread_id(&mut self) -> Result<u32, String> {
        Err("native startup requires Windows".into())
    }
    fn process_id(&mut self) -> Result<u32, String> {
        Err("native startup requires Windows".into())
    }
    fn performance_counter(&mut self) -> Result<u64, String> {
        Err("native startup requires Windows".into())
    }
}
