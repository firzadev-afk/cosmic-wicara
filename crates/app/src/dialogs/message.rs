//! Unified message dialog (info/error/warning/question).

use cosmic::iced::Length;
use cosmic::iced::core::text::{Ellipsize, EllipsizeHeightLimit, Wrapping};
use cosmic::widget::{self, button, dialog, header_bar};
use cosmic::{Element, Theme};
use wicara_core::{icons as core_icons, theme as core_theme};

use crate::window::{
    App, DEFAULT_CANCEL_LABEL, DEFAULT_OK_LABEL, DEFAULT_WIDTH, DIALOG_PADDING, Message,
};

/// Ellipsize max characters.
pub const ELLIPSIZE_MAX_CHARS: usize = 1_000;

/// Ellipsize kept lines.
pub const ELLIPSIZE_LINES: usize = 3;

/// Unified message dialog state.
#[derive(Clone, Debug)]
pub struct MessageDialog {
    /// Dialog kind (drives icon + cancel presence).
    pub kind: core_icons::DialogIcon,
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// OK button label.
    pub ok_label: String,
    /// Cancel label, if this kind shows a cancel button.
    pub cancel_label: Option<String>,
    /// Default-cancel mode.
    pub default_cancel: bool,
    /// Ellipsize enabled.
    pub ellipsize: bool,
    /// Icon override.
    pub icon: String,
    /// Minimum width.
    pub width: u32,
    /// Timeout seconds.
    pub timeout: Option<u64>,
}

/// Info dialog alias.
pub type Info = MessageDialog;
/// Error dialog alias.
pub type Error = MessageDialog;
/// Warning dialog alias.
pub type Warning = MessageDialog;
/// Question dialog alias.
pub type Question = MessageDialog;

impl MessageDialog {
    /// Create message dialog for a kind.
    #[must_use]
    pub fn new(kind: core_icons::DialogIcon, title: String, text: String) -> Self {
        let cancel_label = if kind == core_icons::DialogIcon::Question {
            Some(DEFAULT_CANCEL_LABEL.to_owned())
        } else {
            None
        };
        Self {
            kind,
            title,
            text,
            ok_label: DEFAULT_OK_LABEL.to_owned(),
            cancel_label,
            default_cancel: false,
            ellipsize: false,
            icon: String::new(),
            width: DEFAULT_WIDTH,
            timeout: None,
        }
    }

    /// Create info dialog.
    #[must_use]
    pub fn info(title: String, text: String) -> Self {
        Self::new(core_icons::DialogIcon::Information, title, text)
    }

    /// Create error dialog.
    #[must_use]
    pub fn error(title: String, text: String) -> Self {
        Self::new(core_icons::DialogIcon::Error, title, text)
    }

    /// Create warning dialog.
    #[must_use]
    pub fn warning(title: String, text: String) -> Self {
        Self::new(core_icons::DialogIcon::Warning, title, text)
    }

    /// Create question dialog.
    #[must_use]
    pub fn question(title: String, text: String) -> Self {
        Self::new(core_icons::DialogIcon::Question, title, text)
    }

    /// Set OK label.
    #[must_use]
    pub fn with_ok_label(mut self, ok_label: String) -> Self {
        self.ok_label = ok_label;
        self
    }

    /// Set Cancel label.
    #[must_use]
    pub fn with_cancel_label(mut self, cancel_label: String) -> Self {
        self.cancel_label = Some(cancel_label);
        self
    }

    /// Set ellipsize.
    #[must_use]
    pub fn with_ellipsize(mut self, ellipsize: bool) -> Self {
        self.ellipsize = ellipsize;
        self
    }

    /// Set icon override.
    #[must_use]
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = icon;
        self
    }

    /// Set width.
    #[must_use]
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width.max(1);
        self
    }

    /// Set timeout.
    #[must_use]
    pub fn with_timeout(mut self, timeout: Option<u64>) -> Self {
        self.timeout = timeout;
        self
    }

    /// Icon for dialog.
    pub fn icon_named(&self) -> cosmic::widget::icon::Named {
        core_icons::for_override(&self.icon, self.kind)
    }

    /// Symbolic icon name.
    #[must_use]
    pub fn icon_name(&self) -> String {
        if self.icon.trim().is_empty() {
            self.kind.name().to_owned()
        } else {
            core_icons::symbolic_name(&self.icon)
        }
    }

    /// Apply ellipsize.
    #[must_use]
    pub fn apply_ellipsize(text: &str, ellipsize: bool) -> String {
        if !ellipsize {
            return text.to_owned();
        }
        if text.chars().count() <= ELLIPSIZE_MAX_CHARS {
            return text.to_owned();
        }
        let truncated: String = text.chars().take(ELLIPSIZE_MAX_CHARS).collect();
        format!("{truncated}…")
    }

    /// Ellipsized body text.
    #[must_use]
    pub fn visible_text(&self) -> String {
        Self::apply_ellipsize(&self.text, self.ellipsize)
    }

    /// Ellipsize mode.
    #[must_use]
    pub const fn ellipsize_mode(enabled: bool) -> Ellipsize {
        if enabled {
            Ellipsize::End(EllipsizeHeightLimit::Lines(ELLIPSIZE_LINES))
        } else {
            Ellipsize::None
        }
    }

    /// Body with countdown.
    #[must_use]
    pub fn display_text(&self) -> String {
        let base = self.visible_text();
        match self.timeout {
            Some(secs) => {
                if base.is_empty() {
                    format!("(closes in {secs}s)")
                } else {
                    format!("{base}\n\n(closes in {secs}s)")
                }
            }
            None => base,
        }
    }

    /// Render dialog.
    pub fn view(&self) -> Element<'_, Message> {
        self.view_with_theme(&cosmic::theme::active())
    }

    /// Render dialog variant.
    pub fn view_with_theme(&self, theme: &Theme) -> Element<'_, Message> {
        let _ = core_theme::is_dark(theme);
        let display = self.display_text();

        let mut app = App::with_options(
            self.title.clone(),
            display.clone(),
            self.ok_label.clone(),
            self.cancel_label.clone().unwrap_or_default(),
            self.timeout,
        );
        app.min_width = self.width.max(1);

        let body = widget::text::body(display)
            .wrapping(Wrapping::WordOrGlyph)
            .ellipsize(Self::ellipsize_mode(self.ellipsize));

        let body_element: Element<'_, Message> = widget::container(widget::scrollable(body))
            .max_height(300.0)
            .into();

        let primary = button::suggested(app.ok_label.clone()).on_press(Message::Ok);

        let mut card = dialog()
            .title(app.title.clone())
            .icon(self.icon_named().size(48).icon())
            .control(body_element)
            .primary_action(primary)
            .width(Length::Fixed(app.min_width as f32));
        if let Some(label) = &self.cancel_label {
            card = card.secondary_action(button::standard(label.clone()).on_press(Message::Cancel));
        }
        let card: Element<'_, Message> = card.into();

        let header = header_bar()
            .title(app.title.clone())
            .on_close(Message::Close)
            .on_drag(Message::Drag);

        let shell = widget::column![header, card].spacing(0).width(Length::Fill);

        widget::container(shell).padding(DIALOG_PADDING).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn long_text() -> String {
        format!("{}{}", "a".repeat(9_000), "é😀".repeat(500))
    }

    #[test]
    fn kinds_have_distinct_icons_and_cancel() {
        let info = MessageDialog::info("T".to_owned(), "x".to_owned());
        assert_eq!(info.icon_name(), core_icons::INFORMATION_ICON);
        assert_eq!(info.cancel_label, None);
        let q = MessageDialog::question("T".to_owned(), "x".to_owned());
        assert_eq!(q.icon_name(), core_icons::QUESTION_ICON);
        assert_eq!(q.cancel_label, Some(DEFAULT_CANCEL_LABEL.to_owned()));
        assert_eq!(
            MessageDialog::error("T".to_owned(), "x".to_owned()).icon_name(),
            core_icons::ERROR_ICON
        );
        assert_eq!(
            MessageDialog::warning("T".to_owned(), "x".to_owned()).icon_name(),
            core_icons::WARNING_ICON
        );
    }

    #[test]
    fn ellipsize_long_text_does_not_panic() {
        let text = long_text();
        assert_eq!(text.chars().count(), 10_000);
        let d = MessageDialog::info("T".to_owned(), text.clone()).with_ellipsize(false);
        assert_eq!(d.visible_text(), text);
        assert_eq!(MessageDialog::ellipsize_mode(false), Ellipsize::None);
        let d = MessageDialog::info("T".to_owned(), text.clone()).with_ellipsize(true);
        let visible = d.visible_text();
        assert!(visible.ends_with('…'));
        assert_eq!(visible.chars().count(), ELLIPSIZE_MAX_CHARS + 1);
        assert!(matches!(
            MessageDialog::ellipsize_mode(true),
            Ellipsize::End(EllipsizeHeightLimit::Lines(ELLIPSIZE_LINES))
        ));
        assert_eq!(
            MessageDialog::apply_ellipsize("hello", true),
            "hello".to_owned()
        );
    }

    #[test]
    fn long_text_view_builds_without_panic() {
        let text = long_text();
        let dialog = MessageDialog::info("Info".to_owned(), text.clone())
            .with_ellipsize(false)
            .with_width(400)
            .with_timeout(Some(30));
        let _ = dialog.view();
        let _ = dialog.view_with_theme(&Theme::light());
        let _ = dialog.view_with_theme(&Theme::dark());
        assert!(core_theme::is_dark(&Theme::dark()));
        assert!(!core_theme::is_dark(&Theme::light()));
        let dialog = MessageDialog::info("Info".to_owned(), text)
            .with_ellipsize(true)
            .with_width(400);
        let _ = dialog.view();
        let _ = dialog.view_with_theme(&Theme::dark());
    }

    #[test]
    fn timeout_zero_keeps_current_semantics() {
        let dialog = MessageDialog::info("T".to_owned(), "body".to_owned()).with_timeout(Some(0));
        assert_eq!(dialog.display_text(), "body\n\n(closes in 0s)");
    }
}
