use iced::widget::{Id as IcedId, button, column, container, operation::focus, text, text_input};
use iced::{Element, Task};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;

use crate::data::LAUNCHER_TEXT_INPUT_ID;
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
        _ => Task::none(),
    }
}

pub fn view(state: &LauncherState) -> Element<Message> {
    //TODO: implement view function
    container(column![
        text_input("", &state.user_input)
            .on_input(Message::UserInputChanged)
            .id(LAUNCHER_TEXT_INPUT_ID)
    ])
    .into()
}

pub fn boot() -> (LauncherState, Task<Message>) {
    (
        LauncherState::default(),
        focus(IcedId::new(LAUNCHER_TEXT_INPUT_ID)),
    )
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
    UserInputFocus(IcedId),
}
