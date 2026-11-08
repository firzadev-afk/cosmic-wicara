//! Workspace scaffold checks.

use std::process::Command;

fn workspace_toml() -> String {
    let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/../../Cargo.toml");
    std::fs::read_to_string(manifest).expect("read workspace Cargo.toml")
}

#[test]
fn workspace_check() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1"])
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("metadata stdout is UTF-8");

    for name in ["cli", "core", "app"] {
        assert!(
            stdout.contains(&format!(r#""name":"{name}""#)),
            "cargo metadata should list member {name:?}"
        );
    }
    assert!(
        stdout.contains(r#""name":"cosmic-wicara""#) || stdout.contains(r#""name":"cli""#),
        "cosmic-wicara binary target should be visible via cli package"
    );

    let toml = workspace_toml();
    assert!(
        toml.contains("resolver = \"3\""),
        "workspace resolver must be 3 in Cargo.toml"
    );
    assert!(
        toml.contains("rust-version = \"1.87\""),
        "rust-version must be 1.87 in Cargo.toml"
    );
    assert!(
        toml.contains("members") && toml.contains("crates/cli"),
        "workspace members must list crates/cli (+ core/app)"
    );
}

#[test]
fn main_returns_exit_code_stub() {
    let bin = env!("CARGO_BIN_EXE_cosmic-wicara");
    let output = Command::new(bin)
        .arg("--help")
        .output()
        .expect("run cosmic-wicara --help");
    assert!(
        output.status.success(),
        "scaffold binary should exit 0, got {:?}",
        output.status.code()
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("cosmic-wicara"),
        "stdout should contain help stub, got: {stdout}"
    );
}
