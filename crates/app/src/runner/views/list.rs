//! List view.

use cosmic::Element;
use cosmic::widget::{button, checkbox, column, radio, row, scrollable, text, text_input};

use super::super::config::RunnerMessage;
use super::super::state::Runner;
use crate::dialogs::list::{List, SelectionMode};

pub(super) fn view<'a>(runner: &'a Runner, list: &List) -> Element<'a, RunnerMessage> {
    let mut view = column![].spacing(4);
    if !list.hide_header && !list.columns.is_empty() {
        view = view.push(text::body(list.columns.join(&list.separator)));
    }
    let cols = list.columns.len().max(1);
    let rows = list.rows.len() / cols;
    for index in 0..rows {
        let cells = &list.rows[index * cols..(index + 1) * cols];
        let label = cells.join(&list.separator);
        let selected = runner.list_selected.contains(&index);
        let row_view: Element<'_, RunnerMessage> = match list.mode {
            SelectionMode::Checklist => {
                let checked = runner.list_checked.get(index).copied().unwrap_or(false);
                checkbox(checked)
                    .label(label)
                    .on_toggle(move |_| RunnerMessage::ListToggle(index))
                    .into()
            }
            SelectionMode::Radiolist => radio(
                text::body(label),
                index,
                if selected { Some(index) } else { None },
                move |_| RunnerMessage::ListToggle(index),
            )
            .into(),
            _ => {
                let style = if selected {
                    button::suggested(label).on_press(RunnerMessage::ListToggle(index))
                } else {
                    button::standard(label).on_press(RunnerMessage::ListToggle(index))
                };
                style.into()
            }
        };
        if list.editable {
            let current = cells.join(&list.separator);
            view = view.push(
                row![
                    row_view,
                    text_input("Edit", current)
                        .on_input(move |v| RunnerMessage::ListEdit(index, v))
                ]
                .spacing(4),
            );
        } else {
            view = view.push(row_view);
        }
    }
    scrollable(view).into()
}
