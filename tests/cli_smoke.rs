use std::process::Command;

/// Smoke test: the binary runs and prints help without panicking.
#[test]
fn help_exits_successfully() {
    let status = Command::new(env!("CARGO_BIN_EXE_cargo-smith"))
        .arg("--help")
        .status()
        .expect("failed to run cargo-smith");
    assert!(status.success());
}

/// Smoke test: --version prints a version string.
#[test]
fn version_exits_successfully() {
    let status = Command::new(env!("CARGO_BIN_EXE_cargo-smith"))
        .arg("--version")
        .status()
        .expect("failed to run cargo-smith");
    assert!(status.success());
}