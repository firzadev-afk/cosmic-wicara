//! Window shell tests.

use app::window::{
    blur_supported, build_window, enable_blur, prefers_reduced_transparency, solid_settings,
};

#[test]
fn settings_contain_enable_blur_policy() {
    let (settings, app) = build_window("Question".to_owned(), 400);
    let expect_blur = blur_supported() && !prefers_reduced_transparency();
    assert_eq!(
        settings.blur, expect_blur,
        "glass must follow enable_blur(true) policy"
    );
    assert_eq!(settings.transparent, expect_blur);
    assert_eq!(settings.size.width, 400.0);
    assert_eq!(settings.min_size.unwrap().width, 400.0);
    assert_eq!(app.title, "Question");
    assert_eq!(app.ok_label, "OK");
}

#[test]
fn build_window_min_width_floor() {
    let (settings, _) = build_window("T".to_owned(), 300);
    assert_eq!(settings.min_size.unwrap().width, 300.0);
}

#[test]
fn solid_fallback_never_blurs() {
    let s = solid_settings("T", 400);
    assert!(!s.blur);
    assert!(!s.transparent);
}

#[test]
fn enable_blur_helper_matches_build_window() {
    let mut s = cosmic::iced::window::Settings::default();
    enable_blur(&mut s);
    assert_eq!(s.blur, blur_supported() && !prefers_reduced_transparency());
    assert_eq!(s.transparent, s.blur);
}
