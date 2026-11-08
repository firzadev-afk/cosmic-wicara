//! Forms view.

use cosmic::Element;
use cosmic::widget::{column, row, scrollable, text, text_input};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::forms::Forms;

pub(super) fn view<'a>(_runner: &'a Runner, forms: &Forms) -> Element<'a, RunnerMessage> {
    let mut view = column![].spacing(8);
    for (index, field) in forms.fields.iter().enumerate() {
        let value = field.value();
        let input = text_input(field.label().to_owned(), value)
            .on_input(move |v| RunnerMessage::FormInput(index, v));
        view = view.push(row![text::body(field.label().to_owned()), input].spacing(8));
    }
    scrollable(view).into()
}
