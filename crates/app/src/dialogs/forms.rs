//! Forms dialog.

/// Default row separator.
pub const DEFAULT_SEPARATOR: &str = "\n";

/// One form field.
#[derive(Clone, Debug, PartialEq)]
pub enum FormField {
    /// Entry field.
    Entry { label: String, value: String },
    /// Password field.
    Password { label: String, value: String },
    /// Multi-line field.
    MultiLine { label: String, value: String },
    /// Calendar field.
    Calendar { label: String, value: String },
    /// List field.
    List {
        label: String,
        values: Vec<String>,
        columns: Vec<String>,
    },
    /// Combo field.
    Combo {
        label: String,
        values: Vec<String>,
        selected: String,
    },
}

impl FormField {
    /// Field label.
    #[must_use]
    pub fn label(&self) -> &str {
        match self {
            FormField::Entry { label, .. }
            | FormField::Password { label, .. }
            | FormField::MultiLine { label, .. }
            | FormField::Calendar { label, .. }
            | FormField::List { label, .. }
            | FormField::Combo { label, .. } => label,
        }
    }

    /// Current value.
    #[must_use]
    pub fn value(&self) -> String {
        match self {
            FormField::Entry { value, .. }
            | FormField::Password { value, .. }
            | FormField::MultiLine { value, .. }
            | FormField::Calendar { value, .. } => value.clone(),
            FormField::List { values, .. } => values.join("|"),
            FormField::Combo { selected, .. } => selected.clone(),
        }
    }
}

/// Split separator values.
#[must_use]
pub fn split_values(text: &str) -> Vec<String> {
    if text.is_empty() {
        Vec::new()
    } else {
        text.split('|').map(str::to_owned).collect()
    }
}

/// Forms dialog config.
#[derive(Clone, Debug)]
pub struct Forms {
    /// Window title.
    pub title: String,
    /// Body text.
    pub text: String,
    /// Form fields.
    pub fields: Vec<FormField>,
    /// Show headers?
    pub show_header: bool,
    /// Calendar date format.
    pub date_format: Option<String>,
    /// Row separator.
    pub separator: Option<String>,
}

impl Forms {
    /// Create forms dialog.
    #[must_use]
    pub fn new(title: String, text: String) -> Self {
        Self {
            title,
            text,
            fields: Vec::new(),
            show_header: false,
            date_format: None,
            separator: None,
        }
    }

    /// Add entry field.
    pub fn add_entry(&mut self, label: String) {
        self.fields.push(FormField::Entry {
            label,
            value: String::new(),
        });
    }

    /// Add password field.
    pub fn add_password(&mut self, label: String) {
        self.fields.push(FormField::Password {
            label,
            value: String::new(),
        });
    }

    /// Add multi-line field.
    pub fn add_multiline_entry(&mut self, label: String) {
        self.fields.push(FormField::MultiLine {
            label,
            value: String::new(),
        });
    }

    /// Add calendar field.
    pub fn add_calendar(&mut self, label: String) {
        self.fields.push(FormField::Calendar {
            label,
            value: String::new(),
        });
    }

    /// Add list field.
    pub fn add_list(&mut self, label: String, list_values: &str, column_values: &str) {
        self.fields.push(FormField::List {
            label,
            values: split_values(list_values),
            columns: split_values(column_values),
        });
    }

    /// Add combo field.
    pub fn add_combo(&mut self, label: String, combo_values: &str) {
        let values = split_values(combo_values);
        let selected = values.first().cloned().unwrap_or_default();
        self.fields.push(FormField::Combo {
            label,
            values,
            selected,
        });
    }

    /// Effective separator.
    #[must_use]
    pub fn effective_separator(&self) -> &str {
        self.separator
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or(DEFAULT_SEPARATOR)
    }

    /// Set field value.
    pub fn set_value(&mut self, index: usize, value: String) {
        if let Some(field) = self.fields.get_mut(index) {
            match field {
                FormField::Entry { value: v, .. }
                | FormField::Password { value: v, .. }
                | FormField::MultiLine { value: v, .. }
                | FormField::Calendar { value: v, .. } => *v = value,
                FormField::Combo { selected, .. } => *selected = value,
                FormField::List { values, .. } => {
                    *values = split_values(&value);
                }
            }
        }
    }

    /// OK payload.
    #[must_use]
    pub fn output(&self) -> String {
        let sep = self.effective_separator();
        self.fields
            .iter()
            .map(|f| format!("{}={}", f.label(), f.value()))
            .collect::<Vec<_>>()
            .join(sep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_values_keeps_spaces_and_handles_empty() {
        assert_eq!(split_values("a|b c|d"), ["a", "b c", "d"]);
        assert!(split_values("").is_empty());
    }

    #[test]
    fn list_value_round_trips_all_values() {
        let mut forms = Forms::new("T".to_owned(), "Hi".to_owned());
        forms.add_list("Pick".to_owned(), "a|b|c", "Col");
        assert_eq!(forms.output(), "Pick=a|b|c");
        forms.set_value(0, "x|y".to_owned());
        assert_eq!(forms.output(), "Pick=x|y");
        forms.set_value(0, String::new());
        assert_eq!(forms.output(), "Pick=");
    }

    #[test]
    fn output_formats_label_value_rows() {
        let mut forms = Forms::new("T".to_owned(), "Hi".to_owned());
        forms.add_entry("Name".to_owned());
        forms.add_combo("OS".to_owned(), "Ubuntu|Fedora");
        forms.show_header = true;
        assert_eq!(forms.output(), "Name=\nOS=Ubuntu");
        assert!(forms.show_header);
    }
}
