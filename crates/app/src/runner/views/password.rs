//! Password view control.

use cosmic::Element;
use cosmic::widget::{column, secure_input, text_input};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::password::Password;

pub(super) fn control<'a>(
    runner: &'a Runner,
    password: &'a Password,
) -> Element<'a, RunnerMessage> {
    let input = runner.input.as_str();
    let username = runner.username.as_str();
    let secret = secure_input("Password", input, None, true).on_input(RunnerMessage::Input);
    if password.username {
        let user = text_input("Username", username).on_input(RunnerMessage::UsernameInput);
        column![user, secret].spacing(8).into()
    } else {
        secret.into()
    }
}
