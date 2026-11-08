//! Progress dialog builder.

use wicara_app::dialogs::progress::Progress;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build(cli: &Cli, title: String, text: String) -> DialogRequest {
    let mut prog = Progress::new(title, text);
    prog.percentage = cli.per_dialog.percentage;
    prog.pulsate = cli.per_dialog.pulsate;
    prog.auto_close = cli.per_dialog.auto_close;
    prog.auto_kill = cli.per_dialog.auto_kill;
    prog.no_cancel = cli.per_dialog.no_cancel;
    prog.time_remaining = cli.per_dialog.time_remaining;
    DialogRequest::Progress(prog)
}
