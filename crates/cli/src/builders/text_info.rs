//! Text info dialog builder.

use std::process::ExitCode;

use wicara_app::dialogs::text_info::TextInfo;
use wicara_app::runner::DialogRequest;

use super::read_stdin;
use cli::args::Cli;

pub fn build(
    cli: &Cli,
    title: String,
    text: String,
    width: u32,
) -> Result<DialogRequest, ExitCode> {
    let mut info = TextInfo::new();
    info.title = title;
    info.width = width;
    info.filename = cli.per_dialog.filename.clone();
    info.editable = cli.per_dialog.editable;
    info.font = cli.per_dialog.font.clone();
    info.checkbox = cli.per_dialog.checkbox.clone();
    info.auto_scroll = cli.per_dialog.auto_scroll;
    if info.filename.is_some() {
        match info.resolve_text() {
            Ok(content) => info.text = content,
            Err(exit) => return Err(ExitCode::from(exit.into_code())),
        }
    } else if !std::io::IsTerminal::is_terminal(&std::io::stdin()) {
        info.text = read_stdin();
    } else {
        info.text = text;
    }
    Ok(DialogRequest::TextInfo(info))
}
