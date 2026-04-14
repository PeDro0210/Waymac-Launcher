use std::collections::HashMap;

use iced::{Element, window::Id as IcedId};

use iced_layershell::to_layer_message;
//TODO: refactor this in the future

/* GLOBAL UPDATE AND VIEW*/
pub fn update(state: &mut GlobalState, msg: Message) {
    todo!()
}
pub fn view(state: &GlobalState, id: IcedId) -> Element<Message> {
    todo!()
}

//TODO: declare GlobalNamespace fields

#[derive(Default)]
pub struct GlobalState; // cause of the pattern that layer_shell uses, we need to declare an
// struct which get's in charge of most of our variables.

//TODO: added needed message requests
#[cfg_attr(target_os = "linux", to_layer_message(multi))]
#[derive(Debug)]
pub enum Message {}
