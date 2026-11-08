//! Output building and OK/Cancel handlers.

use wicara_core::dialog::DialogResult;
use wicara_core::dialog::Exit;

use super::state::{Runner, kill_parent};
use crate::dialogs::color_selection::ColorSelection;
use crate::dialogs::scale::Scale;

use super::config::DialogRequest;

impl Runner {
    /// OK payload.
    pub(crate) fn ok_output(&self) -> Option<String> {
        match &self.cfg.request {
            DialogRequest::Message => None,
            DialogRequest::Entry(entry) => {
                let mut entry = entry.clone();
                entry.set_value(self.input.clone());
                Some(entry.output())
            }
            DialogRequest::Password(password) => {
                let mut password = password.clone();
                password.set_username(self.username.clone());
                password.set_secret(self.input.clone());
                Some(password.output())
            }
            DialogRequest::Calendar(cal) => {
                let mut cal = cal.clone();
                if let Some(model) = &self.cal_model {
                    cal.day = model.selected.day() as u32;
                    cal.month = model.selected.month() as u32;
                    cal.year = i32::from(model.selected.year()) as u32;
                }
                Some(cal.format_date())
            }
            DialogRequest::Scale(scale) => {
                if scale.print_partial {
                    let mut lines = scale.partial_output(&self.scale_partials);
                    let current =
                        Scale::format_value(scale.snap(scale.clamp(self.scale_value as i64)));
                    if lines.last() != Some(&current) {
                        lines.push(current);
                    }
                    Some(lines.join("\n"))
                } else {
                    Some(Scale::format_value(
                        scale.snap(scale.clamp(self.scale_value as i64)),
                    ))
                }
            }
            DialogRequest::Color(sel) => {
                if let Some(model) = &self.color_model {
                    if let Some(color) = model.get_applied_color() {
                        let r = (color.r * 255.0).round() as u8;
                        let g = (color.g * 255.0).round() as u8;
                        let b = (color.b * 255.0).round() as u8;
                        return Some(ColorSelection::to_hex(r, g, b));
                    }
                }
                sel.canonical_output().ok().flatten()
            }
            DialogRequest::File(_) => None,
            DialogRequest::List(list) => {
                let lines = list
                    .selected_output(&self.list_selected)
                    .ok()
                    .unwrap_or_default();
                if lines.is_empty() {
                    None
                } else {
                    Some(lines.join("\n"))
                }
            }
            DialogRequest::TextInfo(info) => {
                if info.editable {
                    Some(self.text_edited.clone())
                } else {
                    None
                }
            }
            DialogRequest::Progress(_) => None,
            DialogRequest::Forms(forms) => Some(forms.output()),
        }
    }

    /// Handle OK press.
    pub(crate) fn on_ok(&mut self) {
        if let DialogRequest::TextInfo(info) = &self.cfg.request {
            if !info.checkbox_satisfied(self.text_checked) {
                return;
            }
        }
        let output = self.ok_output();
        self.finish(DialogResult::ok(output));
    }

    /// Handle Cancel press.
    pub(crate) fn on_cancel(&mut self) {
        if let DialogRequest::Progress(prog) = &self.cfg.request {
            if prog.auto_kill {
                kill_parent(self.parent_pid);
            }
        }
        self.finish(DialogResult::bare(Exit::Cancel));
    }
}
