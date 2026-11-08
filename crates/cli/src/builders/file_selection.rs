//! File selection dialog builder.

use wicara_app::dialogs::file_selection::FileSelection;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String) -> Result<DialogRequest, ()> {
    let mut sel = FileSelection::new(title);
    sel.filename = cli.per_dialog.filename.clone();
    sel.multiple = cli.per_dialog.multiple;
    sel.directory = cli.per_dialog.directory;
    sel.save = cli.per_dialog.save;
    sel.separator = cli.per_dialog.separator.clone();
    sel.file_filter = cli
        .per_dialog
        .file_filter
        .iter()
        .map(|f| f.rsplit('|').next().unwrap_or(f).trim().to_owned())
        .collect();
    if let Err(detail) = sel.check_start_dir() {
        eprintln!("file-selection: {detail}");
        return Err(());
    }
    Ok(DialogRequest::File(sel))
}
