use std::error::Error as StdError;
use std::process::exit;

use crate::config::{app::WayMacConfig, toml::TomlConfig};
use iced::{Element, Task};
use iced_core::theme::Style;

use iced::Color;

#[cfg(target_os = "linux")]
use iced_layershell::{
    Settings, application,
    reexport::{Anchor, KeyboardInteractivity, Layer::Top},
    settings::{LayerShellSettings, StartMode},
};

use iced::{Renderer, Theme};

use crate::{
    Args,
    common::{LauncherState, Message, boot, subscription, update, view},
};

use log::error;

pub struct WaylandApp;

#[cfg(target_os = "linux")]
// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn run(arg: &'static Args) -> Result<(), Box<dyn StdError>> {
        //For knowing in which screen to output

        let binded_output_name = std::env::args().nth(1);
        let start_mode = match binded_output_name {
            Some(output) => StartMode::TargetScreen(output),
            None => StartMode::Active,
        };

        let toml_config = TomlConfig::from_path(arg.config_path.as_str());

        //TODO: implement a big parsing config function

        let bg_image_path = toml_config.main_window.background_image.clone();

        // just for fields that can be copy
        let config = match WayMacConfig::parse_from_toml(toml_config) {
            Ok(config) => config,
            Err(err) => {
                error!("Error: {err:?}");
                exit(1);
            }
        };

        application(
            move || boot(&config, &bg_image_path),
            WaylandApp::namespace,
            WaylandApp::update,
            WaylandApp::view::<Theme, Renderer>,
        )
        .subscription(subscription)
        //TODO: pass launcher config to the layer settings
        .settings(Settings {
            layer_settings: LayerShellSettings {
                layer: Top,
                size: Some((
                    config.main_window.size.width as u32,
                    config.main_window.size.height as u32,
                )),
                anchor: Anchor::Left | Anchor::Right,
                keyboard_interactivity: KeyboardInteractivity::Exclusive,
                start_mode,
                events_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .style(|status, theme| Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        // this one
        // are just for debugging
        .run()?;

        Ok(())
    }

    fn namespace() -> String {
        String::from("XWay - Wayland App")
    }

    fn update(state: &mut LauncherState, msg: Message) -> Task<Message> {
        update(state, msg)
    }

    fn view<Theme, Renderer>(state: &LauncherState) -> Element<Message> {
        view::<Theme, Renderer>(state)
    }
}
