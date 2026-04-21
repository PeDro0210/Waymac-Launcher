mod app_launcher;
mod common;
mod data;
mod display_servers;

mod logger;

use std::{any::Any, env, error::Error as StdError};

use iced::widget::Id;
use log::info;

use crate::{
    data::LAUNCHER_TEXT_INPUT_ID,
    display_servers::{
        SupportedDisplayServer, get_supported_display_server_target, wayland::WaylandApp,
        xorg::XorgApp,
    },
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

        match get_supported_display_server_target() {
            SupportedDisplayServer::Wayland => WaylandApp::run()?,
            SupportedDisplayServer::Xorg => XorgApp::run()?,
        }

        Ok(())
    }
}
