use std::error::Error as StdError;

use iced::{Element, Task, Theme};
use iced_layershell::{
    Settings, application,
    reexport::Anchor,
    settings::{LayerShellSettings, StartMode},
};

use crate::common::{LauncherState, Message, update, view};

pub struct WaylandApp;

// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //FOr knowing in which screen to output
        let binded_output_name = std::env::args().nth(1);
        let start_mode = match binded_output_name {
            Some(output) => StartMode::TargetScreen(output),
            None => StartMode::Active,
        };

        //TODO: setup correctly for config take in mind
        application(
            LauncherState::default,
            WaylandApp::namespace,
            WaylandApp::update,
            WaylandApp::view,
        )
        //TODO: pass launcher config to the layer settings
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((0, 400)),
                exclusive_zone: 400,
                anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
                start_mode,
                ..Default::default()
            },
            ..Default::default()
        }) // this one
        // are just for debugging
        .run()?;

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
