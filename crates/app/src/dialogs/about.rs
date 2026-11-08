//! About dialog.

use cosmic::iced::window::Settings;
use wicara_core::theme;

use crate::window;

/// About icon.
pub const ICON_NAME: &str = "dialog-information";

/// Application name.
pub const APP_NAME: &str = "cosmic-wicara";

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Pinned libcosmic revision.
pub const LIBCOSMIC_REV: &str = "dd108aa99d7306110eef06da43eafc109e1ab151";

/// License line.
pub const LICENSE: &str = "GPL-2.0-or-later";

/// About dialog state.
#[derive(Debug, Clone)]
pub struct About {
    /// Window title.
    pub title: String,
    /// Preferred width.
    pub width: u32,
    /// Version override.
    pub version: String,
}

impl Default for About {
    fn default() -> Self {
        Self {
            title: String::new(),
            width: 400,
            version: String::new(),
        }
    }
}

/// About dialog config.
#[allow(clippy::new_without_default)]
impl About {
    /// Create about dialog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set title.
    #[must_use]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Effective version.
    #[must_use]
    pub fn version(&self) -> &str {
        if self.version.is_empty() {
            VERSION
        } else {
            &self.version
        }
    }

    /// Effective title.
    #[must_use]
    pub fn window_title(&self) -> String {
        if self.title.trim().is_empty() {
            format!("About {APP_NAME}")
        } else {
            self.title.clone()
        }
    }

    /// About body text.
    #[must_use]
    pub fn about_text(&self) -> String {
        let version = self.version();
        format!(
            "{APP_NAME} {version}\n\
             Native COSMIC dialog tool (Rust + libcosmic, COSMIC-only).\n\
             libcosmic rev {LIBCOSMIC_REV}\n\
             Icon: {ICON_NAME}\n\
             License: {LICENSE}"
        )
    }

    /// Dark theme?
    #[must_use]
    pub fn is_dark_active() -> bool {
        theme::is_dark_active()
    }

    /// Build window settings.
    #[must_use]
    pub fn build_window(&self) -> (Settings, window::App) {
        let (mut settings, mut app) = window::build_window(self.window_title(), self.width);
        window::enable_blur(&mut settings);
        app.body = self.about_text();
        app.ok_label = "OK".to_owned();
        app.cancel_label = "Close".to_owned();
        (settings, app)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_about_text_contains_version_and_icon() {
        let about = About::new();
        let text = about.about_text();
        assert!(text.contains(APP_NAME));
        assert!(text.contains(VERSION));
        assert!(text.contains(LIBCOSMIC_REV));
        assert!(text.contains(ICON_NAME));
        assert_eq!(about.window_title(), "About cosmic-wicara");
    }

    #[test]
    fn smoke_about_builds_glass_window() {
        let about = About::new().with_title("About".to_owned());
        let (settings, app) = about.build_window();
        assert_eq!(app.title, "About");
        assert_eq!(app.ok_label, "OK");
        assert_eq!(app.cancel_label, "Close");
        assert!(app.body.contains(APP_NAME));
        assert_eq!(
            settings.blur,
            window::blur_supported() && !window::prefers_reduced_transparency()
        );
        assert_eq!(settings.transparent, settings.blur);
    }

    #[test]
    fn smoke_about_text_never_claims_ok() {
        assert!(About::new().about_text().contains("cosmic-wicara"));
    }
}
