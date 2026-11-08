//! Progress view.

use cosmic::Element;
use cosmic::widget::{column, progress_bar, text};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::progress::Progress;

pub(super) fn view<'a>(runner: &'a Runner, prog: &Progress) -> Element<'a, RunnerMessage> {
    let state = runner.progress_state.as_ref().expect("progress state set");
    let bar: Element<'_, RunnerMessage> = if prog.pulsate {
        progress_bar::indeterminate_linear().into()
    } else {
        progress_bar::determinate_linear((state.percentage / 100.0).clamp(0.0, 1.0)).into()
    };
    let mut view = column![text::body(state.text.clone()), bar].spacing(8);
    if prog.time_remaining && !prog.pulsate && state.percentage > 0.0 {
        let elapsed = runner.progress_started.elapsed().as_secs_f32();
        let remaining = (elapsed / state.percentage * (100.0 - state.percentage)).ceil() as u64;
        view = view.push(text::body(format!("{remaining}s remaining")));
    }
    view.into()
}
