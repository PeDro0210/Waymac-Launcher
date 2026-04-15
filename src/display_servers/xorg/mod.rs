use std::error::Error as StdError;

use iced::application;

use crate::common::{LauncherState, update, view};

pub struct XorgApp;

impl XorgApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        application(LauncherState::default, update, view).run()?;
        Ok(())
    }

    fn namespace() -> String {
        String::from("XWay - Xorg App")
    }
}
