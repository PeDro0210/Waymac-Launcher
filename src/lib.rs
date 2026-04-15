mod common;
mod display_servers;

mod logger;

use std::{env, error::Error as StdError};

use log::info;

use crate::{
    display_servers::{wayland::WaylandApp, xorg::XorgApp},
    logger::init_logger,
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

        if env::var_os("XDG_SESSION_TYPE").unwrap_or_default() == "wayland" {
            #[cfg(target_os = "linux")]
            WaylandApp::run()?;
        } else {
            XorgApp::run()?;
        }

        Ok(())
    }
}
