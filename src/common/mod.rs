use std::process::exit;

use iced::advanced::widget::operation::focusable::{Count, count};
use iced::keyboard::{self, key};
use iced::widget::{Id as IcedId, column, container, operation::focus, text, text_input};
use iced::{Element, Subscription, Task};

use iced::{
    Event,
    event::{self, Status},
    keyboard::{Event::KeyPressed, Key, key::Named},
};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
use log::info;

use crate::data::{LAUNCHER_CONTAINER_ID, LAUNCHER_TEXT_INPUT_ID};
//TODO: refactor this in the future

/* GLOBAL UPDATE AND VIEW*/
pub fn update(state: &mut LauncherState, msg: Message) -> Task<Message> {
    //TODO: implement update function

    match msg {
        // this match statemente will get out of hands LMAO
        Message::UserInputChanged(user_input) => {
            state.user_input = user_input;
            Task::none()
        }
        Message::UserInputFocus => {
            focus::<IcedId>(IcedId::new(LAUNCHER_TEXT_INPUT_ID));
            Task::none()
        }
        Message::KeyboardEvent(key_event) => match key_event {
            KeyPressed { key, .. } => {
                info!("key pressed{key:?}");
                if key == Key::Character("q".into()) {
                    // trying to exit through iced::exit (didn't worked btw)
                    exit(1);
                } else {
                    Task::none()
                }
            }
            _ => Task::none(),
        },
        _ => Task::none(),
    }
}

pub fn view(state: &LauncherState) -> Element<Message> {
    //TODO: implement view function
    container(column![
        text_input("", &state.user_input)
            .on_input(Message::UserInputChanged)
            .id(LAUNCHER_TEXT_INPUT_ID),
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
        _ => None,
    })
}

//TODO: declare LauncherState fields
#[derive(Default)]
pub struct LauncherState {
    user_input: String,
} // cause of the pattern that layer_shell uses, we need to declare an
// struct which get's in charge of most of our variables.

//TODO: added needed message requests
#[cfg_attr(target_os = "linux", to_layer_message(multi))]
#[derive(Debug, Clone)]
pub enum Message {
    UserInputChanged(String),
    UserInputFocus,
    KeyboardEvent(iced::keyboard::Event),
}
