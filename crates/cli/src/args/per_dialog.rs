//! Per-dialog CLI flags.

use std::path::PathBuf;

use clap::Args;

/// Per-dialog CLI flags.
#[derive(Debug, Clone, Args)]
pub struct PerDialogArgs {
    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display calendar dialog"
    )]
    pub calendar: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display text entry dialog"
    )]
    pub entry: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display error dialog"
    )]
    pub error: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display info dialog"
    )]
    pub info: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display file selection dialog"
    )]
    pub file_selection: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display list dialog"
    )]
    pub list: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display notification"
    )]
    pub notification: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display progress indication dialog"
    )]
    pub progress: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display question dialog"
    )]
    pub question: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display warning dialog"
    )]
    pub warning: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display scale dialog"
    )]
    pub scale: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display text information dialog"
    )]
    pub text_info: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display color selection dialog"
    )]
    pub color_selection: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display password dialog"
    )]
    pub password: bool,

    #[arg(
        long,
        help_heading = "Application Options",
        help = "Display forms dialog"
    )]
    pub forms: bool,

    #[arg(long, help_heading = "Calendar options", help = "Set the dialog text")]
    pub text: Option<String>,

    #[arg(long, help_heading = "Calendar options", help = "Set the calendar day")]
    pub day: Option<u32>,

    #[arg(
        long,
        help_heading = "Calendar options",
        help = "Set the calendar month"
    )]
    pub month: Option<u32>,

    #[arg(
        long,
        help_heading = "Calendar options",
        help = "Set the calendar year"
    )]
    pub year: Option<u32>,

    #[arg(
        long,
        help_heading = "Calendar options",
        help = "Set the format for the generated date"
    )]
    pub date_format: Option<String>,

    #[arg(long, help_heading = "Text entry options", help = "Set the entry text")]
    pub entry_text: Option<String>,

    #[arg(
        long,
        help_heading = "Text entry options",
        help = "Hide the entry text"
    )]
    pub hide_text: bool,

    #[arg(long, help_heading = "Error options", help = "Set the icon name")]
    pub icon: Option<String>,

    #[arg(
        long,
        help_heading = "Info options",
        help = "Do not enable text wrapping"
    )]
    pub no_wrap: bool,

    #[arg(
        long,
        help_heading = "Warning options",
        help = "Do not enable Pango markup"
    )]
    pub no_markup: bool,

    #[arg(
        long,
        help_heading = "Question options",
        help = "Enable ellipsizing in the dialog text. This fixes the high window size with long texts"
    )]
    pub ellipsize: bool,

    #[arg(
        long,
        help_heading = "Question options",
        help = "Give Cancel button focus by default"
    )]
    pub default_cancel: bool,

    #[arg(
        long,
        help_heading = "Question options",
        help = "Suppress OK and Cancel buttons"
    )]
    pub switch: bool,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Set the filename"
    )]
    pub filename: Option<PathBuf>,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Allow multiple file selection"
    )]
    pub multiple: bool,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Enables selection of only directories"
    )]
    pub directory: bool,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Enables save mode"
    )]
    pub save: bool,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Set the output separator character"
    )]
    pub separator: Option<String>,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "Set the file filter"
    )]
    pub file_filter: Vec<String>,

    #[arg(
        long,
        help_heading = "File selection options",
        help = "DEPRECATED; does nothing"
    )]
    pub confirm_overwrite: bool,

    #[arg(long, help_heading = "List options", help = "Set the column header")]
    pub column: Vec<String>,

    #[arg(
        long,
        help_heading = "List options",
        help = "Use checkboxes for the first column"
    )]
    pub checklist: bool,

    #[arg(
        long,
        help_heading = "List options",
        help = "Use radio buttons for the first column"
    )]
    pub radiolist: bool,

    #[arg(
        long,
        help_heading = "List options",
        help = "Use images for the first column"
    )]
    pub imagelist: bool,

    #[arg(long, help_heading = "List options", help = "Allow changing the text")]
    pub editable: bool,

    #[arg(
        long,
        help_heading = "List options",
        help = "Print the selected column(s) (1..n, 'ALL' prints all columns)"
    )]
    pub print_column: Option<String>,

    #[arg(
        long,
        help_heading = "List options",
        help = "Hide the specified column(s)"
    )]
    pub hide_column: Vec<String>,

    #[arg(long, help_heading = "List options", help = "Hide the column headers")]
    pub hide_header: bool,

    #[arg(long, help_heading = "List options", help = "DEPRECATED; does nothing")]
    pub mid_search: bool,

    #[arg(
        long,
        help_heading = "Notification options",
        help = "Read commands from stdin"
    )]
    pub listen: bool,

    #[arg(
        long,
        help_heading = "Notification options",
        help = "DEPRECATED; does nothing"
    )]
    pub hint: Option<String>,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Set the initial percentage"
    )]
    pub percentage: Option<u32>,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Pulsate the progress indicator"
    )]
    pub pulsate: bool,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Close the dialog after reaching 100%"
    )]
    pub auto_close: bool,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Kill the parent process if Cancel button is pressed"
    )]
    pub auto_kill: bool,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Hide the Cancel button"
    )]
    pub no_cancel: bool,

    #[arg(
        long,
        help_heading = "Progress options",
        help = "Estimate when progress will reach 100%"
    )]
    pub time_remaining: bool,

    #[arg(long, help_heading = "Scale options", help = "Set the initial value")]
    pub value: Option<i64>,

    #[arg(long, help_heading = "Scale options", help = "Set the minimum value")]
    pub min_value: Option<i64>,

    #[arg(long, help_heading = "Scale options", help = "Set the maximum value")]
    pub max_value: Option<i64>,

    #[arg(long, help_heading = "Scale options", help = "Set the step size")]
    pub step: Option<i64>,

    #[arg(long, help_heading = "Scale options", help = "Print the partial value")]
    pub print_partial: bool,

    #[arg(long, help_heading = "Scale options", help = "Hide the value")]
    pub hide_value: bool,

    #[arg(
        long,
        help_heading = "Text information options",
        help = "Set the text font"
    )]
    pub font: Option<String>,

    #[arg(
        long,
        help_heading = "Text information options",
        help = "Enable the 'I have read and agree' checkbox"
    )]
    pub checkbox: Option<String>,

    #[arg(
        long,
        help_heading = "Text information options",
        help = "Auto-scroll the text to the end. Only when the text is captured from stdin"
    )]
    pub auto_scroll: bool,

    #[arg(long, help_heading = "Color selection options", help = "Set the color")]
    pub color: Option<String>,

    #[arg(
        long,
        help_heading = "Color selection options",
        help = "Show the palette"
    )]
    pub show_palette: bool,

    #[arg(
        long,
        help_heading = "Password dialog options",
        help = "Show the username option"
    )]
    pub username: bool,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new entry in forms dialog"
    )]
    pub add_entry: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new password entry in forms dialog"
    )]
    pub add_password: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new multi-line entry in forms dialog (Since: 4.2)"
    )]
    pub add_multiline_entry: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new calendar in forms dialog"
    )]
    pub add_calendar: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new list in forms dialog"
    )]
    pub add_list: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "List of values separated by '|', for List"
    )]
    pub list_values: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "List of values separated by '|', for column"
    )]
    pub column_values: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Add a new combo box in forms dialog"
    )]
    pub add_combo: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "List of values separated by '|', for Combo"
    )]
    pub combo_values: Vec<String>,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Show the column headers"
    )]
    pub show_header: bool,

    #[arg(
        long,
        help_heading = "Forms dialog options",
        help = "Set the format for the generated date"
    )]
    pub forms_date_format: Option<String>,
}
