//! List dialog builder.

use wicara_app::dialogs::list::{List, SelectionMode};
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String, width: u32) -> Result<DialogRequest, ()> {
    let mut list = List::new(cli.per_dialog.column.clone(), cli.values.clone());
    list.title = title;
    list.text = text;
    list.width = width;
    list.mode = if cli.per_dialog.checklist {
        SelectionMode::Checklist
    } else if cli.per_dialog.radiolist {
        SelectionMode::Radiolist
    } else if cli.per_dialog.imagelist {
        SelectionMode::Imagelist
    } else {
        SelectionMode::Plain
    };
    list.editable = cli.per_dialog.editable;
    list.multiple = cli.per_dialog.multiple;
    list.print_column = cli.per_dialog.print_column.clone();
    list.hide_column = cli.per_dialog.hide_column.clone();
    list.hide_header = cli.per_dialog.hide_header;
    if let Some(separator) = &cli.per_dialog.separator {
        list.separator = separator.clone();
    }
    if list.validate().is_err() {
        return Err(());
    }
    Ok(DialogRequest::List(list))
}
