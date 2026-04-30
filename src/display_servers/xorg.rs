use std::error::Error as StdError;

use iced::theme::Style;
use iced::{Size, application, window::Level::AlwaysOnTop};

use crate::common::{LauncherState, boot, subscription, update, view};

use iced::{Color, Renderer, Theme};

use iced::window::Position::Centered;

pub struct XorgApp;

//TODO: implement this for using the rendererer directly instead of passing through the application
//abstraction
impl XorgApp {
    pub fn run() -> Result<(), Box<dyn StdError>> {
        // most of burned settings are for simulating a layer with a window
        //TODO: setup correctly for config take in mind
        application(boot, update, view::<Theme, Renderer>)
            // TODO: change the text_color depending the config
            .style(|_state, _theme| Style {
                background_color: Color::TRANSPARENT,
                text_color: Color::WHITE,
            })
            .decorations(false)
            .transparent(true)
            .window_size(Size {
                width: 520.,
                height: 520.,
            })
            .position(Centered)
            .subscription(subscription)
            .level(AlwaysOnTop)
            .resizable(false)
            .run()?;
        Ok(())
    }
}
