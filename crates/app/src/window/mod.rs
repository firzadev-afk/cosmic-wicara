//! Shared window shell.

use cosmic::Element;
use cosmic::widget::{button, dialog};
use wicara_core::dialog::Exit;

pub mod blur;
pub mod geometry;

pub use blur::{
    apply_ellipsize, blur_supported, build_window, enable_blur, prefers_reduced_transparency,
    solid_settings,
};

/// Content padding pixels.
pub const DIALOG_PADDING: f32 = 24.0;

/// Default dialog size.
pub const DEFAULT_WIDTH: u32 = 400;
pub const DEFAULT_HEIGHT: u32 = 220;

/// Default button labels.
pub const DEFAULT_OK_LABEL: &str = "OK";
pub const DEFAULT_CANCEL_LABEL: &str = "Cancel";

/// Shell messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Message {
    /// Close requested.
    Close,
    /// OK pressed.
    Ok,
    /// Cancel pressed.
    Cancel,
    /// Timeout expired.
    Timeout,
    /// Countdown tick.
    Tick,
    /// Header dragged (stub).
    Drag,
    /// Gained focus.
    Focused,
    /// Lost focus.
    Unfocused,
}

/// Shared shell state.
#[derive(Clone, Debug)]
pub struct App {
    /// Window title.
    pub title: String,
    /// Body text.
    pub body: String,
    /// OK button label.
    pub ok_label: String,
    /// Cancel button label.
    pub cancel_label: String,
    /// Minimum window width.
    pub min_width: u32,
    /// Has focus?
    pub focused: bool,
    /// Activation token?
    pub activated: bool,
    /// Timeout seconds.
    pub timeout: Option<u64>,
    /// Seconds remaining.
    pub remaining: Option<u64>,
    /// Exit code set?
    pub exit_code: Option<u8>,
    /// Icon name.
    pub icon: Option<String>,
    /// Ellipsize enabled?
    pub ellipsize: bool,
    /// No-wrap enabled?
    pub no_wrap: bool,
}

impl App {
    /// Create shell defaults.
    #[must_use]
    pub fn new(title: String, body: String) -> Self {
        Self {
            title,
            body,
            ok_label: DEFAULT_OK_LABEL.to_owned(),
            cancel_label: DEFAULT_CANCEL_LABEL.to_owned(),
            min_width: DEFAULT_WIDTH,
            focused: false,
            activated: false,
            timeout: None,
            remaining: None,
            exit_code: None,
            icon: None,
            ellipsize: false,
            no_wrap: false,
        }
    }

    /// Create shell options.
    #[must_use]
    pub fn with_options(
        title: String,
        body: String,
        ok_label: String,
        cancel_label: String,
        timeout: Option<u64>,
    ) -> Self {
        let mut app = Self::new(title, body);
        app.ok_label = ok_label;
        app.cancel_label = cancel_label;
        app.timeout = timeout;
        app.remaining = timeout;
        app
    }

    /// Mark focused.
    pub fn on_focused(&mut self) {
        self.focused = true;
        self.activated = true;
    }

    /// Mark unfocused.
    pub fn on_unfocused(&mut self) {
        self.focused = false;
    }

    /// Record activation token.
    pub fn on_activate(&mut self) {
        self.activated = true;
    }

    /// Tick countdown.
    pub fn tick(&mut self) -> Option<u8> {
        if self.exit_code.is_some() {
            return None;
        }
        match self.remaining {
            Some(0) => {
                let code = Exit::Timeout.into_code();
                self.exit_code = Some(code);
                Some(code)
            }
            Some(n) => {
                let next = n.saturating_sub(1);
                self.remaining = Some(next);
                if next == 0 {
                    let code = Exit::Timeout.into_code();
                    self.exit_code = Some(code);
                    Some(code)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Handle shell message.
    pub fn update(&mut self, message: Message) -> Option<u8> {
        match message {
            Message::Close => {
                let code = Exit::Cancel.into_code();
                self.exit_code = Some(code);
                Some(code)
            }
            Message::Ok => {
                let code = Exit::Ok.into_code();
                self.exit_code = Some(code);
                Some(code)
            }
            Message::Cancel => {
                let code = Exit::Cancel.into_code();
                self.exit_code = Some(code);
                Some(code)
            }
            Message::Timeout => {
                let code = Exit::Timeout.into_code();
                self.exit_code = Some(code);
                Some(code)
            }
            Message::Tick => self.tick(),
            Message::Drag => None,
            Message::Focused => {
                self.on_focused();
                None
            }
            Message::Unfocused => {
                self.on_unfocused();
                None
            }
        }
    }

    /// Render dialog surface.
    pub fn view(&self) -> Element<'_, Message> {
        let body = self.body_with_countdown();

        let primary = button::suggested(self.ok_label.clone()).on_press(Message::Ok);
        let secondary = button::standard(self.cancel_label.clone()).on_press(Message::Cancel);

        let widget = dialog()
            .title(self.title.clone())
            .body(body)
            .primary_action(primary)
            .secondary_action(secondary)
            .is_overlay(false)
            .width(cosmic::iced::Length::Fill);
        if let Some(name) = &self.icon {
            let named =
                wicara_core::icons::for_override(name, wicara_core::icons::DialogIcon::Information);
            widget.icon(named.icon())
        } else {
            widget
        }
        .into()
    }

    /// Preferred window size.
    #[must_use]
    pub fn preferred_size(&self, width: u32, height: u32) -> (u32, u32) {
        let width = if width == 0 { DEFAULT_WIDTH } else { width };
        let height = if height > 0 {
            height
        } else {
            self.auto_height(width)
        };
        (width, height.max(1))
    }

    /// Height for content.
    #[must_use]
    pub fn auto_height(&self, width: u32) -> u32 {
        use geometry::{
            BODY_LINE_HEIGHT, BODY_MAX_HEIGHT, BODY_SLACK, BUTTON_GAP, BUTTON_ROW_HEIGHT,
            CARD_PADDING, TITLE_GAP, TITLE_HEIGHT,
        };
        let lines = if self.no_wrap {
            self.body.lines().count().max(1)
        } else {
            geometry::wrapped_lines(&self.body_with_countdown(), width)
        };
        let body = (lines as f32 * BODY_LINE_HEIGHT).min(BODY_MAX_HEIGHT);
        let title = if self.title.is_empty() {
            0.0
        } else {
            TITLE_HEIGHT + TITLE_GAP
        };
        let chrome = CARD_PADDING * 2.0 + title + BUTTON_GAP + BUTTON_ROW_HEIGHT + BODY_SLACK;
        (chrome + body).ceil() as u32
    }

    /// Body with countdown.
    pub(crate) fn body_with_countdown(&self) -> String {
        let text = if self.ellipsize {
            apply_ellipsize(&self.body)
        } else {
            self.body.clone()
        };
        match self.remaining {
            Some(secs) if self.exit_code.is_none() => {
                if text.is_empty() {
                    format!("(closes in {secs}s)")
                } else {
                    format!("{}\n\n(closes in {secs}s)", text)
                }
            }
            _ => text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_window_requests_blur() {
        let (settings, app) = build_window("Hello".to_owned(), 400);
        if blur_supported() && !prefers_reduced_transparency() {
            assert!(settings.blur, "glass settings must request blur");
            assert!(settings.transparent);
        } else {
            assert!(!settings.blur);
            assert!(!settings.transparent);
        }
        assert_eq!(settings.size.width, 400.0);
        assert_eq!(settings.min_size.unwrap().width, 400.0);
        assert_eq!(app.title, "Hello");
        assert_eq!(app.ok_label, DEFAULT_OK_LABEL);
    }

    #[test]
    fn message_routing_maps_to_codes() {
        let mut app = App::new("t".to_owned(), "b".to_owned());
        assert_eq!(app.update(Message::Ok), Some(0));
        let mut app = App::new("t".to_owned(), "b".to_owned());
        assert_eq!(app.update(Message::Cancel), Some(1));
        let mut app = App::new("t".to_owned(), "b".to_owned());
        assert_eq!(app.update(Message::Close), Some(1));
        let mut app = App::new("t".to_owned(), "b".to_owned());
        assert_eq!(app.update(Message::Timeout), Some(5));
    }

    #[test]
    fn timeout_countdown_expires_to_five() {
        let mut app = App::with_options(
            "t".to_owned(),
            "b".to_owned(),
            "O".to_owned(),
            "C".to_owned(),
            Some(2),
        );
        assert_eq!(app.tick(), None);
        assert_eq!(app.remaining, Some(1));
        assert_eq!(app.tick(), Some(5));
        assert_eq!(app.exit_code, Some(5));
    }

    #[test]
    fn preferred_size_is_fixed_and_grows_with_body() {
        let short = App::new("Info".to_owned(), "one line".to_owned());
        let tall = App::new(
            "Info".to_owned(),
            (0..12)
                .map(|i| format!("line {i}"))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        let (w, h_short) = short.preferred_size(0, 0);
        let (_, h_tall) = tall.preferred_size(0, 0);

        assert_eq!(w, DEFAULT_WIDTH, "width 0 falls back to the default width");
        assert!(h_tall > h_short, "body text must grow the window");

        let (w, h) = short.preferred_size(420, 0);
        assert_eq!(w, 420, "--width is honoured exactly (min == max)");
        assert!(h >= 120, "window must fit title + body + buttons: {h}");

        let (_, forced) = short.preferred_size(420, 600);
        assert_eq!(forced, 600, "--height wins over the computed height");
    }

    #[test]
    fn auto_height_is_capped_for_overlong_bodies() {
        use geometry::{
            BODY_MAX_HEIGHT, BODY_SLACK, BUTTON_GAP, BUTTON_ROW_HEIGHT, CARD_PADDING, TITLE_GAP,
            TITLE_HEIGHT,
        };
        let huge = App::new("Info".to_owned(), "x".repeat(20_000));
        let height = huge.auto_height(400);
        assert!(
            height
                <= (CARD_PADDING * 2.0
                    + TITLE_HEIGHT
                    + TITLE_GAP
                    + BUTTON_GAP
                    + BUTTON_ROW_HEIGHT
                    + BODY_SLACK
                    + BODY_MAX_HEIGHT) as u32,
            "window must not grow past the body scroll cap: {height}"
        );
    }

    #[test]
    fn focus_stubs_toggle() {
        let mut app = App::new("t".to_owned(), String::new());
        app.on_focused();
        assert!(app.focused && app.activated);
        app.on_unfocused();
        assert!(!app.focused);
    }
}
