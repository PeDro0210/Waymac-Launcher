use std::{fs::File, io::Read, process::exit};

use serde::Deserialize;

use log::error;

use crate::config::app::Location;

// for parsing Toml file to the WayMac config
#[derive(Deserialize)]
pub struct TomlConfig {
    main_window: MainWindow,
    inputbar: InputBar,
    entry: Entry,
}

impl TomlConfig {
    pub fn from_path(path: &str) -> Self {
        let toml_string_file = &mut String::new();
        let mut toml_file = File::open(path).unwrap();
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
struct MainWindow {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: String, // will be parse for RGB triplet

    pub font: String, // will fallback incase of not founding desire font

    pub location: Location, // will be parsed after in the "Location Enum"

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_color: Option<String>,
    pub border_radius: Option<f32>,
}

#[derive(Deserialize)]
struct InputBar {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: Option<String>, // will be parse for RGB triplet

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_color: Option<String>,
    pub border_radius: Option<f32>,
}

#[derive(Deserialize)]
struct Entry {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: Option<String>, // will be parse for RGB triplet
    pub focus_text_color: Option<String>, // will be parse for RGB triplet

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_color: Option<String>,
    pub border_radius: Option<f32>,
}
