//! Color view control.

use cosmic::Element;
use cosmic::widget::column;

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::color_selection::ColorSelection;

pub(super) fn control<'a>(
    runner: &'a Runner,
    sel: &'a ColorSelection,
) -> Element<'a, RunnerMessage> {
    let model = runner.color_model.as_ref().expect("color model set");
    let mut view = column![model.picker_button(RunnerMessage::ColorUpdate, Some(12))].spacing(8);
    if sel.is_palette_visible() {
        view = view.push(
            model
                .builder(RunnerMessage::ColorUpdate)
                .build("Recent", "Copy", "Copied"),
        );
    }
    view.into()
}
