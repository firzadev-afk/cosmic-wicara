//! Calendar view control.

use cosmic::Element;
use cosmic::widget::calendar;
use jiff::civil::Weekday;

use super::super::config::RunnerMessage;

pub(super) fn control(model: &calendar::CalendarModel) -> Element<'_, RunnerMessage> {
    calendar(
        model,
        RunnerMessage::CalendarSelect,
        || RunnerMessage::CalendarPrev,
        || RunnerMessage::CalendarNext,
        Weekday::Monday,
    )
    .into()
}
