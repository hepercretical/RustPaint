//! Host integration for recovered startup and independently callable routines.
//! Image addresses never cross into native Windows APIs as host pointers.
use crate::recovered::{
    adapters24,
    call_support::{MemoryFault, MemoryImage},
};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

pub const DEFAULT_COOKIE: u64 = 0x2b992ddfa232;
const BASE: u64 = 0x140179000;
const FRAME: u64 = BASE + 0x200;

/// Host operations required by the recovered cookie initializer.
pub trait Host {
    fn file_time(&mut self) -> Result<u64, String>;
    fn thread_id(&mut self) -> Result<u32, String>;
    fn process_id(&mut self) -> Result<u32, String>;
    fn performance_counter(&mut self) -> Result<u64, String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    UnsupportedCall(String),
    Arguments {
        call: String,
        expected: usize,
        actual: usize,
    },
    Host(String),
    Memory(MemoryFault),
    Adapter(String),
}
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedCall(name) => write!(f, "runtime operation not implemented: {name}"),
            Self::Arguments {
                call,
                expected,
                actual,
            } => write!(f, "{call}: expected {expected} arguments, got {actual}"),
            Self::Host(message) | Self::Adapter(message) => f.write_str(message),
            Self::Memory(fault) => std::fmt::Display::fmt(fault, f),
        }
    }
}
impl std::error::Error for RuntimeError {}
impl From<MemoryFault> for RuntimeError {
    fn from(f: MemoryFault) -> Self {
        Self::Memory(f)
    }
}

/// Checked dispatch: unsupported operations and host errors never become success.
pub fn dispatch(
    host: &mut impl Host,
    name: &str,
    args: &[u64],
    memory: &mut MemoryImage,
) -> Result<u64, RuntimeError> {
    let expected = match name {
        "GetSystemTimeAsFileTime" | "QueryPerformanceCounter" => 1,
        "GetCurrentThreadId" | "GetCurrentProcessId" => 0,
        _ => return Err(RuntimeError::UnsupportedCall(name.into())),
    };
    if args.len() != expected {
        return Err(RuntimeError::Arguments {
            call: name.into(),
            expected,
            actual: args.len(),
        });
    }
    match name {
        "GetSystemTimeAsFileTime" => {
            let value = host.file_time().map_err(RuntimeError::Host)?;
            memory.write_u64(args[0], value)?;
            Ok(0)
        }
        "QueryPerformanceCounter" => {
            let value = host.performance_counter().map_err(RuntimeError::Host)?;
            memory.write_u64(args[0], value)?;
            Ok(1)
        }
        "GetCurrentThreadId" => Ok(u64::from(host.thread_id().map_err(RuntimeError::Host)?)),
        "GetCurrentProcessId" => Ok(u64::from(host.process_id().map_err(RuntimeError::Host)?)),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartupState {
    pub cookie: u64,
    pub complement: u64,
}

pub struct Runtime<H> {
    host: H,
    memory: MemoryImage,
}
struct RuntimeTrap(RuntimeError);
impl<H: Host> Runtime<H> {
    pub fn into_host(self) -> H {
        self.host
    }
    pub fn new(host: H) -> Self {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0; 0x300],
        };
        memory
            .write_u64(BASE + 0x40, DEFAULT_COOKIE)
            .expect("fixed runtime layout");
        Self { host, memory }
    }
    pub fn state(&self) -> StartupState {
        StartupState {
            cookie: self
                .memory
                .read_u64(BASE + 0x40)
                .expect("fixed runtime layout"),
            complement: self
                .memory
                .read_u64(BASE + 0x80)
                .expect("fixed runtime layout"),
        }
    }
    /// Run the original translated initializer, retaining its frame offsets.
    /// This initializes image-model state, not the host executable's CRT cookie.
    pub fn initialize(&mut self) -> Result<StartupState, RuntimeError> {
        let Self { host, memory } = self;
        let result = catch_unwind(AssertUnwindSafe(|| {
            adapters24::fun_1400b07f8(
                &[],
                memory,
                &mut |symbol| match symbol {
                    "DAT_140179040" => BASE + 0x40,
                    "DAT_140179080" => BASE + 0x80,
                    "STACK_local_res8" => FRAME + 8,
                    "STACK_local_res10" => FRAME + 16,
                    "STACK_local_18" => FRAME - 24,
                    _ => resume_unwind(Box::new(RuntimeTrap(RuntimeError::UnsupportedCall(
                        symbol.into(),
                    )))),
                },
                &mut |name, args, memory| match dispatch(host, name, args, memory) {
                    Ok(word) => word,
                    // The generated callback cannot return Result. Unwind only to
                    // this boundary, preserving prior writes and skipping later calls.
                    Err(error) => resume_unwind(Box::new(RuntimeTrap(error))),
                },
            )
        }));
        match result {
            Ok(Ok(_)) => Ok(self.state()),
            Ok(Err(error)) => Err(RuntimeError::Adapter(format!("startup adapter: {error:?}"))),
            Err(payload) => match payload.downcast::<RuntimeTrap>() {
                Ok(trap) => Err(trap.0),
                Err(other) => resume_unwind(other),
            },
        }
    }
}
