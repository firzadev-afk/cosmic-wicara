//! Forms dialog builder.

use wicara_app::dialogs::forms::Forms;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String) -> DialogRequest {
    let mut forms = Forms::new(title, text);
    for label in &cli.per_dialog.add_entry {
        forms.add_entry(label.clone());
    }
    for label in &cli.per_dialog.add_password {
        forms.add_password(label.clone());
    }
    for label in &cli.per_dialog.add_multiline_entry {
        forms.add_multiline_entry(label.clone());
    }
    for label in &cli.per_dialog.add_calendar {
        forms.add_calendar(label.clone());
    }
    for (index, label) in cli.per_dialog.add_list.iter().enumerate() {
        let list_values = cli
            .per_dialog
            .list_values
            .get(index)
            .map(String::as_str)
            .unwrap_or("");
        let column_values = cli
            .per_dialog
            .column_values
            .get(index)
            .map(String::as_str)
            .unwrap_or("");
        forms.add_list(label.clone(), list_values, column_values);
    }
    for (index, label) in cli.per_dialog.add_combo.iter().enumerate() {
        let combo_values = cli
            .per_dialog
            .combo_values
            .get(index)
            .map(String::as_str)
            .unwrap_or("");
        forms.add_combo(label.clone(), combo_values);
    }
    forms.show_header = cli.per_dialog.show_header;
    forms.date_format = cli.per_dialog.forms_date_format.clone();
    forms.separator = cli.per_dialog.separator.clone();
    DialogRequest::Forms(forms)
}
