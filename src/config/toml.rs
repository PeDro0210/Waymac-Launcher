use std::{fs::File, io::Read, process::exit};

use serde::Deserialize;

use log::error;

use crate::config::app::Location;

// for parsing Toml file to the WayMac config
#[derive(Deserialize)]
pub struct TomlConfig {
    pub main_window: MainWindow,
    pub inputbar: InputBar,
    pub entry: Entry,
}

impl TomlConfig {
    pub fn from_path(path: &str) -> Self {
        let toml_string_file = &mut String::new();

        let mut toml_file = if let Ok(mut file) = File::open(path) {
            let _ = file.read_to_string(toml_string_file);
            file
        } else {
            error!("Couldn't open '{path}'");
            exit(1);
        };

        let _ = toml_file.read_to_string(toml_string_file);

        match toml::from_str::<TomlConfig>(toml_string_file.as_str()) {
            Ok(config) => config,
            Err(err) => {
                error!("{err}");
                exit(1);
            }
        }
    }
}

// not a simple way to be explicit with things in here

#[derive(Deserialize)]
pub struct MainWindow {
    pub height: u32,
    pub width: u32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: String, // will be parse for RGB triplet

    pub font: String, // will fallback incase of not founding desire font

    pub location: Location, // will be parsed after in the "Location Enum"

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border: Option<Border>,
}

#[derive(Deserialize)]
pub struct InputBar {
    pub height: u32,
    pub width: u32,

    pub text_color: Option<String>, // will be parse for RGB triplet

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border: Option<Border>,
}

#[derive(Deserialize)]
pub struct Entry {
    pub height: u32,
    pub width: u32,

    pub text_color: Option<String>, // will be parse for RGB triplet
    pub focus_text_color: Option<String>, // will be parse for RGB triplet

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border: Option<Border>,
}

#[derive(Deserialize)]
pub struct Border {
    pub color: String,

    pub radius: f32,
    pub width: f32,

    pub top_left_radius: f32,
    pub bottom_left_radius: f32,
    pub top_right_radius: f32,
    pub bottom_right_radius: f32,
}
