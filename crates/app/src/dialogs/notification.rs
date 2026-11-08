//! Notification dialog.

use wicara_core::dialog::Exit;
use wicara_core::notify;

use crate::window;

/// Notification icon.
pub const ICON_NAME: &str = "dialog-information";

/// Notification dialog state.
#[derive(Debug, Clone, Default)]
pub struct Notification {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Icon name.
    pub icon: String,
    /// Listen mode?
    pub listen: bool,
    /// Deprecated hint.
    pub hint: Option<String>,
    /// Preferred width.
    pub width: u32,
}

impl Notification {
    /// Create notification dialog.
    #[must_use]
    pub fn new(title: String, text: String) -> Self {
        Self {
            title,
            text,
            icon: ICON_NAME.to_owned(),
            ..Self::default()
        }
    }

    /// Set icon name.
    #[must_use]
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = icon;
        self
    }

    /// Set listen mode.
    #[must_use]
    pub fn with_listen(mut self, listen: bool) -> Self {
        self.listen = listen;
        self
    }

    /// Set deprecated hint.
    #[must_use]
    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Direct send?
    #[must_use]
    pub fn is_noop(&self) -> bool {
        !self.listen && self.title.trim().is_empty() && self.text.trim().is_empty()
    }

    /// Build send request.
    #[must_use]
    pub fn to_request(&self) -> notify::NotificationRequest {
        let icon = if self.icon.trim().is_empty() {
            ICON_NAME.to_owned()
        } else {
            self.icon.clone()
        };
        notify::NotificationRequest::new(self.title.clone(), self.text.clone()).with_icon(icon)
    }

    /// Send directly.
    pub fn send_direct(&self) -> Result<u32, notify::NotifyError> {
        notify::send_request(&self.to_request())
    }

    /// Run listen, injected reader.
    pub fn listen_with<R, F>(&self, reader: R, mut sender: F) -> Result<usize, notify::NotifyError>
    where
        R: std::io::BufRead,
        F: FnMut(&notify::NotificationRequest) -> Result<u32, notify::NotifyError>,
    {
        notify::run_listen_with(reader, &self.title, &mut sender)
    }
}

impl Notification {
    /// Run dialog, return outcome.
    pub fn run(&self) -> Exit {
        let _ = &self.hint;
        if self.listen {
            match notify::run_listen_stdin(&self.title) {
                Ok(_sent) => Exit::Ok,
                Err(err) => {
                    eprintln!("notification: {err} (exit {})", err.exit());
                    err.exit()
                }
            }
        } else if self.is_noop() {
            Exit::Ok
        } else {
            match self.send_direct() {
                Ok(_id) => Exit::Ok,
                Err(err) => {
                    eprintln!("notification: {err} (exit {})", err.exit());
                    err.exit()
                }
            }
        }
    }
}

/// No glass settings.
#[must_use]
pub fn window_settings(title: String, min_width: u32) -> cosmic::iced::window::Settings {
    window::solid_settings(&title, min_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_notification_empty_is_noop_ok() {
        let dialog = Notification::new(String::new(), String::new());
        assert!(dialog.is_noop());
        assert_eq!(dialog.run(), Exit::Ok);
    }

    #[test]
    fn smoke_notification_request_carries_title_text_icon() {
        let dialog = Notification::new("Update".to_owned(), "10 updates".to_owned())
            .with_icon("dialog-information".to_owned());
        let request = dialog.to_request();
        assert_eq!(request.title, "Update");
        assert_eq!(request.body, "10 updates");
        assert_eq!(request.icon, "dialog-information");
        assert!(!dialog.is_noop());
    }
}
