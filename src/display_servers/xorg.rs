use std::error::Error as StdError;

use iced::{Size, application, window::Level::AlwaysOnTop};

use crate::common::{LauncherState, boot, subscription, update, view};

use iced::{Renderer, Theme};

pub struct XorgApp;

impl XorgApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        application(boot, update, view::<Theme, Renderer>)
            .decorations(false)
            .window_size(Size {
                width: 350.,
                height: 350.,
            })
            .subscription(subscription)
            .level(AlwaysOnTop)
            .resizable(false)
            .run()?;
        Ok(())
    }
}
