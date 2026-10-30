use mspaint_port::recovered::call_support::MemoryImage;
use mspaint_port::runtime::{dispatch, Host, Runtime, RuntimeError, DEFAULT_COOKIE};
#[derive(Default)]
struct Fake {
    calls: Vec<&'static str>,
    fail: bool,
}
impl Host for Fake {
    fn file_time(&mut self) -> Result<u64, String> {
        self.calls.push("time");
        Ok(0x1122334455667788)
    }
    fn thread_id(&mut self) -> Result<u32, String> {
        self.calls.push("thread");
        Ok(11)
    }
    fn process_id(&mut self) -> Result<u32, String> {
        self.calls.push("process");
        Ok(22)
    }
    fn performance_counter(&mut self) -> Result<u64, String> {
        self.calls.push("counter");
        if self.fail {
            Err("counter failed".into())
        } else {
            Ok(0xaabbccddeeff0011)
        }
    }
}
#[test]
fn actual_initializer_uses_host_entropy_and_is_idempotent() {
    let mut runtime = Runtime::new(Fake::default());
    let state = runtime.initialize().unwrap();
    let counter = 0xaabbccddeeff0011u64;
    let expected = (((counter as u32 as u64) << 32)
        ^ counter
        ^ 0x1122334455667788
        ^ 11
        ^ 22
        ^ (0x140179000 + 0x200 - 24))
        & 0xffffffffffff;
    assert_eq!(state.cookie, expected);
    assert_eq!(state.complement, !expected);
    assert_ne!(state.cookie, DEFAULT_COOKIE);
    assert_eq!(runtime.initialize().unwrap(), state);
    assert_eq!(
        runtime.into_host().calls,
        vec!["time", "thread", "process", "counter"]
    );
}
#[test]
fn host_failure_stops_before_cookie_commit_and_can_be_retried() {
    let mut runtime = Runtime::new(Fake {
        fail: true,
        ..Fake::default()
    });
    assert!(matches!(runtime.initialize(), Err(RuntimeError::Host(_))));
    assert_eq!(runtime.state().cookie, DEFAULT_COOKIE);
    assert_eq!(runtime.state().complement, 0);
}
#[test]
fn dispatcher_rejects_missing_dependencies_arguments_and_memory() {
    let mut host = Fake::default();
    let mut memory = MemoryImage {
        base: 100,
        bytes: vec![0; 8],
    };
    assert!(matches!(
        dispatch(&mut host, "AfxWinMain", &[], &mut memory),
        Err(RuntimeError::UnsupportedCall(_))
    ));
    assert!(matches!(
        dispatch(&mut host, "QueryPerformanceCounter", &[], &mut memory),
        Err(RuntimeError::Arguments { .. })
    ));
    assert!(host.calls.is_empty());
    assert!(matches!(
        dispatch(&mut host, "GetSystemTimeAsFileTime", &[101], &mut memory),
        Err(RuntimeError::Memory(_))
    ));
    assert_eq!(memory.bytes, vec![0; 8]);
    assert_eq!(
        dispatch(&mut host, "GetCurrentThreadId", &[], &mut memory).unwrap(),
        11
    );
}
