//! Text-info dialog.
use wicara_core::dialog::Exit;

use std::path::PathBuf;

/// Text-info dialog state.
#[derive(Debug, Clone, Default)]
pub struct TextInfo {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// File to show.
    pub filename: Option<PathBuf>,
    /// Editable text?
    pub editable: bool,
    /// Text font.
    pub font: Option<String>,
    /// Checkbox prompt.
    pub checkbox: Option<String>,
    /// Auto-scroll?
    pub auto_scroll: bool,
    /// Preferred width.
    pub width: u32,
}

impl TextInfo {
    /// Create text-info dialog.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: 400,
            ..Self::default()
        }
    }

    /// Set filename.
    #[must_use]
    pub fn with_filename(mut self, filename: PathBuf) -> Self {
        self.filename = Some(filename);
        self
    }

    /// Set editable.
    #[must_use]
    pub fn with_editable(mut self, editable: bool) -> Self {
        self.editable = editable;
        self
    }

    /// Set font.
    #[must_use]
    pub fn with_font(mut self, font: String) -> Self {
        self.font = Some(font);
        self
    }

    /// Set checkbox prompt.
    #[must_use]
    pub fn with_checkbox(mut self, checkbox: String) -> Self {
        self.checkbox = Some(checkbox);
        self
    }

    /// Set auto-scroll.
    #[must_use]
    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// Checkbox checked?
    #[must_use]
    pub fn checkbox_required(&self) -> bool {
        self.checkbox.is_some()
    }

    /// OK allowed?
    #[must_use]
    pub fn checkbox_satisfied(&self, checked: bool) -> bool {
        !self.checkbox_required() || checked
    }

    /// Effective font.
    #[must_use]
    pub fn font_display(&self) -> &str {
        self.font.as_deref().unwrap_or("")
    }

    /// Resolve display text.
    pub fn resolve_text(&self) -> Result<String, Exit> {
        if let Some(path) = &self.filename {
            std::fs::read_to_string(path).map_err(|err| {
                eprintln!("text-info: cannot read {}: {err}", path.display());
                Exit::Err255
            })
        } else {
            Ok(self.text.clone())
        }
    }

    /// Validate filename.
    pub fn validate(&self) -> Result<(), Exit> {
        self.resolve_text().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let info = TextInfo::new();
        assert_eq!(info.width, 400);
        assert!(info.font_display().is_empty());
    }

    #[test]
    fn missing_file_is_err255() {
        let info = TextInfo::new().with_filename(PathBuf::from("/nonexistent-xyz.txt"));
        assert_eq!(info.resolve_text(), Err(Exit::Err255));
        assert_eq!(info.validate(), Err(Exit::Err255));
    }
}
