//! Entry dialog.

/// Entry dialog config.
#[derive(Clone, Debug)]
pub struct Entry {
    /// Window title.
    pub title: String,
    /// Field label.
    pub text: String,
    /// Initial value.
    pub entry_text: String,
    /// Hide text?
    pub hide_text: bool,
    /// Current value.
    pub value: String,
}

impl Entry {
    /// Create entry dialog.
    #[must_use]
    pub fn new(title: String, text: String, entry_text: String, hide_text: bool) -> Self {
        Self {
            value: entry_text.clone(),
            title,
            text,
            entry_text,
            hide_text,
        }
    }

    /// Replace value.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    /// Displayed text.
    #[must_use]
    pub fn display_text(&self) -> String {
        if self.hide_text {
            "•".repeat(self.value.chars().count())
        } else {
            self.value.clone()
        }
    }

    /// OK payload.
    #[must_use]
    pub fn output(&self) -> String {
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_returns_typed_text_unmasked() {
        let mut entry = Entry::new(
            "T".to_owned(),
            "Name:".to_owned(),
            "initial".to_owned(),
            true,
        );
        assert_eq!(entry.display_text(), "•••••••");
        entry.set_value("hello".to_owned());
        assert_eq!(entry.output(), "hello");
        assert_eq!(entry.display_text(), "•••••");
    }

    #[test]
    fn visible_entry_shows_raw_value() {
        let entry = Entry::new("T".to_owned(), String::new(), "abc".to_owned(), false);
        assert_eq!(entry.display_text(), "abc");
        assert_eq!(entry.output(), "abc");
    }
}
