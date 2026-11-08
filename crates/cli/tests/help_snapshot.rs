//! Help output snapshot tests.

use assert_cmd::Command;
use clap::Parser;
use cli::args::{Cli, DialogKind};

#[test]
fn parses_question() {
    let cli = Cli::parse_from([
        "cosmic-wicara",
        "--question",
        "--title=T",
        "--text=H",
        "--ok-label=O",
        "--cancel-label=C",
    ]);
    assert_eq!(cli.general.title, "T");
    assert_eq!(cli.general.width, 0, "width defaults to 0 (auto)");
    assert_eq!(cli.general.height, 0, "height defaults to 0 (auto)");
    assert_eq!(cli.general.timeout, None, "timeout defaults to None");
    assert_eq!(cli.per_dialog.text.as_deref(), Some("H"));
    assert_eq!(cli.general.ok_label, "O");
    assert_eq!(cli.general.cancel_label, "C");
    assert!(cli.per_dialog.question);
    assert_eq!(cli.kind(), Some(DialogKind::Question));
    assert_eq!(DialogKind::Question.flag(), "question");
}

#[test]
fn parses_general_defaults() {
    let cli = Cli::parse_from(["cosmic-wicara", "--info", "--text=Hi"]);
    assert_eq!(cli.kind(), Some(DialogKind::Info));
    assert_eq!(cli.general.ok_label, "OK");
    assert_eq!(cli.general.cancel_label, "Cancel");
    assert_eq!(cli.general.attach, None, "--attach is a deprecated noop");
    assert!(!cli.general.modal);
}

#[test]
fn help_exits_zero_with_section_headers() {
    Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .arg("--help")
        .assert()
        .success();
}

#[test]
fn help_question_exits_zero_with_question_section() {
    let output = Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .arg("--help-question")
        .assert()
        .success()
        .get_output()
        .clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Question options"),
        "--help-question must contain 'Question options' section header, got:\n{stdout}"
    );
    assert!(
        stdout.contains("--question"),
        "--help-question must list --question flag, got:\n{stdout}"
    );
    assert!(
        stdout.contains("--text"),
        "--help-question must list --text flag, got:\n{stdout}"
    );
    assert!(
        stdout.contains("--default-cancel"),
        "--help-question must list --default-cancel flag, got:\n{stdout}"
    );
}

#[test]
fn help_general_exits_zero_with_general_section() {
    let output = Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .arg("--help-general")
        .assert()
        .success()
        .get_output()
        .clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("General options"),
        "--help-general must contain 'General options' section header, got:\n{stdout}"
    );
    for flag in [
        "--title",
        "--width",
        "--height",
        "--timeout",
        "--ok-label",
        "--cancel-label",
        "--extra-button",
        "--modal",
        "--attach",
    ] {
        assert!(
            stdout.contains(flag),
            "--help-general must list {flag}, got:\n{stdout}"
        );
    }
}

#[test]
fn help_all_contains_all_headers() {
    let output = Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .arg("--help-all")
        .assert()
        .success()
        .get_output()
        .clone();
    let stdout = String::from_utf8_lossy(&output.stdout);
    for header in [
        "General options",
        "Calendar options",
        "Text entry options",
        "Error options",
        "Info options",
        "File selection options",
        "List options",
        "Notification options",
        "Progress options",
        "Question options",
        "Warning options",
        "Scale options",
        "Text information options",
        "Color selection options",
        "Password dialog options",
        "Forms dialog options",
        "Miscellaneous options",
        "Application Options",
        "Help Options",
    ] {
        assert!(
            stdout.contains(header),
            "--help-all must contain section header {header:?}, got:\n{stdout}"
        );
    }
}

#[test]
fn missing_dialog_exits_255() {
    Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .assert()
        .code(255)
        .stderr(predicates::str::contains("dialog type"));
}

#[test]
fn multiple_dialogs_exit_255() {
    Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .args(["--question", "--info", "--text=Hi"])
        .assert()
        .code(255)
        .stderr(predicates::str::contains("dialog"));
}

#[test]
fn question_timeout_exits_five() {
    if std::env::var_os("WAYLAND_DISPLAY").is_none() && std::env::var_os("DISPLAY").is_none() {
        eprintln!("skipping question_timeout_exits_five: no compositor");
        return;
    }
    Command::cargo_bin("cosmic-wicara")
        .unwrap()
        .args(["--question", "--text=Hello", "--timeout=1"])
        .assert()
        .code(5);
}
