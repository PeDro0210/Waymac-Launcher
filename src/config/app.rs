use iced::{Background, Color, Font, Size, advanced::graphics::Image, font};
use serde::Deserialize;

use log::{error, warn};

use crate::config::toml::TomlConfig;
use crate::config::util::ColorHEX;

type TextConfig = (Font, Color);

enum AppConfigError {
    TextConfigParsingError,
}

// config struct for using directly in WayMacApp
pub struct WayMacConfig {
    pub main_font: Font,
    pub text_color: Color,
    pub main_window: ContainerConfig,
    pub input_bar: ContainerConfig,
    pub entry: ContainerConfig,
}

impl WayMacConfig {
    fn parse_text_config(toml: TomlConfig) -> Result<TextConfig, AppConfigError> {
        let raw_main_font = toml.main_window.font;
        let raw_main_text_color = toml.main_window.text_color;

        //TODO: manage font error
        let main_font = Font::with_name(Box::leak(raw_main_font.into_boxed_str())); //leak for
        //making the String a static reference

        let text_color = match Color::from_raw_hex(raw_main_text_color.as_str()) {
            Ok(color) => color,
            Err(err) => {
                error!(
                    "{err:?} while trying to parse color: {}",
                    raw_main_text_color
                );
                return Err(AppConfigError::TextConfigParsingError);
            }
        };

        Ok((main_font, text_color))
    }
    pub fn parse_from_toml(toml: TomlConfig) -> Self {
        todo!()
    }
}

// each different type of container that WayMac has
pub enum ContainerType {
    MainWindow {
        // location depending the layer
        location: Location,
        padding: f32,
        spacing: f32,
    },
    InputBar,
    Entry {
        focus_text_color: Color,
    },
}

#[derive(Deserialize)]
pub enum Location {
    Center,
    Top,
    Bottom,
    Right,
    Left,
}

// describes the behavoir of WayMac components
pub struct ContainerConfig {
    pub size: Size,

    pub text_color: Color,
    pub font: Font,

    pub background: Background,

    pub border_color: Option<Color>,
    pub border_radius: f32,

    // other configs depending which container is it
    pub specific: ContainerType,
}
