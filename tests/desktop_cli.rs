use std::process::Command;
#[test]
fn compiled_runner_formats_real_values_and_rejects_unknown_commands() {
    let executable = env!("CARGO_BIN_EXE_mspaint-port");
    for (args, expected) in [
        (["hresult", "5"], "0x80070005"),
        (["decimal", "4294967295"], "4294967295"),
    ] {
        let output = Command::new(executable).args(args).output().unwrap();
        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), expected);
    }
    let output = Command::new(executable)
        .arg("missing-command")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .unwrap()
        .contains("unknown command"));
}
#[test]
#[cfg(windows)]
fn native_windows_startup_returns_a_real_cookie() {
    let output = Command::new(env!("CARGO_BIN_EXE_mspaint-port"))
        .arg("startup")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("Recovered startup cookie initialized:"));
}
