mod common;
mod data;
mod display_servers;

mod logger;

use std::{any::Any, env, error::Error as StdError};

use iced::{
    advanced::widget::operation::{
        focusable::{focus, is_focused},
        text_input,
    },
    widget::Id,
};
use log::info;

use crate::{
    data::LAUNCHER_TEXT_INPUT_ID,
    display_servers::{wayland::WaylandApp, xorg::XorgApp},
};

pub struct WayXApp;

impl WayXApp {
    pub fn init() -> Result<(), Box<dyn StdError>> {
        //TODO: pass the log file
        logger::init_logger(None)?;

        info!(
            "XDG_SESSION_TYPE: {:?}",
            env::var_os("XDG_SESSION_TYPE").unwrap_or_default()
        );

        //focusing the launcher text_input just at start

        if env::var_os("XDG_SESSION_TYPE").unwrap_or_default() == "wayland" {
            #[cfg(target_os = "linux")]
            WaylandApp::run()?;
        } else {
            XorgApp::run()?;
        }

        Ok(())
    }
}
