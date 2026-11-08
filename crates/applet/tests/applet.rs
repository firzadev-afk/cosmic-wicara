//! Tray applet tests.

#[allow(dead_code)]
#[path = "../src/main.rs"]
mod applet_under_test;

use applet_under_test::{poll, tray};

/// Three-update fixture.
const THREE_UPDATES: &str = "bash 5.2-1 -> 5.2-2\n\
                             linux 6.11.1.arch2-1 -> 6.11.2.arch1-1\n\
                             zsh 5.9-1 -> 5.9-2\n";

#[test]
fn applet_status_updates_and_idle() {
    assert_eq!(tray::TrayStatus::from_pending(&[]), tray::TrayStatus::Idle);
    assert!(!tray::TrayStatus::Idle.has_updates());
    assert_eq!(tray::TrayStatus::Idle.update_count(), 0);
    assert_eq!(
        tray::TrayStatus::Idle.tooltip(),
        "cosmic-wicara: no updates".to_owned()
    );
    assert_eq!(
        tray::TrayStatus::Idle.popup_lines(),
        vec!["No updates".to_owned()]
    );

    let pending = poll::parse_checkupdates_output(THREE_UPDATES);
    assert_eq!(pending.len(), 3);
    let status = tray::TrayStatus::from_pending(&pending);
    assert_eq!(
        status,
        tray::TrayStatus::Updates {
            count: 3,
            lines: vec![
                "bash 5.2-1 -> 5.2-2".to_owned(),
                "linux 6.11.1.arch2-1 -> 6.11.2.arch1-1".to_owned(),
                "zsh 5.9-1 -> 5.9-2".to_owned(),
            ],
        }
    );
    assert!(status.has_updates());
    assert_eq!(status.update_count(), 3);
    assert_eq!(
        status.tooltip(),
        "cosmic-wicara: 3 updates available".to_owned()
    );

    let failed = tray::TrayStatus::error("checkupdates: No such file or directory".to_owned());
    assert!(!failed.has_updates());
    assert_eq!(failed.update_count(), 0);
    assert!(
        failed.tooltip().contains("update check failed"),
        "error tooltip must name the failure: {:?}",
        failed.tooltip()
    );
    assert_eq!(
        failed.popup_lines(),
        vec!["Check failed: checkupdates: No such file or directory".to_owned()]
    );
}

#[test]
fn applet_parse_checkupdates_output() {
    assert_eq!(poll::parse_checkupdates_output("").len(), 0);
    assert!(poll::parse_checkupdates_output("  \n\n  ").is_empty());
    assert_eq!(poll::count_pending(""), 0);

    let pending = poll::parse_checkupdates_output(THREE_UPDATES);
    assert_eq!(pending.len(), 3);
    assert_eq!(pending[0].package, "bash");
    assert_eq!(pending[0].line, "bash 5.2-1 -> 5.2-2");
    assert_eq!(pending[2].package, "zsh");

    let messy = poll::parse_checkupdates_output("\n  bash 5.2-1 -> 5.2-2  \n\nok 1 -> 2\n");
    assert_eq!(messy.len(), 2);
    assert_eq!(messy[0].package, "bash");
    assert_eq!(messy[0].line, "bash 5.2-1 -> 5.2-2");
    assert_eq!(messy[1].package, "ok");

    assert_eq!(
        poll::parse_checkupdates_output("bash 1 -> 2\r\nzsh 3 -> 4\r\n").len(),
        2
    );
    assert_eq!(poll::count_pending(THREE_UPDATES), 3);

    assert_eq!(poll::CHECKUPDATES_BIN, "checkupdates");
    if std::env::var(poll::CHECKUPDATES_ENV).is_err() {
        assert_eq!(poll::checker_command(), "checkupdates");
    }
}

#[test]
fn applet_poll_contract() {
    assert_eq!(
        tray::poll_duration(),
        std::time::Duration::from_secs(tray::POLL_INTERVAL_SECS)
    );
    assert_eq!(tray::poll_duration().as_secs(), 300);

    assert_eq!(tray::tray_icon_name(), wicara_core::icons::UPDATE_ICON);
    assert!(tray::tray_icon_name().ends_with("-symbolic"));
    assert_eq!(tray::tray_icon_name(), "system-software-update-symbolic");

    assert_eq!(applet_under_test::APP_ID, "com.cosmicwicara.Applet");
}

#[test]
fn applet_theme_watch_contract() {
    assert_eq!(
        tray::watched_config_id(),
        cosmic::cosmic_theme::THEME_MODE_ID
    );
    assert_eq!(tray::watched_config_id(), "com.system76.CosmicTheme.Mode");

    let _: cosmic::cosmic_config::Update<cosmic::cosmic_theme::ThemeMode> =
        cosmic::cosmic_config::Update {
            errors: Vec::new(),
            keys: Vec::new(),
            config: cosmic::cosmic_theme::ThemeMode::default(),
        };
}

#[test]
fn applet_subscription_wires_watch_config_and_poll() {
    let source = format!(
        "{}{}",
        include_str!("../src/main.rs"),
        include_str!("../src/tray.rs")
    );
    let source = format!("{}{}", source, include_str!("../src/poll.rs"));

    assert!(
        source.contains("cosmic::applet::run"),
        "binary must launch via cosmic::applet::run"
    );
    assert!(
        source.contains("cosmic::Application for"),
        "tray must implement cosmic::Application"
    );

    assert!(
        source.contains("core.watch_config"),
        "must use core.watch_config"
    );
    assert!(source.contains("THEME_MODE_ID"), "must watch THEME_MODE_ID");
    assert!(
        source.contains("subscribe_theme_mode"),
        "theme subscription helper must exist"
    );
    assert!(
        source.contains("PollTick"),
        "poll Message variant must exist"
    );
    assert!(
        source.contains("time::every"),
        "subscription must poll on a timer"
    );
    assert!(
        source.contains("POLL_INTERVAL_SECS") || source.contains("poll_duration"),
        "poll cadence must come from tray::POLL_INTERVAL_SECS"
    );

    assert!(
        source.contains("checkupdates"),
        "poll must shell out to checkupdates"
    );
    assert!(
        source.contains("ICON") || source.contains("icons::update"),
        "tray icon must come from wicara_core::icons (system-software-update)"
    );
    assert!(
        source.contains("cosmic::applet::style"),
        "applet style must come from cosmic::applet::style"
    );
}

#[test]
fn applet_manifest_declares_binary() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        manifest.contains("cosmic-applet-wicara"),
        "manifest must name the cosmic-applet-wicara binary"
    );
    assert!(
        manifest.contains("[[bin]]"),
        "manifest must declare a [[bin]] target"
    );
    assert!(
        manifest.contains("\"applet\""),
        "libcosmic must be built with the applet feature"
    );
    assert!(
        manifest.contains("wicara_core"),
        "applet must depend on the shared core lib"
    );
}
