mod utils;

use std::{
    path::{Path, PathBuf},
    vec::Vec,
};

use iced::Error;

pub struct DesktopEntry {
    pub name: String,
    pub desktop_entry: Box<PathBuf>,
    pub application_image: Box<Path>,
}

//TODO: make matching for XDG and Macos
pub fn get_desktop_entry() -> Result<Vec<DesktopEntry>, Error> {
    todo!()
}
