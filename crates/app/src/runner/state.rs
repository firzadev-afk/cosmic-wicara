//! Runner application state.

use std::io::Write;
use std::sync::mpsc::{Receiver, channel};
use std::time::Instant;

use cosmic::app::{Core, Task};
use cosmic::iced::window;
use cosmic::widget::calendar;
use cosmic::widget::color_picker::ColorPickerModel;
use jiff::civil::Date;
use wicara_core::dialog::DialogResult;

use super::config::{DialogConfig, DialogRequest};
use crate::dialogs::color_selection::ColorSelection;
use crate::dialogs::progress::ProgressState;
use crate::window::App;

/// Dialog application.
pub struct Runner {
    pub(crate) core: Core,
    pub(crate) shell: App,
    pub(crate) cfg: DialogConfig,
    /// Already presented?
    pub(crate) presented: bool,
    /// Shared input buffer.
    pub(crate) input: String,
    /// Username buffer.
    pub(crate) username: String,
    /// Calendar model.
    pub(crate) cal_model: Option<calendar::CalendarModel>,
    /// Slider value.
    pub(crate) scale_value: f32,
    /// Partial values echo.
    pub(crate) scale_partials: Vec<i64>,
    /// Color picker model.
    pub(crate) color_model: Option<ColorPickerModel>,
    /// Selected rows.
    pub(crate) list_selected: Vec<usize>,
    /// Checklist states.
    pub(crate) list_checked: Vec<bool>,
    /// Progress state.
    pub(crate) progress_state: Option<ProgressState>,
    /// Progress stdin reader.
    pub(crate) progress_rx: Option<Receiver<String>>,
    /// Progress start time.
    pub(crate) progress_started: Instant,
    /// Checkbox state.
    pub(crate) text_checked: bool,
    /// Editable text buffer.
    pub(crate) text_edited: String,
    /// Parent pid captured at startup (for --auto-kill).
    pub(crate) parent_pid: i32,
}

impl Runner {
    /// Create runner.
    pub fn new(core: Core, cfg: DialogConfig) -> Self {
        let shell = cfg.shell();
        let (input, username) = match &cfg.request {
            DialogRequest::Entry(entry) => (entry.value.clone(), String::new()),
            DialogRequest::Password(password) => {
                (password.secret.clone(), password.username_value.clone())
            }
            _ => (String::new(), String::new()),
        };
        let cal_model = match &cfg.request {
            DialogRequest::Calendar(cal) => {
                let date = Date::new(cal.year as i16, cal.month as i8, cal.day as i8)
                    .unwrap_or_else(|_| jiff::Zoned::now().date());
                Some(calendar::CalendarModel::new(date, date))
            }
            _ => None,
        };
        let scale_value = match &cfg.request {
            DialogRequest::Scale(scale) => scale.normalized_value() as f32,
            _ => 0.0,
        };
        let color_model = match &cfg.request {
            DialogRequest::Color(sel) => {
                let initial = sel
                    .color
                    .as_deref()
                    .and_then(|seed| ColorSelection::parse(seed).ok())
                    .map(|(r, g, b)| cosmic::iced::Color {
                        r: f32::from(r) / 255.0,
                        g: f32::from(g) / 255.0,
                        b: f32::from(b) / 255.0,
                        a: 1.0,
                    });
                Some(ColorPickerModel::new("HEX", "RGB", initial, initial))
            }
            _ => None,
        };
        let (progress_state, progress_rx) = match &cfg.request {
            DialogRequest::Progress(prog) => {
                let state = ProgressState::new(prog.percentage, prog.text.clone());
                let (tx, rx) = channel::<String>();
                std::thread::spawn(move || {
                    use std::io::BufRead;
                    let stdin = std::io::stdin();
                    for line in stdin.lock().lines() {
                        match line {
                            Ok(text) => {
                                if tx.send(text).is_err() {
                                    return;
                                }
                            }
                            Err(_) => return,
                        }
                    }
                });
                (Some(state), Some(rx))
            }
            _ => (None, None),
        };
        let list_checked = match &cfg.request {
            DialogRequest::List(list) => vec![false; list.row_count()],
            _ => Vec::new(),
        };
        let text_edited = match &cfg.request {
            DialogRequest::TextInfo(info) => info.text.clone(),
            _ => String::new(),
        };
        Self {
            core,
            shell,
            cfg,
            presented: false,
            input,
            username,
            cal_model,
            scale_value,
            scale_partials: Vec::new(),
            color_model,
            list_selected: Vec::new(),
            list_checked,
            progress_state,
            progress_rx,
            progress_started: Instant::now(),
            text_checked: false,
            text_edited,
            parent_pid: parent_pid(),
        }
    }

    /// Present window once.
    pub(crate) fn present(
        &mut self,
        id: window::Id,
        event: window::Event,
    ) -> Task<super::config::RunnerMessage> {
        if self.presented || !matches!(event, window::Event::Opened { .. }) {
            return Task::none();
        }
        self.presented = true;
        Task::batch([
            window::gain_focus(id),
            window::request_user_attention(id, Some(window::UserAttention::Critical)),
        ])
    }

    /// Print result, exit.
    pub(crate) fn finish(&self, result: DialogResult) -> ! {
        let mut stdout = std::io::stdout();
        if self.cfg.json {
            let _ = writeln!(stdout, "{}", result.to_json());
        } else if let Some(output) = &result.output {
            let _ = writeln!(stdout, "{output}");
        }
        let _ = stdout.flush();
        std::process::exit(result.exit.into_code() as i32);
    }
}

/// Parent pid of this process.
#[cfg(unix)]
pub(crate) fn parent_pid() -> i32 {
    // SAFETY: getppid has no preconditions and never fails.
    unsafe { libc::getppid() }
}

/// Parent pid of this process (unsupported off Unix).
#[cfg(not(unix))]
pub(crate) fn parent_pid() -> i32 {
    0
}

/// Kill parent process with SIGTERM (Unix only).
pub(crate) fn kill_parent(ppid: i32) {
    #[cfg(unix)]
    {
        if ppid > 1 {
            // SAFETY: kill with SIGTERM on a captured ppid; ESRCH is ignored by design.
            unsafe {
                libc::kill(ppid, libc::SIGTERM);
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = ppid;
        eprintln!("cosmic-wicara: --auto-kill is only supported on Unix");
    }
}
