use std::default;

use iced::{Background, Color, Font, Size, advanced::graphics::Image, font};
use serde::Deserialize;

use log::{debug, error, warn};

use crate::config::toml::{Entry, InputBar, MainWindow, TomlConfig};
use crate::config::util::ColorHEX;

type TextConfig = (Font, Color);

//TODO: remove pub keyword when done debugging
#[derive(Debug)]
pub enum AppConfigError {
    ColorParsingError,
    TextConfigParsingError,
}

// config struct for using directly in WayMacApp
#[derive(Default, Clone, Copy)]
pub struct WayMacConfig {
    pub main_font: Font,
    pub text_color: Color,
    pub main_window: ContainerConfig,
    pub input_bar: ContainerConfig,
    pub entry: ContainerConfig,
}

impl WayMacConfig {
    fn manage_color_parsing(raw_color: &str) -> Result<Color, AppConfigError> {
        match Color::from_raw_hex(raw_color) {
            Ok(color) => Ok(color),
            Err(err) => {
                error!("{err:?} while trying to parse color: {}", raw_color);
                return Err(AppConfigError::TextConfigParsingError);
            }
        }
    }

    fn parse_text_config(toml: &TomlConfig) -> Result<TextConfig, AppConfigError> {
        let raw_main_font = &toml.main_window.font;
        let raw_main_text_color = &toml.main_window.text_color;

        //TODO: manage font error
        let main_font = Font::with_name(Box::leak(raw_main_font.clone().into_boxed_str())); //leak for
        //making the String a static reference

        let text_color = WayMacConfig::manage_color_parsing(&raw_main_text_color)?;
        Ok((main_font, text_color))
    }

    fn parse_main_window(
        toml: &MainWindow,
        main_font: &Font,
        text_color: &Color,
    ) -> Result<ContainerConfig, AppConfigError> {
        //TODO: apply background for images to
        let background_color =
            WayMacConfig::manage_color_parsing(toml.background_color.clone().unwrap().as_str())?;
        let background = Background::Color(background_color);

        let border_color = if let Some(border_color) = toml.border_color.to_owned() {
            Some(WayMacConfig::manage_color_parsing(border_color.as_str())?)
        } else {
            None
        };

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.height as f32,
            },
            text_color: *text_color,
            //TODO: do the fallback to the main_font
            font: *main_font, //for the moment wi'll leave the main_font
            background,
            border_color,
            border_radius: toml.border_radius,
            specific: ContainerType::MainWindow {
                location: toml.location,
                padding: toml.padding,
                spacing: toml.spacing,
            },
        })
    }
    fn parse_input_bar(
        toml: &InputBar,
        main_font: &Font,
        text_color: &Color,
    ) -> Result<ContainerConfig, AppConfigError> {
        //TODO: apply background for images to
        let background_color =
            WayMacConfig::manage_color_parsing(toml.background_color.clone().unwrap().as_str())?;
        let background = Background::Color(background_color);

        let border_color = if let Some(border_color) = toml.border_color.to_owned() {
            Some(WayMacConfig::manage_color_parsing(border_color.as_str())?)
        } else {
            None
        };

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.height as f32,
            },
            text_color: *text_color,
            //TODO: do the fallback to the main_font
            font: *main_font, //for the moment wi'll leave the main_font
            background,
            border_color,
            border_radius: toml.border_radius,
            specific: ContainerType::InputBar,
        })
    }
    fn parse_entry(
        toml: &Entry,
        main_font: &Font,
        text_color: &Color,
    ) -> Result<ContainerConfig, AppConfigError> {
        //TODO: apply background for images to
        let background_color =
            WayMacConfig::manage_color_parsing(toml.background_color.clone().unwrap().as_str())?;
        let background = Background::Color(background_color);

        let border_color = if let Some(border_color) = toml.border_color.to_owned() {
            Some(WayMacConfig::manage_color_parsing(border_color.as_str())?)
        } else {
            None
        };

        //TODO: manage option for focus_text_color
        let focus_text_color =
            WayMacConfig::manage_color_parsing(&toml.focus_text_color.clone().unwrap().as_str())?;

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.height as f32,
            },
            text_color: *text_color,
            //TODO: do the fallback to the main_font
            font: *main_font, //for the moment wi'll leave the main_font
            background,
            border_color,
            border_radius: toml.border_radius,
            specific: ContainerType::Entry { focus_text_color },
        })
    }

    pub fn parse_from_toml(toml: TomlConfig) -> Result<Self, AppConfigError> {
        let (main_font, text_color) = WayMacConfig::parse_text_config(&toml)?;
        debug!("MAIN FONT: {:?}, TEXT COLOR: {:?}", main_font, text_color);

        return Ok(WayMacConfig {
            main_font,
            text_color,
            main_window: WayMacConfig::parse_main_window(
                &toml.main_window,
                &main_font,
                &text_color,
            )?,
            input_bar: WayMacConfig::parse_input_bar(&toml.inputbar, &main_font, &text_color)?,
            entry: WayMacConfig::parse_entry(&toml.entry, &main_font, &text_color)?,
        });
    }
}

// each different type of container that WayMac has
#[derive(Clone, Copy)]
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

#[derive(Deserialize, Clone, Copy)]
pub enum Location {
    Center,
    Top,
    Bottom,
    Right,
    Left,
}

// describes the behavoir of WayMac components
#[derive(Clone, Copy)]
pub struct ContainerConfig {
    pub size: Size,

    pub text_color: Color,
    pub font: Font,

    //TODO: implement backgrounds
    pub background: Background,

    //TODO: implement border fields
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,

    // other configs depending which container is it
    pub specific: ContainerType,
}

// just for being compliant for the LauncherState
impl Default for ContainerConfig {
    fn default() -> Self {
        ContainerConfig {
            size: Size::default(),
            text_color: Color::BLACK,
            font: Font::default(),
            background: Background::Color(Color::default()),
            border_color: None,
            border_radius: None,
            specific: ContainerType::InputBar,
        }
    }
}
