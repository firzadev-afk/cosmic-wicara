//! Entry and password dialog builders.

use wicara_app::dialogs::entry::Entry;
use wicara_app::dialogs::password::Password;
use wicara_app::runner::DialogRequest;

use cli::args::Cli;

pub fn build_entry(cli: &Cli, title: String, text: String) -> DialogRequest {
    DialogRequest::Entry(Entry::new(
        title,
        text,
        cli.per_dialog.entry_text.clone().unwrap_or_default(),
        cli.per_dialog.hide_text,
    ))
}

pub fn build_password(cli: &Cli, title: String, text: String) -> DialogRequest {
    DialogRequest::Password(Password::new(title, text, cli.per_dialog.username))
}
