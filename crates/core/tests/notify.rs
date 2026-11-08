//! Notification listen tests.

use std::io::BufReader;

use core::notify::{
    APP_NAME, BUS_NAME, DEFAULT_ICON, INTERFACE, NotificationRequest, OBJECT_PATH,
    collect_listen_requests, parse_listen_line, run_listen_with, send, send_request,
};

#[test]
fn notify_empty_stdin_is_ok_noop() {
    let mut called = 0usize;
    let sent = run_listen_with(
        BufReader::new(std::io::Cursor::new(String::new())),
        "Title",
        |_| {
            called += 1;
            Ok(99)
        },
    );
    assert_eq!(sent, Ok(0));
    assert_eq!(called, 0);

    assert!(collect_listen_requests("Title", "  \n\t\n").is_empty());
    assert!(collect_listen_requests("Title", "").is_empty());
}

#[test]
fn notify_blank_send_is_ok_noop_without_bus() {
    assert_eq!(send("", ""), Ok(0));
    assert_eq!(send_request(&NotificationRequest::new("", "")), Ok(0));
    assert_eq!(send_request(&NotificationRequest::new(" ", " \n ")), Ok(0));
}

#[test]
fn notify_bus_names_match_freedesktop_spec() {
    assert_eq!(BUS_NAME, "org.freedesktop.Notifications");
    assert_eq!(INTERFACE, "org.freedesktop.Notifications");
    assert_eq!(OBJECT_PATH, "/org/freedesktop/Notifications");
    assert_eq!(DEFAULT_ICON, "dialog-information");
    assert_eq!(APP_NAME, "cosmic-wicara");
}

#[test]
fn notify_listen_parses_and_collects() {
    assert_eq!(
        parse_listen_line("10 updates available"),
        Some(core::notify::ListenCommand::Text(
            "10 updates available".to_owned()
        ))
    );
    let requests = collect_listen_requests(
        "Update",
        "icon=system-software-update\n10 updates available\n",
    );
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].icon, "system-software-update");
    assert_eq!(requests[0].summary(), "Update");

    let mut seen = Vec::new();
    let sent = run_listen_with(BufReader::new(std::io::Cursor::new("a\nb\n")), "T", |req| {
        seen.push(req.body.clone());
        Ok(1)
    });
    assert_eq!(sent, Ok(2));
    assert_eq!(seen, vec!["a".to_owned(), "b".to_owned()]);
}
