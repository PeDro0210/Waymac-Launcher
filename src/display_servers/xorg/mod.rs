use std::error::Error as StdError;

use iced::{Size, application};

use crate::common::{LauncherState, update, view};

pub struct XorgApp;

impl XorgApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        application(LauncherState::default, update, view)
            .decorations(false)
            .window_size(Size {
                width: 350.,
                height: 350.,
            })
            .resizable(false)
            .run()?;
        Ok(())
    }

    fn namespace() -> String {
        String::from("XWay - Xorg App")
    }
}
