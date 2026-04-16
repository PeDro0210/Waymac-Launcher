use std::error::Error as StdError;

use iced::{Size, application};

use crate::common::{LauncherState, boot, subscription, update, view};

pub struct XorgApp;

impl XorgApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        application(boot, update, view)
            .decorations(false)
            .window_size(Size {
                width: 350.,
                height: 350.,
            })
            .subscription(subscription)
            .resizable(false)
            .run()?;
        Ok(())
    }

    fn namespace() -> String {
        String::from("XWay - Xorg App")
    }
}
