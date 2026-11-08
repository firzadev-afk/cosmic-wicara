//! General CLI flags.

use clap::Args;

/// General CLI flags.
#[derive(Debug, Clone, Args)]
pub struct GeneralArgs {
    #[arg(
        long,
        default_value = "",
        help_heading = "General options",
        help = "Set the dialog title"
    )]
    pub title: String,

    #[arg(
        long,
        default_value_t = 0,
        help_heading = "General options",
        help = "Set the width"
    )]
    pub width: u32,

    #[arg(
        long,
        default_value_t = 0,
        help_heading = "General options",
        help = "Set the height"
    )]
    pub height: u32,

    #[arg(
        long,
        help_heading = "General options",
        help = "Set dialog timeout in seconds"
    )]
    pub timeout: Option<u64>,

    #[arg(
        long,
        default_value = "OK",
        help_heading = "General options",
        help = "Set the label of the OK button"
    )]
    pub ok_label: String,

    #[arg(
        long,
        default_value = "Cancel",
        help_heading = "General options",
        help = "Set the label of the Cancel button"
    )]
    pub cancel_label: String,

    #[arg(long, help_heading = "General options", help = "Add an extra button")]
    pub extra_button: Vec<String>,

    #[arg(long, help_heading = "General options", help = "Set the modal hint")]
    pub modal: bool,

    #[arg(
        long,
        help_heading = "General options",
        help = "DEPRECATED; does nothing"
    )]
    pub attach: Option<String>,

    #[arg(long, help_heading = "Help Options", help = "Show all help options")]
    pub help_all: bool,

    #[arg(long, help_heading = "Help Options", help = "Show general options")]
    pub help_general: bool,

    #[arg(long, help_heading = "Help Options", help = "Show calendar options")]
    pub help_calendar: bool,

    #[arg(long, help_heading = "Help Options", help = "Show text entry options")]
    pub help_entry: bool,

    #[arg(long, help_heading = "Help Options", help = "Show error options")]
    pub help_error: bool,

    #[arg(long, help_heading = "Help Options", help = "Show info options")]
    pub help_info: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show file selection options"
    )]
    pub help_file_selection: bool,

    #[arg(long, help_heading = "Help Options", help = "Show list options")]
    pub help_list: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show notification options"
    )]
    pub help_notification: bool,

    #[arg(long, help_heading = "Help Options", help = "Show progress options")]
    pub help_progress: bool,

    #[arg(long, help_heading = "Help Options", help = "Show question options")]
    pub help_question: bool,

    #[arg(long, help_heading = "Help Options", help = "Show warning options")]
    pub help_warning: bool,

    #[arg(long, help_heading = "Help Options", help = "Show scale options")]
    pub help_scale: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show text information options"
    )]
    pub help_text_info: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show color selection options"
    )]
    pub help_color_selection: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show password dialog options"
    )]
    pub help_password: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show forms dialog options"
    )]
    pub help_forms: bool,

    #[arg(
        long,
        help_heading = "Help Options",
        help = "Show miscellaneous options"
    )]
    pub help_misc: bool,

    #[arg(
        long,
        help_heading = "Miscellaneous options",
        help = "About cosmic-wicara"
    )]
    pub about: bool,

    #[arg(long, help_heading = "General options", help = "Print result as JSON")]
    pub json: bool,

    #[arg(
        long,
        default_value = "en",
        help_heading = "General options",
        help = "UI language: en or id"
    )]
    pub lang: String,

    #[arg(long, help_heading = "General options", help = "Set the window icon")]
    pub window_icon: Option<String>,
}
