use std::future;
use std::process::exit;

use iced::advanced::widget::operation::focusable::{Count, count, find_focused};
use iced::keyboard::{self, key};
use iced::widget::{Id as IcedId, column, container, operation::focus, text, text_input};
use iced::widget::{keyed_column, scrollable, space};
use iced::{Element, Length, Subscription, Task, window};

use iced::{
    Event,
    event::{self, Status},
    keyboard::{Event::KeyPressed, Key, key::Named},
    window::Event::Opened,
};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
use log::{debug, info};

use crate::app_launcher::{DesktopEntry, get_desktop_entry};
use crate::data::{LAUNCHER_CONTAINER_ID, LAUNCHER_TEXT_INPUT_ID};
//TODO: refactor this in the future

/* GLOBAL UPDATE AND VIEW*/
pub fn update(state: &mut LauncherState, msg: Message) -> Task<Message> {
    //TODO: implement update function

    match msg {
        Message::DesktopEntriesFetched(desktop_entries) => {
            state.desktop_entries = Some(desktop_entries.clone());
            state.cached_desktop_entries = Some(desktop_entries);

            Task::none()
        }
        Message::DesktopEntriesChanged(new_desktop_entries) => {
            state.cached_desktop_entries = Some(new_desktop_entries);
            Task::none()
        }
        // this match statemente will get out of hands LMAO
        Message::UserInputChanged(user_input) => {
            state.user_input = user_input.clone();

            let desktop_entries_borrowed = state.desktop_entries.to_owned();

            Task::future(async move {
                let desktop_entries = desktop_entries_borrowed
                    .clone()
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|entry| {
                        entry
                            .name
                            .to_lowercase()
                            .contains(&user_input.to_lowercase().clone())
                    })
                    .collect();
                Message::DesktopEntriesChanged(desktop_entries)
            })
        }

        Message::UserInputFocus => {
            let _ = focus::<IcedId>(IcedId::new(LAUNCHER_TEXT_INPUT_ID));
            Task::none()
        }
        //TODO: implement correct key handleling
        Message::KeyboardEvent(key_event) => match key_event {
            KeyPressed { key, .. } => {
                match key {
                    Key::Named(Named::Escape) => {
                        exit(1);
                    }
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        },
        Message::OnOpen(win_event) => match win_event {
            Opened { .. } => Task::perform(get_desktop_entry(), Message::DesktopEntriesFetched),
            _ => Task::none(),
        },
        _ => Task::none(),
    }
}

//TODO: implement view function
pub fn view(state: &LauncherState) -> Element<Message> {
    container(column![
        //TODO: Separate launcher  widgets in different functions
        text_input("", &state.user_input)
            .on_input(Message::UserInputChanged)
            .id(LAUNCHER_TEXT_INPUT_ID),
        scrollable(column(
            state
                .cached_desktop_entries
                .as_ref()
                .unwrap_or(&mut Vec::new())
                .iter()
                .filter_map(|entry| {
                    if entry.name.is_empty() {
                        None
                    } else {
                        Some(text(entry.name.clone()).into())
                    }
                }),
        ))
        .width(Length::Fill)
    ])
    .id(LAUNCHER_CONTAINER_ID)
    .into()
}

pub fn boot() -> (LauncherState, Task<Message>) {
    (
        LauncherState::default(),
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
    user_input: String,
    desktop_entries: Option<Vec<DesktopEntry>>,
    cached_desktop_entries: Option<Vec<DesktopEntry>>,
} // cause of the pattern that layer_shell uses, we need to declare an
// struct which get's in charge of most of our variables.

#[cfg_attr(target_os = "linux", to_layer_message(multi))]
#[derive(Debug, Clone)]
pub enum Message {
    DesktopEntriesFetched(Vec<DesktopEntry>),
    DesktopEntriesChanged(Vec<DesktopEntry>),

    UserInputChanged(String),
    UserInputFocus,

    KeyboardEvent(iced::keyboard::Event),
    OnOpen(iced::window::Event),
}
