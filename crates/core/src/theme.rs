//! COSMIC theme and blur helpers.

use cosmic::Theme;
use cosmic::cosmic_theme::ThemeMode;
use cosmic::iced::window::Settings as WindowSettings;
use cosmic::theme::CosmicColor;

/// Request frosted glass windows.
pub const ENABLE_BLUR: bool = true;

/// Dialog content padding pixels.
pub const DIALOG_PADDING: u32 = 24;

/// Dark theme check.
#[must_use]
pub fn is_dark(theme: &Theme) -> bool {
    theme.theme_type.is_dark()
}

/// Active theme is dark.
#[must_use]
pub fn is_dark_active() -> bool {
    cosmic::theme::is_dark()
}

/// Background color for theme.
#[must_use]
pub fn bg_color(theme: &Theme) -> CosmicColor {
    theme.cosmic().bg_color()
}

/// Foreground color on background.
#[must_use]
pub fn on_bg_color(theme: &Theme) -> CosmicColor {
    theme.cosmic().on_bg_color()
}

/// Raised surface color.
#[must_use]
pub fn bg_component_color(theme: &Theme) -> CosmicColor {
    theme.cosmic().bg_component_color()
}

/// Button background color.
#[must_use]
pub fn button_bg_color(theme: &Theme) -> CosmicColor {
    theme.cosmic().button_bg_color()
}

/// Opaque background when blur off.
#[must_use]
pub fn solid_bg_color(theme: &Theme) -> CosmicColor {
    let mut c = bg_color(theme);
    c.alpha = 1.0;
    c
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

/// Compositor supports blur?
#[must_use]
pub fn blur_supported() -> bool {
    !(std::env::var("WAYLAND_DISPLAY").is_err() && std::env::var("DISPLAY").is_err())
}

/// Decide blur policy.
#[must_use]
pub fn ensure_blur(wanted: bool, supported: bool) -> bool {
    wanted && supported && !prefers_reduced_transparency()
}

/// Apply blur to settings.
pub fn apply_blur(settings: &mut WindowSettings) {
    let blur = ensure_blur(ENABLE_BLUR, blur_supported());
    settings.blur = blur;
    settings.transparent = blur;
}

/// Subscribe to COSMIC theme-mode changes.
pub fn subscribe(
    core: &cosmic::Core,
) -> cosmic::iced::Subscription<cosmic::cosmic_config::Update<ThemeMode>> {
    core.watch_config::<ThemeMode>(cosmic::cosmic_theme::THEME_MODE_ID)
}

/// Frosted-glass window settings.
#[must_use]
pub fn glass_window_settings(
    title: &str,
    width: u32,
    height: u32,
    min_width: u32,
) -> WindowSettings {
    let _ = title;
    let mut s = WindowSettings {
        size: cosmic::iced::Size::new(width.max(1) as f32, height.max(1) as f32),
        min_size: if min_width > 0 {
            Some(cosmic::iced::Size::new(
                min_width as f32,
                height.max(1) as f32,
            ))
        } else {
            None
        },
        resizable: true,
        decorations: true,
        closeable: true,
        ..Default::default()
    };
    apply_blur(&mut s);
    s
}

/// Window settings alias.
#[must_use]
pub fn window_settings(title: &str, width: u32, height: u32, min_width: u32) -> WindowSettings {
    glass_window_settings(title, width, height, min_width)
}

/// Build opaque solid window settings.
#[must_use]
pub fn solid_window_settings(
    title: &str,
    width: u32,
    height: u32,
    min_width: u32,
) -> WindowSettings {
    let _ = title;
    WindowSettings {
        size: cosmic::iced::Size::new(width.max(1) as f32, height.max(1) as f32),
        min_size: if min_width > 0 {
            Some(cosmic::iced::Size::new(
                min_width as f32,
                height.max(1) as f32,
            ))
        } else {
            None
        },
        resizable: true,
        blur: false,
        transparent: false,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dark_dark_vs_light() {
        assert!(is_dark(&Theme::dark()));
        assert!(!is_dark(&Theme::light()));
    }

    #[test]
    fn ensure_blur_policy() {
        if prefers_reduced_transparency() {
            assert!(!ensure_blur(true, true));
        } else {
            assert!(ensure_blur(true, true));
        }
        assert!(!ensure_blur(true, false));
        assert!(!ensure_blur(false, true));
        assert!(!ensure_blur(false, false));
    }

    #[test]
    fn glass_settings_request_blur_when_supported() {
        let s = glass_window_settings("t", 400, 200, 300);
        assert_eq!(s.blur, ensure_blur(true, blur_supported()));
        assert_eq!(s.transparent, s.blur);
        assert_eq!(s.size.width, 400.0);
        assert_eq!(s.size.height, 200.0);
        assert_eq!(s.min_size.unwrap().width, 300.0);
    }

    #[test]
    fn solid_settings_never_blur() {
        let s = solid_window_settings("t", 400, 200, 300);
        assert!(!s.blur);
        assert!(!s.transparent);
    }
}
