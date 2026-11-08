//! Tray status.

use super::poll;
use std::time::Duration;

/// Poll interval.
pub const POLL_INTERVAL_SECS: u64 = 300;

/// Max popup lines.
pub const MAX_POPUP_LINES: usize = 20;

/// Theme config id.
#[must_use]
pub fn watched_config_id() -> &'static str {
    cosmic::cosmic_theme::THEME_MODE_ID
}

/// Tray icon name.
#[must_use]
pub fn tray_icon_name() -> &'static str {
    wicara_core::icons::UPDATE_ICON
}

/// Poll duration.
#[must_use]
pub fn poll_duration() -> Duration {
    Duration::from_secs(POLL_INTERVAL_SECS)
}

/// Subscribe theme changes.
pub fn subscribe_theme_mode(
    core: &cosmic::Core,
) -> cosmic::iced::Subscription<cosmic::cosmic_config::Update<cosmic::cosmic_theme::ThemeMode>> {
    core.watch_config::<cosmic::cosmic_theme::ThemeMode>(watched_config_id())
}

/// Tray status.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TrayStatus {
    /// No updates.
    #[default]
    Idle,
    /// Pending updates.
    Updates {
        /// Pending count.
        count: usize,
        /// Update lines.
        lines: Vec<String>,
    },
    /// Last poll failed.
    Error {
        /// Error message.
        message: String,
    },
}

impl TrayStatus {
    /// Summarize poll result.
    #[must_use]
    pub fn from_pending(pending: &[poll::PendingUpdate]) -> Self {
        if pending.is_empty() {
            Self::Idle
        } else {
            Self::Updates {
                count: pending.len(),
                lines: pending.iter().map(|item| item.line.clone()).collect(),
            }
        }
    }

    /// Create error status.
    #[must_use]
    pub fn error(message: String) -> Self {
        Self::Error { message }
    }

    /// Has updates?
    #[must_use]
    pub const fn has_updates(&self) -> bool {
        matches!(self, Self::Updates { .. })
    }

    /// Pending count.
    #[must_use]
    pub const fn update_count(&self) -> usize {
        match self {
            Self::Updates { count, .. } => *count,
            Self::Idle | Self::Error { .. } => 0,
        }
    }

    /// Tray tooltip.
    #[must_use]
    pub fn tooltip(&self) -> String {
        match self {
            Self::Idle => "cosmic-wicara: no updates".to_owned(),
            Self::Updates { count, .. } => format!("cosmic-wicara: {count} updates available"),
            Self::Error { message } => format!("cosmic-wicara: update check failed: {message}"),
        }
    }

    /// Popup lines.
    #[must_use]
    pub fn popup_lines(&self) -> Vec<String> {
        match self {
            Self::Idle => vec!["No updates".to_owned()],
            Self::Updates { lines, .. } => lines.iter().take(MAX_POPUP_LINES).cloned().collect(),
            Self::Error { message } => vec![format!("Check failed: {message}")],
        }
    }
}
