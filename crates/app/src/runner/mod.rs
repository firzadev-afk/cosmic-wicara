//! Dialog runner.

pub mod config;
pub mod file_task;
pub mod output;
pub mod state;
pub mod views;

pub use config::{DialogConfig, DialogRequest, FileOutcome, RunnerMessage};
pub use state::Runner;

use std::time::Duration;

use cosmic::ApplicationExt;
use cosmic::app::Settings;
use cosmic::app::{Core, Task};
use cosmic::executor;
use cosmic::iced::keyboard::{self, Key, key::Named};
use cosmic::iced::{Event, event, window};
use wicara_core::dialog::{DialogResult, Exit};

use crate::dialogs::list::SelectionMode;
use crate::dialogs::progress::ProgressEvent;
use crate::window::Message as ShellMessage;

impl cosmic::Application for Runner {
    type Executor = executor::Default;
    type Flags = DialogConfig;
    type Message = RunnerMessage;
    const APP_ID: &'static str = "com.cosmicwicara.Dialog";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, input: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = Runner::new(core, input);
        app.core.window.show_headerbar = false;
        app.core.window.show_close = false;
        app.core.window.show_minimize = false;
        app.core.window.show_maximize = false;
        app.core.window.content_container = false;
        let title = app.cfg.title.clone();
        let id = app.core.main_window_id();
        let title_cmd = if let Some(id) = id {
            app.set_window_title(title, id)
        } else {
            Task::none()
        };
        let blur_cmd = if let Some(id) = app.core.main_window_id() {
            window::enable_blur(id)
        } else {
            Task::none()
        };
        let mut task = Task::batch([title_cmd, blur_cmd]);
        if matches!(app.cfg.request, DialogRequest::File(_)) {
            task = Task::batch([task, file_task::file_task(app.cfg.clone())]);
        }
        (app, task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            RunnerMessage::Ok => self.on_ok(),
            RunnerMessage::Cancel | RunnerMessage::Close => self.on_cancel(),
            RunnerMessage::Tick => {
                if let Some(code) = self.shell.update(ShellMessage::Tick) {
                    let _ = code;
                    self.finish(DialogResult::bare(Exit::Timeout));
                }
            }
            RunnerMessage::ProgressTick => {
                let mut closed = false;
                if let Some(rx) = &self.progress_rx {
                    while let Ok(line) = rx.try_recv() {
                        if let Some(state) = &mut self.progress_state {
                            let event = state.apply_line(&line);
                            if matches!(event, ProgressEvent::Complete) {
                                if let DialogRequest::Progress(prog) = &self.cfg.request {
                                    if prog.auto_close && !prog.pulsate {
                                        self.finish(DialogResult::bare(Exit::Ok));
                                    }
                                }
                            }
                        }
                    }
                    if matches!(
                        rx.try_recv(),
                        Err(std::sync::mpsc::TryRecvError::Disconnected)
                    ) {
                        closed = true;
                    }
                }
                if closed {
                    self.finish(DialogResult::bare(Exit::Cancel));
                }
            }
            RunnerMessage::Extra(index) => {
                let label = self
                    .cfg
                    .extra_button
                    .get(index)
                    .cloned()
                    .unwrap_or_default();
                self.finish(DialogResult {
                    exit: Exit::Cancel,
                    output: Some(label),
                });
            }
            RunnerMessage::Key(enter) => {
                if enter {
                    self.on_ok();
                } else {
                    self.on_cancel();
                }
            }
            RunnerMessage::Input(value) => self.input = value,
            RunnerMessage::UsernameInput(value) => self.username = value,
            RunnerMessage::FormInput(index, value) => {
                if let DialogRequest::Forms(forms) = &mut self.cfg.request {
                    forms.set_value(index, value);
                }
            }
            RunnerMessage::CalendarSelect(date) => {
                if let Some(model) = &mut self.cal_model {
                    model.selected = date;
                    model.visible = date;
                }
            }
            RunnerMessage::CalendarPrev => {
                if let Some(model) = &mut self.cal_model {
                    model.show_prev_month();
                }
            }
            RunnerMessage::CalendarNext => {
                if let Some(model) = &mut self.cal_model {
                    model.show_next_month();
                }
            }
            RunnerMessage::ScaleChanged(value) => {
                self.scale_value = value;
                if let DialogRequest::Scale(scale) = &self.cfg.request {
                    if scale.print_partial {
                        self.scale_partials
                            .push(scale.snap(scale.clamp(value as i64)));
                    }
                }
            }
            RunnerMessage::ColorUpdate(update) => {
                if let Some(model) = &mut self.color_model {
                    return model.update(update);
                }
            }
            RunnerMessage::ListToggle(index) => {
                if let DialogRequest::List(list) = &self.cfg.request {
                    match list.mode {
                        SelectionMode::Checklist => {
                            if let Some(checked) = self.list_checked.get_mut(index) {
                                *checked = !*checked;
                            }
                            if self.list_checked.get(index).copied().unwrap_or(false) {
                                if !self.list_selected.contains(&index) {
                                    self.list_selected.push(index);
                                }
                            } else {
                                self.list_selected.retain(|&i| i != index);
                            }
                        }
                        SelectionMode::Radiolist => {
                            self.list_selected = vec![index];
                            self.list_checked = vec![false; list.row_count()];
                            if let Some(checked) = self.list_checked.get_mut(index) {
                                *checked = true;
                            }
                        }
                        _ => {
                            if list.multiple {
                                if self.list_selected.contains(&index) {
                                    self.list_selected.retain(|&i| i != index);
                                } else {
                                    self.list_selected.push(index);
                                }
                            } else {
                                self.list_selected = vec![index];
                            }
                        }
                    }
                }
            }
            RunnerMessage::ListEdit(index, value) => {
                if let DialogRequest::List(list) = &mut self.cfg.request {
                    let cols = list.columns.len().max(1);
                    if index * cols < list.rows.len() {
                        let sep = list.separator.clone();
                        let mut cells: Vec<String> = value.split(&sep).map(str::to_owned).collect();
                        while cells.len() < cols {
                            cells.push(String::new());
                        }
                        for (offset, cell) in cells.into_iter().take(cols).enumerate() {
                            list.rows[index * cols + offset] = cell;
                        }
                    }
                }
            }
            RunnerMessage::TextChecked(checked) => self.text_checked = checked,
            RunnerMessage::TextEdited(value) => self.text_edited = value,
            RunnerMessage::FileChosen(outcome) => match outcome {
                FileOutcome::Chosen(paths) => {
                    if let DialogRequest::File(sel) = &self.cfg.request {
                        let output = sel.format_output(&paths);
                        self.finish(DialogResult::ok(Some(output)));
                    }
                }
                FileOutcome::Cancelled => self.finish(DialogResult::bare(Exit::Cancel)),
                FileOutcome::Error(detail) => {
                    eprintln!("file-selection: {detail}");
                    self.finish(DialogResult::bare(Exit::Err255));
                }
            },
            RunnerMessage::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Surface(action));
            }
            RunnerMessage::WindowEvent(id, event) => {
                return self.present(id, event);
            }
        }
        Task::none()
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        let windows = window::events().map(|(id, event)| RunnerMessage::WindowEvent(id, event));
        let keys = event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Enter),
                ..
            }) => Some(RunnerMessage::Key(true)),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            }) => Some(RunnerMessage::Key(false)),
            _ => None,
        });
        let mut subs = vec![windows, keys];
        if self.shell.timeout.is_some() {
            subs.push(
                cosmic::iced::time::every(Duration::from_secs(1)).map(|_| RunnerMessage::Tick),
            );
        }
        if matches!(self.cfg.request, DialogRequest::Progress(_)) {
            subs.push(
                cosmic::iced::time::every(Duration::from_millis(100))
                    .map(|_| RunnerMessage::ProgressTick),
            );
        }
        cosmic::iced::Subscription::batch(subs)
    }

    fn view(&self) -> cosmic::Element<'_, Self::Message> {
        match &self.cfg.request {
            DialogRequest::Message => self.view_message(),
            _ => self.view_input(),
        }
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        None
    }
}

/// Run dialog, block.
pub fn run_dialog(cfg: DialogConfig) -> u8 {
    let (width, height) = cfg.shell().preferred_size(cfg.width, cfg.height);
    let height = if cfg.wants_taller() && height < 360 {
        360
    } else {
        height
    };
    let (w, h) = (width as f32, height as f32);
    let size_limits = cosmic::iced::core::layout::Limits::NONE
        .min_width(w)
        .max_width(w)
        .min_height(h)
        .max_height(h);

    let mut base_theme = cosmic::theme::system_preference();
    base_theme.transparent = true;

    let settings = Settings::default()
        .size(cosmic::iced::Size::new(w, h))
        .size_limits(size_limits)
        .resizable(None::<f64>)
        .theme(base_theme)
        .transparent(true);
    let result = cosmic::app::run::<Runner>(settings, cfg);
    match result {
        Ok(()) => 1,
        Err(e) => {
            eprintln!("cosmic-wicara: window error: {e}");
            255
        }
    }
}

#[cfg(test)]
mod kill_parent_tests {
    use crate::runner::state::parent_pid;

    #[test]
    fn parent_pid_is_sane() {
        #[cfg(unix)]
        assert!(parent_pid() > 0);
        #[cfg(not(unix))]
        assert_eq!(parent_pid(), 0);
    }
}

#[cfg(test)]
mod characterization_tests {
    use super::{DialogConfig, DialogRequest, Runner, RunnerMessage};
    use crate::dialogs::list::{List as ListDialog, SelectionMode};
    use cosmic::Application;
    use cosmic::app::Core;

    fn checklist_cfg() -> DialogConfig {
        let list = ListDialog::new(
            vec!["ID".to_owned(), "Name".to_owned()],
            vec![
                "1".to_owned(),
                "Andi".to_owned(),
                "2".to_owned(),
                "Budi".to_owned(),
                "3".to_owned(),
                "Citra".to_owned(),
            ],
        );
        let mut list = list;
        list.mode = SelectionMode::Checklist;
        DialogConfig {
            title: "T".to_owned(),
            body: String::new(),
            ok_label: "OK".to_owned(),
            cancel_label: "Cancel".to_owned(),
            extra_button: Vec::new(),
            switch: false,
            default_cancel: false,
            timeout: None,
            width: 400,
            height: 0,
            json: false,
            icon: None,
            ellipsize: false,
            no_wrap: false,
            request: DialogRequest::List(list),
        }
    }

    #[test]
    fn checklist_toggle_then_ok_output_lists_selected() {
        let mut runner = Runner::new(Core::default(), checklist_cfg());
        let _ = runner.update(RunnerMessage::ListToggle(1));
        assert_eq!(runner.list_selected, vec![1]);
        let _ = runner.update(RunnerMessage::ListToggle(1));
        assert!(
            runner.list_selected.is_empty(),
            "toggle off must clear selection"
        );
        let _ = runner.update(RunnerMessage::ListToggle(1));
        assert_eq!(runner.list_selected, vec![1]);
        assert_eq!(runner.ok_output(), Some("2|Budi".to_owned()));
    }

    #[test]
    fn out_of_bounds_list_toggle_is_noop() {
        let mut runner = Runner::new(Core::default(), checklist_cfg());
        let before = runner.list_selected.clone();
        let _ = runner.update(RunnerMessage::ListToggle(999));
        assert_eq!(
            runner.list_selected, before,
            "out-of-bounds toggle must be a no-op, not a panic"
        );
    }

    #[test]
    fn plain_list_single_mode_replaces_selection() {
        let mut cfg = checklist_cfg();
        if let DialogRequest::List(ref mut list) = cfg.request {
            list.mode = SelectionMode::Plain;
            list.multiple = false;
        }
        let mut runner = Runner::new(Core::default(), cfg);
        let _ = runner.update(RunnerMessage::ListToggle(0));
        let _ = runner.update(RunnerMessage::ListToggle(2));
        assert_eq!(
            runner.list_selected,
            vec![2],
            "single mode must replace selection"
        );
        assert_eq!(runner.ok_output(), Some("3|Citra".to_owned()));
    }
}
