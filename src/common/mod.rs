use std::process::exit;
use std::thread::spawn;

use iced::widget::{Id as IcedId, column, container, operation::focus, text, text_input};
use iced::widget::{Text, scrollable};
use iced::{Color, Element, Length, Subscription, Task};

use iced::{
    event,
    keyboard::{Event::KeyPressed, Key, key::Named},
    window::Event::Opened,
};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
use log::info;

use crate::app_launcher::{DesktopEntry, get_desktop_entry};
use crate::data::{
    ENTRY_FOCUS_COLOR, LAUNCHER_CONTAINER_ID, LAUNCHER_SCROLLABLE_ID, LAUNCHER_TEXT_INPUT_ID,
    MAIN_ENTRY_FOCUS_IDX,
};
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
            state.filtering_cached_entry = false;
            state.cached_desktop_entries = Some(new_desktop_entries);

            Task::done(Message::FocusDesktopEntry(MAIN_ENTRY_FOCUS_IDX))
        }
        // this match statemente will get out of hands LMAO
        Message::UserInputChanged(user_input) => {
            state.user_input = user_input.clone();

            let desktop_entries_borrowed = state.desktop_entries.to_owned();

            info!("filted_cached_entry: {}", state.filtering_cached_entry);

            let desktop_filter_join = spawn(move || {
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
            });

            if !state.filtering_cached_entry {
                state.filtering_cached_entry = true;
                return Task::future(async move { desktop_filter_join.join().unwrap() });
            }

            Task::none()
        }

        Message::UserInputFocus => {
            let _ = focus::<IcedId>(IcedId::new(LAUNCHER_TEXT_INPUT_ID));
            Task::none()
        }

        Message::FocusDesktopEntry(key) => {
            // get the specific DesktopEntry and copying it
            let mut selected_entry = state.cached_desktop_entries.as_ref().unwrap().get(key);

            if let Some(entry) = selected_entry {
                let mut entry_owned = entry.to_owned();
                entry_owned.is_focus = true;

                info!("entry is: {selected_entry:?}");
                let mut desktop_entries_with_focus_owned =
                    state.cached_desktop_entries.to_owned().unwrap();

                desktop_entries_with_focus_owned[key] = entry_owned.clone();

                state.cached_desktop_entries = Some(desktop_entries_with_focus_owned);
            }
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
                            if key == "p" {}
                            if key == "n" {}
                        }
                        _ => {}
                    }
                }
                match key {
                    Key::Named(Named::Escape) => {
                        exit(1);
                    }
                    Key::Named(Named::Backspace) => {
                        // for not looking at the user_input while deleating
                        info!("ignoring input backspace");
                        return Task::none();
                    }
                    _ => {}
                }
                Task::none()
            }
            _ => Task::none(),
        },
        Message::OnOpen(win_event) => match win_event {
            Opened { .. } => Task::batch(vec![
                Task::perform(get_desktop_entry(), Message::DesktopEntriesFetched),
                Task::done(Message::FocusDesktopEntry(MAIN_ENTRY_FOCUS_IDX)),
            ]),
            _ => Task::none(),
        },
        _ => Task::none(),
    }
}

//TODO: implement view function
pub fn view<Theme, Renderer>(state: &LauncherState) -> Element<'_, Message> {
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
                    let desktop_entry_text: Text = text(entry.name.clone()).into();

                    if entry.is_focus {
                        return Some(desktop_entry_text.color(ENTRY_FOCUS_COLOR).into());
                    }

                    Some(desktop_entry_text.into())
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
    filtering_cached_entry: bool,
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

    FocusDesktopEntry(usize),
}
