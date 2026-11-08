//! File view control.

use cosmic::Element;
use cosmic::widget::text;

use super::super::config::RunnerMessage;
use crate::dialogs::file_selection::FileSelection;

pub(super) fn control(_sel: &FileSelection) -> Element<'_, RunnerMessage> {
    text::body("Choose a file...").into()
}
