use std::default;

use iced::border::Radius;
use iced::gradient::{ColorStop, Linear};
use iced::{Background, Color, Font, Size, advanced::graphics::Image, font};
use iced::{Border, Radians};
use serde::Deserialize;

use log::{debug, error, warn};

use crate::config::AppConfigError;
use crate::config::toml::{Border as RawBorder, Entry, InputBar, MainWindow, TomlConfig};
use crate::config::util::ColorHEX;
use crate::data::STDOUT_POSTFIX_WAYMAC;

type TextConfig = (Font, Color);

// config struct for using directly in WayMacApp
#[derive(Clone, Copy)]
pub struct WayMacConfig {
    pub main_font: Font,
    pub text_color: Color,
    pub main_window: ContainerConfig,
    pub input_bar: ContainerConfig,
    pub entry: ContainerConfig,
}

// default WayMacConfig will be the fall back in case of failing the config
impl Default for WayMacConfig {
    // will be burned for ease of use
    fn default() -> Self {
        WayMacConfig {
            main_font: Font::default(),
            text_color: Color::default(),
            main_window: ContainerConfig {
                size: Size {
                    width: 350.,
                    height: 350.,
                },
                specific: ContainerType::MainWindow {
                    location: Location::Center,
                    padding: 0.,
                    spacing: 0.,
                },
                background: Some(Background::Color(Color::BLACK)),
                ..Default::default()
            },
            input_bar: ContainerConfig {
                size: Size {
                    height: 1.,
                    width: 350.,
                },
                specific: ContainerType::InputBar,
                ..Default::default()
            },
            entry: ContainerConfig {
                size: Size {
                    width: 350.,
                    height: 25.,
                },
                specific: ContainerType::Entry {
                    focus_text_color: Color::default(),
                },
                ..Default::default()
            },
        }
    }
}

impl WayMacConfig {
    fn manage_color_parsing(raw_color: &str) -> Result<Color, AppConfigError> {
        match Color::from_raw_hex(raw_color) {
            Ok(color) => Ok(color),
            Err(err) => {
                error!(
                    "{err:?} while trying to parse color: {} {}",
                    raw_color, STDOUT_POSTFIX_WAYMAC
                );
                return Err(AppConfigError::TextConfigParsingError);
            }
        }
    }

    fn parse_background(
        raw_bg_colors: &Option<Vec<String>>,
        gradient_angle: &Option<f32>,
    ) -> Result<Option<Background>, AppConfigError> {
        if let Some(background_colors) = raw_bg_colors {
            if background_colors.len() == 1 {
                let background_color =
                    WayMacConfig::manage_color_parsing(&background_colors.first().unwrap())?;
                return Ok(Some(Background::Color(background_color)));
            }

            // has to be lower than 8
            let mut gradient_stops: [Option<ColorStop>; 8] = [None; 8];

            for bg_raw_idx in 0..background_colors.len() {
                let color = WayMacConfig::manage_color_parsing(
                    &background_colors.get(bg_raw_idx).unwrap(),
                )?;

                gradient_stops[bg_raw_idx] = Some(ColorStop {
                    color,
                    offset: bg_raw_idx as f32,
                });
            }

            return Ok(Some(Background::Gradient(iced::Gradient::Linear(Linear {
                angle: Radians(match gradient_angle {
                    Some(gradient_angle) => *gradient_angle,
                    None => 0.,
                }),
                stops: gradient_stops,
            }))));
        }
        Ok(None)
    }

    fn parse_border(border: &Option<RawBorder>) -> Result<Option<Border>, AppConfigError> {
        if let Some(border) = border.as_ref() {
            return Ok(Some(Border {
                color: WayMacConfig::manage_color_parsing(border.color.as_str())?,
                width: border.width,
                radius: Radius {
                    top_left: border.top_left_radius,
                    top_right: border.top_right_radius,
                    bottom_right: border.bottom_right_radius,
                    bottom_left: border.bottom_left_radius,
                },
            }));
        }
        Ok(None)
    }

    fn parse_text_config(toml: &TomlConfig) -> Result<TextConfig, AppConfigError> {
        let main_window = &toml.main_window;

        let raw_main_font = &main_window.font;
        let raw_main_text_color = &main_window.text_color;

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

        let background =
            WayMacConfig::parse_background(&toml.background_colors, &toml.gradient_angle)?;

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
        let background =
            WayMacConfig::parse_background(&toml.background_colors, &toml.gradient_angle)?;

        let border = WayMacConfig::parse_border(&toml.border)?;

        Ok(ContainerConfig {
            size: Size {
                width: toml.width as f32,
                height: toml.line_height as f32,
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
        let background =
            WayMacConfig::parse_background(&toml.background_colors, &toml.gradient_angle)?;

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

    pub fn parse_from_toml(
        toml: &Result<&TomlConfig, &AppConfigError>,
    ) -> Result<Self, AppConfigError> {
        if let Err(_) = &toml {
            return Err(AppConfigError::TextConfigParsingError);
        }
        let toml_unwrapped = toml.unwrap();

        let (main_font, text_color) = WayMacConfig::parse_text_config(&toml_unwrapped)?;
        debug!("MAIN FONT: {:?}, TEXT COLOR: {:?}", main_font, text_color);

        return Ok(WayMacConfig {
            main_font,
            text_color,
            main_window: WayMacConfig::parse_main_window(
                &toml_unwrapped.main_window,
                &main_font,
                &text_color,
            )?,
            input_bar: WayMacConfig::parse_input_bar(
                &toml_unwrapped.inputbar,
                &main_font,
                &text_color,
            )?,
            entry: WayMacConfig::parse_entry(&toml_unwrapped.entry, &main_font, &text_color)?,
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

#[derive(Deserialize, Clone, Copy, Default)]
pub enum Location {
    #[default]
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
    pub border: Option<Border>,

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
