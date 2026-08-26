use std::{error::Error, fmt::Display};

pub mod app;
pub mod toml;

mod util;

//TODO: remove pub keyword when done debugging
#[derive(Debug)]
pub enum AppConfigError {
    ColorParsingError,
    TextConfigParsingError,
    TomlConfigParsingError,
}

impl Display for AppConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppConfigError::ColorParsingError => {
                write!(f, "Error was found while parsing color")
            }
            AppConfigError::TextConfigParsingError => {
                write!(f, "Error while parsing text from toml")
            }
            AppConfigError::TomlConfigParsingError => {
                write!(f, "Error while parsing toml")
            }
        }
    }
}

impl Error for AppConfigError {}
