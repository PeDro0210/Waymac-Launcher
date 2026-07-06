use std::process::exit;
use std::thread::spawn;

use iced::Length::Fill;
use iced::widget::container::Style;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text_input::Style as TextInputStyle;
use iced::widget::text_input::default as text_input_default;
use iced::widget::{Id as IcedId, column, container, operation::focus, text, text_input};
use iced::widget::{Text, scrollable};
use iced::{Background, Border, Color, Element, Length, Size, Subscription, Task};

use iced::{
    event,
    keyboard::{Event::KeyPressed, Key, key::Named},
    window::Event::Opened,
};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
use log::{error, info, trace};

use crate::app_launcher::{DesktopEntry, get_desktop_entry, launch_application};
use crate::common::util::change_focus;
use crate::config::app::{ContainerType, WayMacConfig};
use crate::data::{
    LAUNCHER_CONTAINER_ID, LAUNCHER_SCROLLABLE_ID, LAUNCHER_TEXT_INPUT_ID, MAIN_ENTRY_FOCUS_IDX,
};

mod util;
//TODO: refactor this in the future

/* GLOBAL UPDATE AND VIEW*/
pub fn update(state: &mut LauncherState, msg: Message) -> Task<Message> {
    //TODO: implement update function

    match msg {
        Message::DesktopEntriesFetched(desktop_entries) => {
            state.desktop_entries = Some(desktop_entries.to_vec());
            state.cached_desktop_entries = Some(desktop_entries);

            state.ui_desktop_entries = state.cached_desktop_entries.clone();

            Task::none()
        }
        Message::DesktopEntriesChanged(new_desktop_entries) => {
            state.filtering_cached_entry = false;
            state.cached_desktop_entries = Some(new_desktop_entries);

            Task::done(Message::ToogleFocusDesktopEntry(
                state.focus_desktop_entry_id,
                true,
            ))
        }
        // this match statemente will get out of hands LMAO
        Message::UserInputChanged(user_input) => {
            state.user_input = user_input.clone();

            let desktop_entries_borrowed = state.desktop_entries.to_owned();

            let desktop_filter_join = spawn(move || {
                let desktop_entries = Box::new(
                    desktop_entries_borrowed
                        .unwrap_or_default()
                        .into_iter()
                        .filter(|entry| {
                            entry
                                .name
                                .to_lowercase()
                                .contains(&user_input.to_lowercase().clone())
                        })
                        .collect(),
                );
                Message::DesktopEntriesChanged(desktop_entries)
            });

            if !state.filtering_cached_entry {
                state.filtering_cached_entry = true;
                state.focus_desktop_entry_id = MAIN_ENTRY_FOCUS_IDX;
                return Task::future(async move { desktop_filter_join.join().unwrap() });
            }

            Task::none()
        }

        Message::UserInputFocus => {
            let _ = focus::<IcedId>(IcedId::new(LAUNCHER_TEXT_INPUT_ID));
            Task::none()
        }

        //TODO: make that the selected entry just get's reference and doesn't copy all of them
        Message::ToogleFocusDesktopEntry(key, focus) => {
            // get the specific DesktopEntry and copying it
            let selected_entry = state.cached_desktop_entries.as_ref().unwrap().get(key);

            if let Some(entry) = selected_entry {
                let mut entry_owned = entry.to_owned();
                entry_owned.is_focus = focus;

                state.cached_desktop_entries.as_mut().unwrap()[key] = entry_owned;
            }

            state.ui_desktop_entries = state.cached_desktop_entries.clone();

            Task::none()
        }

        //TODO: implement correct key handleling
        Message::KeyboardEvent(key_event) => match key_event {
            KeyPressed { key, modifiers, .. } => {
                //TODO: implement modifier keys
                // for managing different modifiers
                if modifiers.control() {
                    match key.clone() {
                        Key::Character(key) => {
                            // pressing the moddifier to pass thorugh all this process
                            if key == "n" {
                                return change_focus(state, 1);
                            }
                            if key == "p" {
                                return change_focus(state, -1);
                            }
                        }

                        _ => {}
                    }
                }
                match key {
                    Key::Named(Named::Escape) => {
                        exit(1);
                    }
                    Key::Named(Named::Enter) => {
                        let selected_entry = state
                            .ui_desktop_entries
                            .as_ref()
                            .unwrap()
                            .get(state.focus_desktop_entry_id);

                        if let Some(selected_entry) = selected_entry {
                            let _ = launch_application(selected_entry);
                        }
                    }
                    Key::Named(Named::Backspace) => {
                        // for not looking at the user_input while deleating
                        trace!("ignoring input backspace");
                        return Task::none();
                    }
                    Key::Named(Named::ArrowUp) => {
                        return change_focus(state, -1);
                    }
                    Key::Named(Named::ArrowDown) => {
                        return change_focus(state, 1);
                    }
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        },
        Message::OnOpen(win_event) => match win_event {
            Opened { size, .. } => Task::batch(vec![
                Task::perform(get_desktop_entry(), Message::DesktopEntriesFetched),
                Task::done((|| {
                    state.focus_desktop_entry_id = MAIN_ENTRY_FOCUS_IDX;
                    state.window_size = size;
                    Message::ToogleFocusDesktopEntry(MAIN_ENTRY_FOCUS_IDX, true)
                })()),
            ]),
            _ => Task::none(),
        },

        _ => Task::none(),
    }
}

//TODO: implement view function
pub fn view<Theme, Renderer>(state: &LauncherState) -> Element<'_, Message> {
    container(
        // TODO: implement a stack for having image/color as base-layer and the rest above
        container(column![
            //TODO: Separate launcher  widgets in different functions
            text_input("", &state.user_input)
                .on_input(Message::UserInputChanged)
                .id(LAUNCHER_TEXT_INPUT_ID)
                .line_height(state.config.input_bar.size.height)
                .width(state.config.input_bar.size.width)
                .font(state.config.input_bar.font)
                .style(|theme, status| TextInputStyle {
                    background: (|| match state.config.input_bar.background {
                        Some(bg) => bg,
                        None => Background::Color(Color::default()),
                    })(),
                    border: (|| match state.config.input_bar.border {
                        Some(bor) => bor,
                        None => Border {
                            ..Default::default()
                        },
                    })(),
                    value: state.config.input_bar.text_color,
                    ..text_input_default(theme, status)
                }),
            scrollable(column(
                state
                    .ui_desktop_entries
                    .as_ref()
                    .unwrap_or(&mut Box::new(Vec::new()))
                    .iter()
                    .filter_map(|entry| {
                        let desktop_entry_text: Text = text(entry.name.clone())
                            .font(state.config.entry.font)
                            .height(Length::Fixed(state.config.entry.size.height))
                            .into();

                        if entry.is_focus {
                            return Some(
                                desktop_entry_text
                                    .color(match state.config.entry.specific {
                                        ContainerType::Entry {
                                            focus_text_color, ..
                                        } => focus_text_color,
                                        _ => {
                                            error!("Error while doing specific container type");
                                            exit(1);
                                        }
                                    })
                                    .font(state.config.entry.font)
                                    .into(),
                            );
                        }

                        Some(
                            desktop_entry_text
                                .color(state.config.entry.text_color)
                                .into(),
                        )
                    }),
            ))
            .spacing(match state.config.main_window.specific {
                ContainerType::MainWindow { spacing, .. } => spacing,
                _ => {
                    error!("Error while doing specific container type");
                    exit(1);
                }
            })
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .id(LAUNCHER_SCROLLABLE_ID)
            .width(Length::Fill)
        ])
        .id(LAUNCHER_CONTAINER_ID)
        .width(state.config.main_window.size.width)
        .height(state.config.main_window.size.height)
        .padding(match state.config.main_window.specific {
            ContainerType::MainWindow { padding, .. } => padding,
            _ => {
                error!("Error while doing specific container type");
                exit(1);
            }
        })
        //TODO: make text_color being exchangble for the toml config
        .style(|_| Style {
            background: state.config.main_window.background,
            border: (|| match state.config.main_window.border {
                Some(bor) => bor,
                None => Border {
                    ..Default::default()
                },
            })(),
            ..Default::default()
        }),
    )
    .width(Fill)
    .height(Fill)
    .center_x(state.window_size.width)
    .center_y(state.window_size.height)
    .into()
}

pub fn boot(config: &WayMacConfig) -> (LauncherState, Task<Message>) {
    (
        LauncherState {
            config: *config,
            ..Default::default()
        },
        focus(IcedId::new(LAUNCHER_TEXT_INPUT_ID)),
    )
}

pub fn subscription(_: &LauncherState) -> Subscription<Message> {
    event::listen_with(|event, _status, _id| match event {
        iced::Event::Keyboard(k) => Some(Message::KeyboardEvent(k)),
        iced::Event::Window(e) => Some(Message::OnOpen(e)),
        _ => None,
    })
}

//TODO: declare LauncherState fields
#[derive(Default)]
pub struct LauncherState {
    config: WayMacConfig, // will neve go for the default (TAKE THAT IN MIND)
    user_input: String,
    filtering_cached_entry: bool,
    focus_desktop_entry_id: usize,
    desktop_entries: Option<Vec<DesktopEntry>>,
    cached_desktop_entries: Option<Box<Vec<DesktopEntry>>>,
    ui_desktop_entries: Option<Box<Vec<DesktopEntry>>>,
    window_size: Size,
} // cause of the pattern that layer_shell uses, we need to declare an
// struct which get's in charge of most of our variables.

#[cfg_attr(target_os = "linux", to_layer_message(multi))]
#[derive(Debug, Clone)]
pub enum Message {
    DesktopEntriesFetched(Box<Vec<DesktopEntry>>),
    DesktopEntriesChanged(Box<Vec<DesktopEntry>>),

    UserInputChanged(String),
    UserInputFocus,

    KeyboardEvent(iced::keyboard::Event),
    OnOpen(iced::window::Event),

    ToogleFocusDesktopEntry(usize, bool),
}
