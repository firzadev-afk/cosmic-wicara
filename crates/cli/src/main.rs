//! Binary entry point.

use std::io::Write;
use std::process::ExitCode;

use clap::{CommandFactory, Parser};

use cli::args::{Cli, DialogKind};

use wicara_app::dialogs::about::About;
use wicara_app::dialogs::notification::Notification;
use wicara_app::runner::DialogConfig;

mod builders;

use builders::default_title;

/// Print clap help (single source of truth from args.rs headings).
fn print_help() {
    let _ = Cli::command().print_long_help();
    let _ = std::io::stdout().flush();
}

/// Show dialog window.
fn show_dialog(cli: &Cli, kind: DialogKind) -> ExitCode {
    let title = if cli.general.title.is_empty() {
        default_title(kind, &cli.general.lang).to_owned()
    } else {
        cli.general.title.clone()
    };
    let width = if cli.general.width == 0 {
        400
    } else {
        cli.general.width
    };
    let icon = cli
        .per_dialog
        .icon
        .clone()
        .or_else(|| cli.general.window_icon.clone());

    if matches!(kind, DialogKind::Notification) {
        let dialog = Notification::new(title, cli.per_dialog.text.clone().unwrap_or_default())
            .with_icon(icon.unwrap_or_default())
            .with_listen(cli.per_dialog.listen)
            .with_hint(cli.per_dialog.hint.clone().unwrap_or_default());
        let exit = dialog.run();
        return ExitCode::from(exit.into_code());
    }

    let text = cli.per_dialog.text.clone().unwrap_or_default();
    let request = match kind {
        DialogKind::Info | DialogKind::Error | DialogKind::Warning | DialogKind::Question => {
            builders::message::build()
        }
        DialogKind::About => builders::message::build(),
        DialogKind::Entry => builders::entry::build_entry(cli, title.clone(), text.clone()),
        DialogKind::Password => builders::entry::build_password(cli, title.clone(), text.clone()),
        DialogKind::Calendar => {
            match builders::calendar::build(cli, title.clone(), text.clone(), width) {
                Ok(request) => request,
                Err(()) => return ExitCode::from(255),
            }
        }
        DialogKind::Scale => {
            match builders::scale::build(cli, title.clone(), text.clone(), width) {
                Ok(request) => request,
                Err(()) => return ExitCode::from(255),
            }
        }
        DialogKind::ColorSelection => {
            match builders::color::build(cli, title.clone(), text.clone(), width) {
                Ok(request) => request,
                Err(()) => return ExitCode::from(255),
            }
        }
        DialogKind::FileSelection => match builders::file_selection::build(cli, title.clone()) {
            Ok(request) => request,
            Err(()) => return ExitCode::from(255),
        },
        DialogKind::List => match builders::list::build(cli, title.clone(), text.clone(), width) {
            Ok(request) => request,
            Err(()) => return ExitCode::from(255),
        },
        DialogKind::TextInfo => {
            match builders::text_info::build(cli, title.clone(), text.clone(), width) {
                Ok(request) => request,
                Err(exit) => return exit,
            }
        }
        DialogKind::Progress => builders::progress::build(cli, title.clone(), text.clone()),
        DialogKind::Forms => builders::forms::build(cli, title.clone(), text.clone()),
        DialogKind::Notification => unreachable!("notification handled above"),
    };

    let body = match kind {
        DialogKind::About => About::new().with_title(title.clone()).about_text(),
        DialogKind::List => text.clone(),
        _ => text.clone(),
    };
    let cfg = DialogConfig {
        title,
        body,
        ok_label: cli.general.ok_label.clone(),
        cancel_label: cli.general.cancel_label.clone(),
        extra_button: cli.general.extra_button.clone(),
        switch: cli.per_dialog.switch,
        default_cancel: cli.per_dialog.default_cancel,
        timeout: cli.general.timeout,
        width,
        height: cli.general.height,
        json: cli.general.json,
        icon,
        ellipsize: cli.per_dialog.ellipsize,
        no_wrap: cli.per_dialog.no_wrap,
        request,
    };
    let code = wicara_app::runner::run_dialog(cfg);
    ExitCode::from(code)
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            if err.exit_code() == 0 {
                let _ = err.print();
                return ExitCode::SUCCESS;
            }
            let _ = err.print();
            return ExitCode::from(255);
        }
    };

    if cli.wants_help() {
        print_help();
        return ExitCode::SUCCESS;
    }

    let count = cli.dialog_count();
    if count == 0 {
        eprintln!("You must specify a dialog type. See 'cosmic-wicara --help' for details");
        return ExitCode::from(255);
    }
    if count > 1 {
        eprintln!("Two or more dialog options specified");
        return ExitCode::from(255);
    }

    let kind = cli.kind().expect("exactly one dialog kind selected");
    show_dialog(&cli, kind)
}
