//! Desktop notifications via D-Bus.

pub mod bus;
pub mod listen;

pub use listen::*;

/// Session-bus destination name.
pub const BUS_NAME: &str = "org.freedesktop.Notifications";

/// Notification object path.
pub const OBJECT_PATH: &str = "/org/freedesktop/Notifications";

/// Notification interface name.
pub const INTERFACE: &str = "org.freedesktop.Notifications";

/// Notify method name.
pub const METHOD_NOTIFY: &str = "Notify";

/// app_name sent on every notification.
pub const APP_NAME: &str = "cosmic-wicara";

/// Default notification icon.
pub const DEFAULT_ICON: &str = "dialog-information";

/// Default expire timeout.
pub const DEFAULT_TIMEOUT_MS: i32 = -1;

/// Error from sending a notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotifyError {
    /// Bus or server failure.
    Db(String),
    /// Listen stdin failure.
    Io(String),
}

use crate::dialog::Exit;

impl std::fmt::Display for NotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db(msg) => write!(f, "D-Bus notification failed: {msg}"),
            Self::Io(msg) => write!(f, "notification stdin: {msg}"),
        }
    }
}

impl std::error::Error for NotifyError {}

impl NotifyError {
    /// Exit code 255.
    #[must_use]
    pub fn exit(&self) -> Exit {
        Exit::Err255
    }
}

/// One notification to send.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationRequest {
    /// Summary, fallback body.
    pub title: String,
    /// Body text.
    pub body: String,
    /// Icon name --icon.
    pub icon: String,
    /// Expire timeout milliseconds.
    pub expire_timeout_ms: i32,
}

impl NotificationRequest {
    /// Create request defaults.
    #[must_use]
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            icon: DEFAULT_ICON.to_owned(),
            expire_timeout_ms: DEFAULT_TIMEOUT_MS,
        }
    }

    /// Set icon name.
    #[must_use]
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = icon.into();
        self
    }

    /// Set expire timeout.
    #[must_use]
    pub fn with_timeout_ms(mut self, timeout_ms: i32) -> Self {
        self.expire_timeout_ms = timeout_ms;
        self
    }

    /// Set expire timeout.
    #[must_use]
    pub fn with_timeout_secs(self, timeout_secs: u64) -> Self {
        let ms = i32::try_from(timeout_secs.saturating_mul(1000)).unwrap_or(i32::MAX);
        self.with_timeout_ms(ms)
    }

    /// Both fields empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.title.trim().is_empty() && self.body.trim().is_empty()
    }

    /// Server summary text.
    #[must_use]
    pub fn summary(&self) -> &str {
        if self.title.trim().is_empty() {
            &self.body
        } else {
            &self.title
        }
    }
}

/// Send one notification.
pub fn send(title: &str, body: &str) -> Result<u32, NotifyError> {
    send_request(&NotificationRequest::new(title, body))
}

/// Send full notification request.
pub fn send_request(request: &NotificationRequest) -> Result<u32, NotifyError> {
    if request.is_empty() {
        return Ok(0);
    }
    let icon = if request.icon.trim().is_empty() {
        DEFAULT_ICON
    } else {
        request.icon.as_str()
    };
    zbus::block_on(bus::notify_async(
        APP_NAME,
        icon,
        request.summary(),
        &request.body,
        request.expire_timeout_ms,
    ))
}
