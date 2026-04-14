use std::error::Error as StdError;

use iced::{Element, Task, Theme};
use iced_layershell::application;

use crate::common::{LauncherState, Message, update, view};

pub struct WaylandApp;

// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        application(
            || LauncherState::default(),
            WaylandApp::namespace,
            WaylandApp::update,
            WaylandApp::view,
        )
        .run();

        Ok(())
    }

    fn namespace() -> String {
        String::from("XWay - Wayland App")
    }

    fn update(state: &mut LauncherState, msg: Message) -> Task<Message> {
        update(state, msg);
        Task::none()
    }

    fn view(state: &LauncherState) -> Element<Message> {
        view(state)
    }
}
