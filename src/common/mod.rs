mod containers;
mod util;

use std::process::exit;
use std::thread::spawn;

use iced::Length;
use iced::advanced::graphics::mesh::SolidVertex2D;
use iced::overlay::menu::Catalog;
use iced::widget::container::Style;
use iced::widget::{Id as IcedId, operation::focus};
use iced::widget::{container, operation};
use iced::window::Settings;
use iced::{Color, Window};
use iced::{Element, Padding, Size, Subscription, Task};

use iced::{
    event,
    keyboard::{Event::KeyPressed, Key, key::Named},
    window,
    window::Event::Opened,
};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
use log::{debug, trace};

use crate::app_launcher::{DesktopEntry, get_desktop_entry, launch_application};
use crate::common::util::{change_focus, location_mapping};
use crate::config::app::{ContainerType, WayMacConfig};
use crate::config::toml::MainWindow;
use crate::data::{LAUNCHER_TEXT_INPUT_ID, MAIN_ENTRY_FOCUS_IDX};
use crate::error_collector::WaymacErrorVector;

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
                        if state.errors_detected.len() > 0 {
                            return Task::done(Message::OnError);
                        }

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
        Message::WindowOpened(id) => {
            if state.errors_detected.len() > 0 {
                return Task::done(Message::OnError);
            }

            Task::batch(vec![
                Task::perform(get_desktop_entry(), Message::DesktopEntriesFetched),
                focus(IcedId::new(LAUNCHER_TEXT_INPUT_ID)),
                Task::done((|| {
                    state.focus_desktop_entry_id = MAIN_ENTRY_FOCUS_IDX;

                    let mut monitor_size: (f32, f32) = (0., 0.);

                    let _ = window::monitor_size(id).map(move |monitor_size_closure| {
                        match monitor_size_closure {
                            Some(size) => {
                                monitor_size.0 = size.height;
                                monitor_size.1 = size.width;
                            }
                            None => {}
                        }
                    });

                    debug!("MONITOR_SIZE: {} {}", monitor_size.0, monitor_size.1);

                    // just a dummy for having a size for the meantime
                    state.window_size = Size {
                        height: monitor_size.0,
                        width: monitor_size.1,
                    };

                    state.window_padding_for_margins = Some(location_mapping(
                        match state.config.main_window.specific {
                            ContainerType::MainWindow { location, .. } => location,
                            _ => {
                                panic!("Can't happen")
                            }
                        },
                        state.window_size,
                        match state.config.main_window.specific {
                            ContainerType::MainWindow { margin, .. } => margin,
                            _ => {
                                panic!("Can't happen")
                            }
                        },
                        state.config.main_window.size,
                    ));
                    Message::ToogleFocusDesktopEntry(MAIN_ENTRY_FOCUS_IDX, true)
                })()),
            ])
        }

        _ => Task::none(),
    }
}

//TODO: implement componenent in case of error
pub fn view<Theme, Renderer>(state: &LauncherState, id: window::Id) -> Element<'_, Message> {
    if state.errors_detected.len() > 0 {
        return containers::error::view::<Theme, Renderer>(state); // it won't even parse the margin
    }

    // depending on target
    #[cfg(target_os = "linux")]
    return container(containers::app_launcher::view::<Theme, Renderer>(state))
        .padding(match state.config.main_window.specific {
            ContainerType::MainWindow { margin, .. } => margin,
            _ => {
                panic!("Can't happen")
            }
        })
        .into();

    #[cfg(target_os = "macos")]
    match state.window_padding_for_margins {
        Some(margins) => container(containers::app_launcher::view::<Theme, Renderer>(state))
            .padding(margins)
            .into(),
        // takes a bit longer loading, cause the widget loads first, but without dimentions so it
        // doesn't appear on screen, while waiting for the margins assignment
        None => container(containers::app_launcher::view::<Theme, Renderer>(state))
            .padding(Padding::default())
            .width(0)
            .height(0)
            .into(),
    }
}

// different opens
//TODO: accept the big config with the static size var and the dynamic
#[cfg(target_os = "macos")]
pub fn boot(
    config: &WayMacConfig,
    bg_img_path: &Option<String>,
    errors_detected: WaymacErrorVector,
) -> (LauncherState, Task<Message>) {
    let (_, open) = window::open(Settings {
        level: window::Level::AlwaysOnTop,
        decorations: false,
        transparent: true,
        resizable: false,
        ..Default::default()
    });
    (
        LauncherState {
            config: *config,
            bg_image_path: bg_img_path.clone(),
            errors_detected,
            ..Default::default()
        },
        open.map(Message::WindowOpened),
    )
}

#[cfg(target_os = "linux")]
pub fn boot(
    config: &WayMacConfig,
    bg_img_path: &Option<String>,
    errors_detected: WaymacErrorVector,
) -> (LauncherState, Task<Message>) {
    (
        LauncherState {
            config: *config,
            bg_image_path: bg_img_path.clone(),
            errors_detected,
            ..Default::default()
        },
        Task::done(Message::WindowOpened(iced::window::Id::unique())),
    )
}
pub fn subscription(_: &LauncherState) -> Subscription<Message> {
    event::listen_with(|event, _status, _id| match event {
        iced::Event::Keyboard(k) => Some(Message::KeyboardEvent(k)),
        _ => None,
    })
}

//TODO: declare LauncherState fields
#[derive(Default)]
pub struct LauncherState {
    config: WayMacConfig, // will never go for the default (TAKE THAT IN MIND)
    bg_image_path: Option<String>,
    user_input: String,
    filtering_cached_entry: bool,
    focus_desktop_entry_id: usize,
    desktop_entries: Option<Vec<DesktopEntry>>,
    cached_desktop_entries: Option<Box<Vec<DesktopEntry>>>,
    ui_desktop_entries: Option<Box<Vec<DesktopEntry>>>,
    errors_detected: WaymacErrorVector,
    window_padding_for_margins: Option<Padding>, //Just for MACOS target
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
    WindowOpened(window::Id),

    ToogleFocusDesktopEntry(usize, bool),

    OnError,
}
