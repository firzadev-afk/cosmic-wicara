//! Scale view control.

use cosmic::Element;
use cosmic::widget::{column, slider, text};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::scale::Scale;

pub(super) fn control<'a>(runner: &'a Runner, scale: &'a Scale) -> Element<'a, RunnerMessage> {
    let value = runner.scale_value;
    let range = scale.min_value as f32..=scale.max_value as f32;
    let mut view = column![slider(range, value, RunnerMessage::ScaleChanged)].spacing(8);
    if scale.is_value_visible() {
        view = view.push(text::body(format!(
            "{}",
            scale.snap(scale.clamp(value as i64))
        )));
    }
    view.into()
}
