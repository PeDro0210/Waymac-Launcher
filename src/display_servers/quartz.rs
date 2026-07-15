use crate::config::app::WayMacConfig;
use std::error::Error as StdError;

use iced::{Color, Size, application, theme::Style, window::Level::AlwaysOnTop};

#[cfg(target_os = "macos")]
use crate::Args;
use crate::config::toml::TomlConfig;

use crate::common::{boot, subscription, update, view};
use iced::{Renderer, Theme};

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;
use log::error;

pub struct QuartzApp;

#[cfg(target_os = "macos")]
impl QuartzApp {
    //TODO: pass args
    pub fn run(arg: &'static Args) -> Result<(), Box<dyn StdError>> {
        let display_pre_info = CGDisplay::main();

        let toml_config = TomlConfig::from_path(arg.config_path.as_str());

        let bg_image_path = toml_config.main_window.background_image.clone();

        let config = match WayMacConfig::parse_from_toml(toml_config) {
            Ok(config) => config,
            Err(err) => {
                use std::process::exit;

                error!("Error: {err:?}");
                exit(1);
            }
        };

        //TODO: setup correctly for config take in mind
        application(
            move || boot(&config, &bg_image_path),
            update,
            view::<Theme, Renderer>,
        )
        .decorations(false)
        .window_size(Size {
            width: display_pre_info.pixels_wide() as f32,
            height: display_pre_info.pixels_high() as f32,
        })
        .subscription(subscription)
        .level(AlwaysOnTop)
        //TODO: make text_color being exchangble for the toml config
        .style(|_, _| Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .resizable(false)
        .transparent(true)
        .centered()
        .run()?;
        Ok(())
    }
}
