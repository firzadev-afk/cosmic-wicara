//! Color selection dialog builder.

use wicara_app::dialogs::color_selection::ColorSelection;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String, width: u32) -> Result<DialogRequest, ()> {
    let mut sel = ColorSelection::new();
    sel.title = title;
    sel.text = text;
    sel.width = width;
    sel.show_palette = cli.per_dialog.show_palette;
    if let Some(color) = &cli.per_dialog.color {
        sel.color = Some(color.clone());
    }
    if sel.validate().is_err() {
        return Err(());
    }
    Ok(DialogRequest::Color(sel))
}
