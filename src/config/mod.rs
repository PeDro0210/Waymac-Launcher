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
