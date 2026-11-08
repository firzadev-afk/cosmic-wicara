//! Notification smoke tests.

use app::dialogs::about::{self, About};
use app::dialogs::notification::{self, Notification};
use app::window;
use wicara_core::dialog::Exit;

#[test]
fn smoke_about_builds_window() {
    let about = About::new();
    let (settings, app) = about.build_window();
    assert_eq!(app.title, "About cosmic-wicara");
    assert_eq!(app.ok_label, "OK");
    assert!(app.body.contains(about::VERSION));
    let expect_blur = window::blur_supported() && !window::prefers_reduced_transparency();
    assert_eq!(settings.blur, expect_blur);
    assert_eq!(settings.transparent, expect_blur);
    assert_eq!(about::ICON_NAME, "dialog-information");
}

#[test]
fn smoke_notification_empty_is_ok() {
    let dialog = Notification::new(String::new(), String::new());
    assert!(dialog.is_noop());
    assert_eq!(dialog.run(), Exit::Ok);
    assert_eq!(notification::ICON_NAME, "dialog-information");
}

#[test]
fn smoke_notification_listen_empty_is_ok() {
    use std::io::BufReader;
    let dialog = Notification::new("T".to_owned(), String::new()).with_listen(true);
    let mut called = 0usize;
    let sent = dialog
        .listen_with(BufReader::new(std::io::Cursor::new("")), |_| {
            called += 1;
            Ok(1)
        })
        .expect("empty listen must be Ok");
    assert_eq!(sent, 0);
    assert_eq!(called, 0);
}
