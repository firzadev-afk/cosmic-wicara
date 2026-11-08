//! Scale dialog builder.

use wicara_app::dialogs::scale::Scale;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String, width: u32) -> Result<DialogRequest, ()> {
    let mut scale = Scale::new();
    scale.title = title;
    scale.text = text;
    scale.width = width;
    if let Some(value) = cli.per_dialog.value {
        scale.value = value;
    }
    if let Some(value) = cli.per_dialog.min_value {
        scale.min_value = value;
    }
    if let Some(value) = cli.per_dialog.max_value {
        scale.max_value = value;
    }
    if let Some(value) = cli.per_dialog.step {
        scale.step = value;
    }
    scale.print_partial = cli.per_dialog.print_partial;
    scale.hide_value = cli.per_dialog.hide_value;
    if scale.validate().is_err() {
        return Err(());
    }
    Ok(DialogRequest::Scale(scale))
}
