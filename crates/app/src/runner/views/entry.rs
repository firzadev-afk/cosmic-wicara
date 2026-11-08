//! Entry view control.

use cosmic::Element;
use cosmic::widget::{secure_input, text_input};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::entry::Entry;

pub(super) fn control<'a>(runner: &'a Runner, entry: &'a Entry) -> Element<'a, RunnerMessage> {
    let input = runner.input.as_str();
    if entry.hide_text {
        secure_input("Type here", input, None, true)
            .on_input(RunnerMessage::Input)
            .into()
    } else {
        text_input("Type here", input)
            .on_input(RunnerMessage::Input)
            .into()
    }
}
