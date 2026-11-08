//! Update tray applet.

use cosmic::Element;
use cosmic::app::{Core, Task};
use cosmic::iced::Rectangle;
use cosmic::iced::Subscription;
use cosmic::iced::core::window;
use cosmic::iced::window::Id;
use cosmic::surface::action::{app_popup, destroy_popup};

pub mod poll;
pub mod tray;

/// Applet application id.
pub const APP_ID: &str = "com.cosmicwicara.Applet";

/// Tray application.
pub struct Applet {
    /// Runtime core.
    pub core: Core,
    /// Current status text.
    pub status: tray::TrayStatus,
    /// Popup window id.
    pub popup: Option<Id>,
}

impl Default for Applet {
    fn default() -> Self {
        Self {
            core: Core::default(),
            status: tray::TrayStatus::Idle,
            popup: None,
        }
    }
}

/// Applet messages.
#[derive(Clone, Debug)]
pub enum Message {
    /// Popup closed.
    PopupClosed(Id),
    /// Poll tick.
    PollTick,
    /// Theme changed.
    ThemeMode,
    /// Surface action.
    Surface(cosmic::surface::Action<Message>),
}

impl cosmic::Application for Applet {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        (
            Self {
                core,
                status: tray::TrayStatus::Idle,
                popup: None,
            },
            Task::none(),
        )
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup == Some(id) {
                    self.popup = None;
                }
            }
            Message::PollTick => {
                self.status = match poll::run_checkupdates() {
                    Ok(pending) => tray::TrayStatus::from_pending(&pending),
                    Err(message) => tray::TrayStatus::error(message),
                };
            }
            Message::ThemeMode => {}
            Message::Surface(action) => {
                return cosmic::task::message(cosmic::Action::Surface(action));
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let theme = tray::subscribe_theme_mode(&self.core).map(|_| Message::ThemeMode);
        let poll = cosmic::iced::time::every(tray::poll_duration()).map(|_| Message::PollTick);
        Subscription::batch([theme, poll])
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup;
        let handle: cosmic::widget::icon::Handle = wicara_core::icons::update().into();
        let tooltip_text = self.status.tooltip();
        let btn = self
            .core
            .applet
            .icon_button_from_handle(handle)
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<Applet>(
                        |_| Default::default(),
                        move |state: &mut Applet| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let anchor = state.core.main_window_id().unwrap_or(new_id);
                            let mut popup_settings = state
                                .core
                                .applet
                                .get_popup_settings(anchor, new_id, None, None, None);

                            popup_settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };

                            popup_settings
                        },
                        Some(Box::new(move |state: &Applet| {
                            Element::from(state.core.applet.popup_container(state.popup_content()))
                                .map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        Element::from(self.core.applet.applet_tooltip::<Message>(
            btn,
            tooltip_text,
            self.popup.is_some(),
            Message::Surface,
            None,
        ))
    }

    fn view_window(&self, _id: window::Id) -> Element<'_, Message> {
        cosmic::widget::text::body("").into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl Applet {
    /// Update popup content.
    fn popup_content(&self) -> Element<'_, Message> {
        let mut content = cosmic::widget::column![cosmic::widget::text::body("Software updates")];
        match &self.status {
            tray::TrayStatus::Idle => {
                content = content.push(cosmic::widget::text::body("No updates"));
                content = content.push(cosmic::widget::text::body("Checked with checkupdates."));
            }
            tray::TrayStatus::Updates { count, lines } => {
                content = content.push(cosmic::widget::text::body(format!(
                    "{count} updates available"
                )));
                for line in lines.iter().take(tray::MAX_POPUP_LINES) {
                    content = content.push(cosmic::widget::text::body(line.clone()));
                }
            }
            tray::TrayStatus::Error { message } => {
                content = content.push(cosmic::widget::text::body(format!(
                    "Check failed: {message}"
                )));
            }
        }
        content.spacing(8).into()
    }
}

#[cfg(not(test))]
fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Applet>(())
}
