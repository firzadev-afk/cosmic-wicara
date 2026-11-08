//! Text info view.

use cosmic::Element;
use cosmic::widget::{checkbox, column, scrollable, text, text_input};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::text_info::TextInfo;

pub(super) fn view<'a>(runner: &'a Runner, info: &TextInfo) -> Element<'a, RunnerMessage> {
    let mut view = column![].spacing(8);
    if info.editable {
        view = view.push(
            text_input("Edit text", runner.text_edited.clone()).on_input(RunnerMessage::TextEdited),
        );
    } else {
        view = view.push(scrollable(text::body(runner.text_edited.clone())));
    }
    if let Some(prompt) = &info.checkbox {
        view = view.push(
            checkbox(runner.text_checked)
                .label(prompt.clone())
                .on_toggle(RunnerMessage::TextChecked),
        );
    }
    view.into()
}
