use std::default;

use iced::Border;
use iced::border::Radius;
use iced::{Background, Color, Font, Size, advanced::graphics::Image, font};
use serde::Deserialize;

use log::{debug, error, warn};

use crate::config::toml::{Border as RawBorder, Entry, InputBar, MainWindow, TomlConfig};
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

    fn parse_background(
        raw_bg_color: &Option<String>,
    ) -> Result<Option<Background>, AppConfigError> {
        if let Some(background_color) = raw_bg_color {
            let background_color = WayMacConfig::manage_color_parsing(&background_color)?;
            Ok(Some(Background::Color(background_color)))
        } else {
            Ok(None)
        }
    }

    fn parse_border(border: &Option<RawBorder>) -> Result<Option<BorderInfo>, AppConfigError> {
        if let Some(border) = border.as_ref() {
            Ok(Some(BorderInfo {
                color: WayMacConfig::manage_color_parsing(border.color.as_str())?,

                top_left_radius: border.top_left_radius,
                top_right_radius: border.top_right_radius,
                bottom_left_radius: border.bottom_left_radius,
                bottom_right_radius: border.bottom_left_radius,

                bottom_width: border.bottom_width,
                top_width: border.top_width,
                right_width: border.right_width,
                left_width: border.left_width,
            }))
        } else {
            Ok(None)
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
        let background = WayMacConfig::parse_background(&toml.background_color)?;

        let border = WayMacConfig::parse_border(&toml.border)?;

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.height as f32,
            },
            text_color: *text_color,
            //TODO: do the fallback to the main_font
            font: *main_font, //for the moment will leave the main_font
            background,
            border,
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
        let background = WayMacConfig::parse_background(&toml.background_color)?;

        let border = WayMacConfig::parse_border(&toml.border)?;

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.height as f32,
            },
            text_color: *text_color,
            //TODO: do the fallback to the main_font
            font: *main_font, //for the moment will leave the main_font
            background,
            border,
            specific: ContainerType::InputBar,
        })
    }
    fn parse_entry(
        toml: &Entry,
        main_font: &Font,
        text_color: &Color,
    ) -> Result<ContainerConfig, AppConfigError> {
        //TODO: apply background for images to
        let background = WayMacConfig::parse_background(&toml.background_color)?;

        //TODO: manage option for focus_text_color
        let focus_text_color = if let Some(background_color) = toml.focus_text_color.to_owned() {
            WayMacConfig::manage_color_parsing(&background_color)?
        } else {
            *text_color
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
            border: None,
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

#[derive(Clone, Copy)]
pub struct BorderInfo {
    pub color: Color,

    pub top_left_radius: f32,
    pub bottom_left_radius: f32,
    pub top_right_radius: f32,
    pub bottom_right_radius: f32,

    pub left_width: f32,
    pub bottom_width: f32,
    pub top_width: f32,
    pub right_width: f32,
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
    pub background: Option<Background>,

    //TODO: implement border fields
    pub border: Option<BorderInfo>,

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
            background: Some(Background::Color(Color::default())),
            border: None,
            specific: ContainerType::InputBar,
        }
    }
}
