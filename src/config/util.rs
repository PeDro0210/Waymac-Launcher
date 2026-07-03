extern crate iced;

use fhex::FromHex;
use iced::Color;
use log::{error, trace};

#[derive(Debug)]
pub enum ColorHexError {
    NotPrefix,
    NotCorrectLenght,
    ParseTriplet,
}

pub trait ColorHEX {
    // given an str with the Hex triplet RGB format, return a color
    fn from_raw_hex(raw_hex: &str) -> Result<Color, ColorHexError>;
}

impl ColorHEX for Color {
    fn from_raw_hex(raw_hex: &str) -> Result<Color, ColorHexError> {
        if !raw_hex.starts_with("#") {
            error!("Raw color {raw_hex} doesn't have '#' prefix");
            return Err(ColorHexError::NotPrefix);
        }

        let raw_hex_no_prefix = raw_hex.strip_prefix("#").ok_or(ColorHexError::NotPrefix)?;

        if raw_hex_no_prefix.len() != 6 {
            error!("Raw color {raw_hex} is not correct size");
            return Err(ColorHexError::NotCorrectLenght);
        }

        let r = f32::from_hex(format!("0x{}", &raw_hex_no_prefix[0..2]).as_str())
            .ok_or(ColorHexError::ParseTriplet)?;

        let g = f32::from_hex(format!("0x{}", &raw_hex_no_prefix[3..4]).as_str())
            .ok_or(ColorHexError::ParseTriplet)?;

        let b = f32::from_hex(format!("0x{}", &raw_hex_no_prefix[5..6]).as_str())
            .ok_or(ColorHexError::ParseTriplet)?;

        trace!("raw_hex: {raw_hex}, r: {r}, g: {g}, b: {b}");
        Ok(Color::from_rgb(r / 255., g / 255., b / 255.))
    }
}
