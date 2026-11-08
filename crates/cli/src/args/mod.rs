//! CLI flags.

pub mod general;
pub mod per_dialog;

pub use general::GeneralArgs;
pub use per_dialog::PerDialogArgs;

use clap::Parser;

/// Dialog kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    Calendar,
    Entry,
    Error,
    Info,
    FileSelection,
    List,
    Notification,
    Progress,
    Question,
    Warning,
    Scale,
    TextInfo,
    ColorSelection,
    Password,
    Forms,
    About,
}

impl DialogKind {
    /// Kind flag name.
    pub fn flag(self) -> &'static str {
        match self {
            DialogKind::Calendar => "calendar",
            DialogKind::Entry => "entry",
            DialogKind::Error => "error",
            DialogKind::Info => "info",
            DialogKind::FileSelection => "file-selection",
            DialogKind::List => "list",
            DialogKind::Notification => "notification",
            DialogKind::Progress => "progress",
            DialogKind::Question => "question",
            DialogKind::Warning => "warning",
            DialogKind::Scale => "scale",
            DialogKind::TextInfo => "text-info",
            DialogKind::ColorSelection => "color-selection",
            DialogKind::Password => "password",
            DialogKind::Forms => "forms",
            DialogKind::About => "about",
        }
    }
}

/// CLI flags.
#[derive(Debug, Clone, Parser)]
#[command(
    name = "cosmic-wicara",
    version,
    about = "Native COSMIC dialog tool",
    long_about = "Native COSMIC dialog tool (Rust + libcosmic, COSMIC-only).\n\
        stdout carries data only; diagnostics go to stderr.\n\
        Exit codes: 0 OK/help, 1 Cancel/close, 5 timeout, 255 error."
)]
pub struct Cli {
    #[command(flatten)]
    pub general: GeneralArgs,

    #[command(flatten)]
    pub per_dialog: PerDialogArgs,

    /// List row data.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub values: Vec<String>,
}

impl Cli {
    /// Selected kinds.
    pub fn selected_kinds(&self) -> Vec<DialogKind> {
        let mut kinds = Vec::new();
        if self.per_dialog.calendar {
            kinds.push(DialogKind::Calendar);
        }
        if self.per_dialog.entry {
            kinds.push(DialogKind::Entry);
        }
        if self.per_dialog.error {
            kinds.push(DialogKind::Error);
        }
        if self.per_dialog.info {
            kinds.push(DialogKind::Info);
        }
        if self.per_dialog.file_selection {
            kinds.push(DialogKind::FileSelection);
        }
        if self.per_dialog.list {
            kinds.push(DialogKind::List);
        }
        if self.per_dialog.notification {
            kinds.push(DialogKind::Notification);
        }
        if self.per_dialog.progress {
            kinds.push(DialogKind::Progress);
        }
        if self.per_dialog.question {
            kinds.push(DialogKind::Question);
        }
        if self.per_dialog.warning {
            kinds.push(DialogKind::Warning);
        }
        if self.per_dialog.scale {
            kinds.push(DialogKind::Scale);
        }
        if self.per_dialog.text_info {
            kinds.push(DialogKind::TextInfo);
        }
        if self.per_dialog.color_selection {
            kinds.push(DialogKind::ColorSelection);
        }
        if self.per_dialog.password {
            kinds.push(DialogKind::Password);
        }
        if self.per_dialog.forms {
            kinds.push(DialogKind::Forms);
        }
        if self.general.about {
            kinds.push(DialogKind::About);
        }
        kinds
    }

    /// Selected flag count.
    pub fn dialog_count(&self) -> usize {
        self.selected_kinds().len()
    }

    /// Any --help-* flag set?
    pub fn wants_help(&self) -> bool {
        let g = &self.general;
        g.help_all
            || g.help_general
            || g.help_calendar
            || g.help_entry
            || g.help_error
            || g.help_info
            || g.help_file_selection
            || g.help_list
            || g.help_notification
            || g.help_progress
            || g.help_question
            || g.help_warning
            || g.help_scale
            || g.help_text_info
            || g.help_color_selection
            || g.help_password
            || g.help_forms
            || g.help_misc
    }

    /// Single selected kind.
    pub fn kind(&self) -> Option<DialogKind> {
        let kinds = self.selected_kinds();
        if kinds.len() == 1 {
            Some(kinds[0])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_json_lang_and_window_icon() {
        let cli = Cli::parse_from([
            "cosmic-wicara",
            "--info",
            "--json",
            "--lang=id",
            "--window-icon=dialog-information",
        ]);
        assert!(cli.general.json);
        assert_eq!(cli.general.lang, "id");
        assert_eq!(
            cli.general.window_icon.as_deref(),
            Some("dialog-information")
        );
        assert_eq!(cli.kind(), Some(DialogKind::Info));
    }

    #[test]
    fn parses_question() {
        let cli = Cli::parse_from([
            "cosmic-wicara",
            "--question",
            "--title=T",
            "--text=H",
            "--ok-label=O",
            "--cancel-label=C",
        ]);
        assert_eq!(cli.general.title, "T");
        assert_eq!(cli.per_dialog.text.as_deref(), Some("H"));
        assert_eq!(cli.general.ok_label, "O");
        assert_eq!(cli.general.cancel_label, "C");
        assert_eq!(cli.kind(), Some(DialogKind::Question));
        assert!(cli.per_dialog.question);
    }
}
