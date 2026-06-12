use std::error::Error as StdError;

use iced::{Color, Size, application, theme::Style, window::Level::AlwaysOnTop};

#[cfg(target_os = "macos")]
use crate::Args;
use crate::common::{boot, subscription, update, view};

use iced::{Renderer, Theme};

#[cfg(target_os = "macos")]
use core_graphics::display::CGDisplay;

pub struct QuartzApp;

#[cfg(target_os = "macos")]
impl QuartzApp {
    //TODO: pass args
    pub fn run(arg: Args) -> Result<(), Box<dyn StdError>> {
        let display_pre_info = CGDisplay::main();
        //TODO: setup correctly for config take in mind
        application(boot, update, view::<Theme, Renderer>)
            .decorations(false)
            .window_size(Size {
                width: display_pre_info.pixels_wide() as f32,
                height: display_pre_info.pixels_high() as f32,
            })
            .subscription(subscription)
            .level(AlwaysOnTop)
            //TODO: make text_color being exchangble for the toml config
            .style(|_, _| Style {
                background_color: Color::TRANSPARENT,
                text_color: Color::WHITE,
            })
            .resizable(false)
            .transparent(true)
            .centered()
            .run()?;
        Ok(())
    }
}
