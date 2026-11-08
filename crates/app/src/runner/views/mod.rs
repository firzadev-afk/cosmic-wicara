//! Per-dialog view helpers.

pub mod calendar;
pub mod color;
pub mod entry;
pub mod file;
pub mod forms;
pub mod list;
pub mod password;
pub mod progress;
pub mod scale;
pub mod text_info;

use cosmic::Element;
use cosmic::iced::Length;
use cosmic::widget::{button, column, dialog, row};

use super::config::{DialogRequest, RunnerMessage};
use super::state::Runner;
use crate::dialogs::forms::Forms;
use crate::dialogs::list::List;
use crate::dialogs::progress::Progress;
use crate::dialogs::text_info::TextInfo;
use wicara_core::icons;

impl Runner {
    /// Extra buttons row.
    pub(crate) fn extra_row(&self) -> Option<Element<'_, RunnerMessage>> {
        if self.cfg.extra_button.is_empty() {
            return None;
        }
        let mut extra = row![].spacing(8);
        for (index, label) in self.cfg.extra_button.iter().enumerate() {
            extra =
                extra.push(button::standard(label.clone()).on_press(RunnerMessage::Extra(index)));
        }
        Some(extra.into())
    }

    /// Wrap with extras.
    pub(crate) fn with_extras<'a>(
        &'a self,
        control: Element<'a, RunnerMessage>,
    ) -> Element<'a, RunnerMessage> {
        match self.extra_row() {
            Some(extras) => column![control, extras].spacing(8).into(),
            None => control,
        }
    }

    /// Render message dialog.
    pub(crate) fn view_message(&self) -> Element<'_, RunnerMessage> {
        let body = self.shell.body_with_countdown();
        let dialog = dialog()
            .title(self.cfg.title.clone())
            .body(body)
            .is_overlay(false)
            .width(Length::Fill);
        let dialog = match self.extra_row() {
            Some(extras) => dialog.control(extras),
            None => dialog,
        };
        let dialog = if let Some(name) = &self.cfg.icon {
            let named = icons::for_override(name, icons::DialogIcon::Information);
            dialog.icon(named.size(32).icon())
        } else {
            dialog
        };
        if self.cfg.switch {
            dialog.into()
        } else if self.cfg.default_cancel {
            dialog
                .primary_action(
                    button::suggested(self.cfg.cancel_label.clone())
                        .on_press(RunnerMessage::Cancel),
                )
                .secondary_action(
                    button::standard(self.cfg.ok_label.clone()).on_press(RunnerMessage::Ok),
                )
                .into()
        } else {
            dialog
                .primary_action(
                    button::suggested(self.cfg.ok_label.clone()).on_press(RunnerMessage::Ok),
                )
                .secondary_action(
                    button::standard(self.cfg.cancel_label.clone()).on_press(RunnerMessage::Cancel),
                )
                .into()
        }
    }

    /// Render input dialog.
    pub(crate) fn view_input(&self) -> Element<'_, RunnerMessage> {
        let body = self.cfg.body.clone();
        let primary = button::suggested(self.cfg.ok_label.clone()).on_press(RunnerMessage::Ok);
        let secondary =
            button::standard(self.cfg.cancel_label.clone()).on_press(RunnerMessage::Cancel);
        let control: Element<'_, RunnerMessage> = match &self.cfg.request {
            DialogRequest::Entry(entry) => entry::control(self, entry),
            DialogRequest::Password(password) => password::control(self, password),
            DialogRequest::Calendar(_) => {
                calendar::control(self.cal_model.as_ref().expect("calendar model set"))
            }
            DialogRequest::Scale(scale) => scale::control(self, scale),
            DialogRequest::Color(sel) => color::control(self, sel),
            DialogRequest::File(sel) => file::control(sel),
            DialogRequest::List(list) => self.view_list(list),
            DialogRequest::TextInfo(info) => self.view_text_info(info),
            DialogRequest::Progress(prog) => self.view_progress(prog),
            DialogRequest::Forms(forms) => self.view_forms(forms),
            DialogRequest::Message => unreachable!("message handled separately"),
        };
        let hide_cancel = matches!(&self.cfg.request, DialogRequest::Progress(p) if p.no_cancel);
        let mut dlg = dialog()
            .title(self.cfg.title.clone())
            .body(body)
            .control(self.with_extras(control))
            .primary_action(primary)
            .is_overlay(false)
            .width(Length::Fill);
        if !hide_cancel {
            dlg = dlg.secondary_action(secondary);
        }
        dlg.into()
    }

    /// Render list rows.
    pub(crate) fn view_list(&self, list: &List) -> Element<'_, RunnerMessage> {
        list::view(self, list)
    }

    /// Render text-info.
    pub(crate) fn view_text_info(&self, info: &TextInfo) -> Element<'_, RunnerMessage> {
        text_info::view(self, info)
    }

    /// Render progress.
    pub(crate) fn view_progress(&self, prog: &Progress) -> Element<'_, RunnerMessage> {
        progress::view(self, prog)
    }

    /// Render forms.
    pub(crate) fn view_forms(&self, forms: &Forms) -> Element<'_, RunnerMessage> {
        forms::view(self, forms)
    }
}
