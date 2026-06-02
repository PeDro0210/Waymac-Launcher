mod app_launcher;
mod common;
mod config;
mod data;
mod display_servers;

mod logger;

use std::{env, error::Error as StdError};

use log::{error, info};

use crate::display_servers::{
    SupportedDisplayServer, get_supported_display_server_target, quartz::QuartzApp,
    wayland::WaylandApp,
};

pub struct WayMacApp;

impl WayMacApp {
    pub fn init() -> Result<(), Box<dyn StdError>> {
        //TODO: pass the log file
        logger::init_logger(None)?;

        info!(
            "XDG_SESSION_TYPE: {:?}",
            env::var_os("XDG_SESSION_TYPE").unwrap_or_default()
        );

        match get_supported_display_server_target() {
            SupportedDisplayServer::Wayland =>
            {
                #[cfg(target_os = "linux")]
                WaylandApp::run()?
            }
            SupportedDisplayServer::Xorg => {
                error!("Xorg is not supported directly, fallbacking to Quartz app build");
            }

            SupportedDisplayServer::Quartz =>
            {
                #[cfg(target_os = "macos")]
                QuartzApp::run()?
            }
        }

        Ok(())
    }
}
