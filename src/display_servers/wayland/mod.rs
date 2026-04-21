use std::error::Error as StdError;

use iced::{Element, Task};

#[cfg(target_os = "linux")]
use iced_layershell::{
    Settings, application,
    reexport::{Anchor, KeyboardInteractivity, Layer::Top},
    settings::{LayerShellSettings, StartMode},
};

use crate::common::{LauncherState, Message, boot, subscription, update, view};

pub struct WaylandApp;

#[cfg(target_os = "linux")]
// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //For knowing in which screen to output

        let binded_output_name = std::env::args().nth(1);
        let start_mode = match binded_output_name {
            Some(output) => StartMode::TargetScreen(output),
            None => StartMode::Active,
        };

        //TODO: setup correctly for config take in mind
        application(
            boot,
            WaylandApp::namespace,
            WaylandApp::update,
            WaylandApp::view,
        )
        .subscription(subscription)
        //TODO: pass launcher config to the layer settings
        .settings(Settings {
            layer_settings: LayerShellSettings {
                layer: Top,
                size: Some((350, 350)),
                exclusive_zone: 350,
                anchor: Anchor::Left | Anchor::Right,
                keyboard_interactivity: KeyboardInteractivity::Exclusive,
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
