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

            Task::done(Message::ToogleFocusDesktopEntry(
                state.focus_desktop_entry_id,
                true,
            ))
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

        Message::ToogleFocusDesktopEntry(key, focus) => {
            // get the specific DesktopEntry and copying it
            let mut selected_entry = state.cached_desktop_entries.as_ref().unwrap().get(key);

            if let Some(entry) = selected_entry {
                let mut entry_owned = entry.to_owned();
                entry_owned.is_focus = focus;

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
                let limit_entry_id = |state_clone: &LauncherState, offset: i32| match (state
                    .focus_desktop_entry_id
                    as i32
                    + offset)
                {
                    val if val < 0 => state
                        .cached_desktop_entries
                        .clone()
                        .unwrap_or(Vec::new())
                        .len(),
                    val if val
                        > state
                            .cached_desktop_entries
                            .clone()
                            .unwrap_or(Vec::new())
                            .len() as i32 =>
                    {
                        0
                    }
                    _ => (state.focus_desktop_entry_id as i32 + offset) as usize,
                };

                //TODO: implement modifier keys
                // for managing different modifiers
                if modifiers.control() {
                    match key.clone() {
                        Key::Character(key) => {
                            // Dunno how convienent is being DRY in here, like I don't want just by
                            // pressing the moddifier to pass thorugh all this process
                            if key == "n" {
                                let old_focus_desktop_entry_id = state.focus_desktop_entry_id;
                                state.focus_desktop_entry_id = limit_entry_id(state, 1);

                                info!("entry num: {}", state.focus_desktop_entry_id);

                                return Task::batch(vec![
                                    Task::done(Message::ToogleFocusDesktopEntry(
                                        old_focus_desktop_entry_id,
                                        false,
                                    )),
                                    Task::done(Message::ToogleFocusDesktopEntry(
                                        state.focus_desktop_entry_id,
                                        true,
                                    )),
                                ]);
                            }
                            if key == "p" {
                                let old_focus_desktop_entry_id = state.focus_desktop_entry_id;
                                state.focus_desktop_entry_id = limit_entry_id(state, -1);

                                info!("entry num: {}", state.focus_desktop_entry_id);

                                return Task::batch(vec![
                                    Task::done(Message::ToogleFocusDesktopEntry(
                                        old_focus_desktop_entry_id,
                                        false,
                                    )),
                                    Task::done(Message::ToogleFocusDesktopEntry(
                                        state.focus_desktop_entry_id,
                                        true,
                                    )),
                                ]);
                            }
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
                Task::done((|| {
                    state.focus_desktop_entry_id = MAIN_ENTRY_FOCUS_IDX;
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
    focus_desktop_entry_id: usize,
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

    ToogleFocusDesktopEntry(usize, bool),
}
