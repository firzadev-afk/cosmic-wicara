//! Blur detection and window settings.

use cosmic::iced::window::Settings;

use super::{App, DEFAULT_HEIGHT, DEFAULT_WIDTH};

/// Enable window blur.
pub fn enable_blur(settings: &mut Settings) {
    let wanted = true; // enable_blur(true)
    let supported = blur_supported();
    let reduced = prefers_reduced_transparency();
    let blur = wanted && supported && !reduced;
    settings.blur = blur;
    settings.transparent = blur;
}

/// Compositor supports blur?
#[must_use]
pub fn blur_supported() -> bool {
    !(std::env::var("WAYLAND_DISPLAY").is_err() && std::env::var("DISPLAY").is_err())
}

/// Reduced transparency requested?
#[must_use]
pub fn prefers_reduced_transparency() -> bool {
    std::env::var("COSMIC_WICARA_REDUCED_TRANSPARENCY")
        .or_else(|_| std::env::var("NO_TRANSPARENCY"))
        .map(|v| {
            matches!(
                v.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

/// Ellipsize long text to one line.
pub fn apply_ellipsize(text: &str) -> String {
    const MAX_CHARS: usize = 200;
    let flat: String = text
        .chars()
        .map(|c| if c == '\n' { ' ' } else { c })
        .collect();
    if flat.chars().count() <= MAX_CHARS {
        flat
    } else {
        let kept: String = flat.chars().take(MAX_CHARS - 3).collect();
        format!("{kept}...")
    }
}

/// Opaque fallback settings.
#[must_use]
pub fn solid_settings(title: &str, min_width: u32) -> Settings {
    let _ = title;
    Settings {
        size: cosmic::iced::Size::new(min_width.max(DEFAULT_WIDTH) as f32, DEFAULT_HEIGHT as f32),
        min_size: Some(cosmic::iced::Size::new(
            min_width.max(1) as f32,
            DEFAULT_HEIGHT as f32,
        )),
        resizable: true,
        blur: false,
        transparent: false,
        ..Default::default()
    }
}

/// Build window settings.
#[must_use]
pub fn build_window(title: String, min_width: u32) -> (Settings, App) {
    let width = min_width.max(1);
    let size = cosmic::iced::Size::new(width as f32, DEFAULT_HEIGHT as f32);
    let mut settings = Settings {
        size,
        min_size: Some(size),
        max_size: Some(size),
        resizable: false,
        exit_on_close_request: true,
        ..Default::default()
    };
    enable_blur(&mut settings);

    let mut app = App::new(title.clone(), String::new());
    app.min_width = width;
    app.title = title;
    (settings, app)
}
