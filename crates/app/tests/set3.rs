//! Dialog error-mapping tests.

use std::io::Cursor;
use std::path::PathBuf;
use std::time::Duration;

use app::dialogs::file_selection::{FileSelection, check_dir_readable};
use app::dialogs::forms::{Forms, split_values};
use app::dialogs::progress::{Progress, ProgressState};
use wicara_core::dialog::Exit;

/// Unique temp dir.
fn temp_dir(tag: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let uniq = format!(
        "cosmic-wicara-set3-{}-{}",
        tag,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock ok")
            .as_nanos()
    );
    dir.push(uniq);
    dir
}

/// Set Unix mode.
#[cfg(unix)]
fn chmod(path: &std::path::Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).expect("chmod ok");
}

#[test]
fn set3_unreadable_dir_maps_to_255() {
    let dir = temp_dir("noaccess");
    std::fs::create_dir_all(&dir).expect("create temp dir");
    #[cfg(unix)]
    chmod(&dir, 0o000);

    assert!(check_dir_readable(&dir).is_err());
    let sel = FileSelection {
        title: "Pick".to_owned(),
        filename: Some(dir.join("file.txt")),
        multiple: false,
        directory: false,
        save: false,
        separator: None,
        file_filter: Vec::new(),
    };
    assert!(sel.check_start_dir().is_err());
    assert!(sel.check_start_dir().is_err());
    assert_eq!(Exit::Err255.into_code(), 255);

    #[cfg(unix)]
    chmod(&dir, 0o755);
    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn set3_readable_dir_is_browsable() {
    let dir = temp_dir("readable");
    std::fs::create_dir_all(&dir).expect("create temp dir");
    assert!(check_dir_readable(&dir).is_ok());
    let sel = FileSelection {
        title: "Pick".to_owned(),
        filename: Some(dir.join("file.txt")),
        multiple: true,
        directory: true,
        save: false,
        separator: Some("|".to_owned()),
        file_filter: vec!["*.txt".to_owned()],
    };
    assert!(sel.check_start_dir().is_ok());
    assert_eq!(sel.effective_separator(), "|");
    assert_eq!(
        sel.format_output(&[PathBuf::from("/a"), PathBuf::from("/b")]),
        "/a|/b"
    );
    assert!(sel.matches_filter(std::path::Path::new("/a/notes.txt")));
    assert!(!sel.matches_filter(std::path::Path::new("/a/notes.md")));
    assert!(sel.check_start_dir().is_ok());

    std::fs::remove_dir_all(&dir).expect("cleanup");
}

#[test]
fn set3_progress_closed_stdin_maps_to_1_not_hang() {
    let progress = Progress::new("Working".to_owned(), "Starting".to_owned());
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = progress.clone();
    std::thread::spawn(move || {
        let _ = tx.send(worker.show_with_reader(Cursor::new(Vec::<u8>::new())));
    });
    let exit = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("progress loop must not hang on closed stdin");
    assert_eq!(exit, Exit::Cancel);
    assert_eq!(exit.into_code(), 1);
    assert!(!progress.is_indeterminate());
}

#[test]
fn set3_progress_lines_update_state_and_complete() {
    let progress = Progress::new("T".to_owned(), " init".to_owned());
    let auto = Progress {
        auto_close: true,
        ..progress.clone()
    };
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = auto.clone();
    let input = "25\n# Halfway\n100\n".as_bytes().to_owned();
    std::thread::spawn(move || {
        let _ = tx.send(worker.show_with_reader(Cursor::new(input)));
    });
    let exit = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("auto-close loop must finish");
    assert_eq!(exit, Exit::Ok);

    let mut state = ProgressState::new(None, "init".to_owned());
    assert_eq!(
        state.apply_line("# Almost"),
        app::dialogs::progress::ProgressEvent::Text
    );
    assert_eq!(state.text, "Almost");
    assert_eq!(
        state.apply_line("150"),
        app::dialogs::progress::ProgressEvent::Complete
    );
    assert!(state.is_complete());
}

#[test]
fn set3_entry_password_forms_outputs() {
    use app::dialogs::entry::Entry;
    use app::dialogs::password::Password;

    let mut entry = Entry::new("T".to_owned(), "Name:".to_owned(), "x".to_owned(), true);
    entry.set_value("hello".to_owned());
    assert_eq!(entry.display_text(), "•••••");
    assert_eq!(entry.output(), "hello");

    let mut pw = Password::new("T".to_owned(), "Password:".to_owned(), true);
    pw.set_username("alice".to_owned());
    pw.set_secret("s3cret".to_owned());
    assert_eq!(pw.output(), "alice|s3cret");
    let mut bare = Password::new("T".to_owned(), String::new(), false);
    bare.set_secret("hunter2".to_owned());
    assert_eq!(bare.output(), "hunter2");

    assert_eq!(split_values("a|b|c"), ["a", "b", "c"]);
    let mut forms = Forms::new("T".to_owned(), "Hi".to_owned());
    forms.add_entry("Name".to_owned());
    forms.add_password("Pass".to_owned());
    forms.add_multiline_entry("Bio".to_owned());
    forms.add_calendar("Day".to_owned());
    forms.add_list("Files".to_owned(), "/a|/b", "File");
    forms.add_combo("OS".to_owned(), "Ubuntu|Fedora");
    let out = forms.output();
    assert!(out.contains("Name="));
    assert!(out.contains("OS=Ubuntu"));
}
