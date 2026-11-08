//! Dialog request and configuration types.

use std::path::PathBuf;

use cosmic::widget::color_picker::ColorPickerUpdate;
use jiff::civil::Date;

use crate::dialogs::calendar::Calendar;
use crate::dialogs::color_selection::ColorSelection;
use crate::dialogs::entry::Entry;
use crate::dialogs::file_selection::FileSelection;
use crate::dialogs::forms::Forms;
use crate::dialogs::list::List;
use crate::dialogs::password::Password;
use crate::dialogs::progress::Progress;
use crate::dialogs::scale::Scale;
use crate::dialogs::text_info::TextInfo;
use crate::window::App;

/// Dialog render request.
#[derive(Clone, Debug)]
pub enum DialogRequest {
    /// Message dialog.
    Message,
    /// Entry dialog.
    Entry(Entry),
    /// Password dialog.
    Password(Password),
    /// Calendar dialog.
    Calendar(Calendar),
    /// Scale dialog.
    Scale(Scale),
    /// Color dialog.
    Color(ColorSelection),
    /// File dialog.
    File(FileSelection),
    /// List dialog.
    List(List),
    /// Text-info dialog.
    TextInfo(TextInfo),
    /// Progress dialog.
    Progress(Progress),
    /// Forms dialog.
    Forms(Forms),
}

/// Runner messages.
#[derive(Clone, Debug)]
pub enum RunnerMessage {
    /// OK pressed.
    Ok,
    /// Cancel pressed.
    Cancel,
    /// Close requested.
    Close,
    /// Timeout countdown tick.
    Tick,
    /// Progress stdin tick.
    ProgressTick,
    /// Extra button pressed.
    Extra(usize),
    /// Enter or Escape.
    Key(bool),
    /// Entry text changed.
    Input(String),
    /// Username changed.
    UsernameInput(String),
    /// Form field changed.
    FormInput(usize, String),
    /// Date picked.
    CalendarSelect(Date),
    /// Previous month.
    CalendarPrev,
    /// Next month.
    CalendarNext,
    /// Slider value changed.
    ScaleChanged(f32),
    /// Picker update.
    ColorUpdate(ColorPickerUpdate),
    /// Row toggled.
    ListToggle(usize),
    /// Row text changed.
    ListEdit(usize, String),
    /// Checkbox toggled.
    TextChecked(bool),
    /// Text edited.
    TextEdited(String),
    /// File chosen.
    FileChosen(FileOutcome),
    /// Surface action.
    Surface(cosmic::surface::Action<RunnerMessage>),
    /// Window event.
    WindowEvent(cosmic::iced::window::Id, cosmic::iced::window::Event),
}

/// File chooser result.
#[derive(Clone, Debug)]
pub enum FileOutcome {
    /// Chosen paths.
    Chosen(Vec<PathBuf>),
    /// Chooser cancelled.
    Cancelled,
    /// Chooser failed.
    Error(String),
}

/// Dialog configuration.
#[derive(Clone, Debug)]
pub struct DialogConfig {
    /// Window title.
    pub title: String,
    /// Body text.
    pub body: String,
    /// OK button label.
    pub ok_label: String,
    /// Cancel button label.
    pub cancel_label: String,
    /// Extra button labels.
    pub extra_button: Vec<String>,
    /// Switch mode?
    pub switch: bool,
    /// Default-cancel mode?
    pub default_cancel: bool,
    /// Timeout seconds.
    pub timeout: Option<u64>,
    /// Preferred width.
    pub width: u32,
    /// Preferred height.
    pub height: u32,
    /// JSON output?
    pub json: bool,
    /// Icon name.
    pub icon: Option<String>,
    /// Ellipsize enabled?
    pub ellipsize: bool,
    /// No-wrap enabled?
    pub no_wrap: bool,
    /// Dialog to render.
    pub request: DialogRequest,
}

impl DialogConfig {
    /// Build shell state.
    pub(crate) fn shell(&self) -> App {
        let mut app = App::with_options(
            self.title.clone(),
            self.body.clone(),
            self.ok_label.clone(),
            self.cancel_label.clone(),
            self.timeout,
        );
        app.icon = self.icon.clone();
        app.ellipsize = self.ellipsize;
        app.no_wrap = self.no_wrap;
        app
    }

    /// Needs tall window?
    pub(crate) fn wants_taller(&self) -> bool {
        if matches!(self.request, DialogRequest::Message) {
            // Long message bodies need the tall minimum, otherwise the fixed
            // dialog window clips the Update/Nanti buttons off the bottom.
            self.body.lines().count() > 2 || self.body.len() > 80
        } else {
            true
        }
    }
}
