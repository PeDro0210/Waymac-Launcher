use std::collections::HashMap;

use log::error;

use iced::{Element, window::Id as IcedId};

#[cfg(target_os = "linux")]
use iced_layershell::to_layer_message;
//TODO: refactor this in the future

/* GLOBAL UPDATE AND VIEW*/
pub fn update(state: &mut LauncherState, msg: Message) {
    todo!("General update function not implemeneted")
}

pub fn view(state: &LauncherState) -> Element<Message> {
    todo!("General view function not implemeneted")
}

//TODO: declare GlobalNamespace fields

#[derive(Default)]
pub struct LauncherState; // cause of the pattern that layer_shell uses, we need to declare an
// struct which get's in charge of most of our variables.

//TODO: added needed message requests
#[cfg_attr(target_os = "linux", to_layer_message(multi))]
#[derive(Debug)]
pub enum Message {}
