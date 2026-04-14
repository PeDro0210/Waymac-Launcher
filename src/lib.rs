mod common;
mod display_servers;

mod logger;

use std::{env, error::Error as StdError};

pub struct WayXDaemon;

impl WayXDaemon {
    pub fn init() -> Result<(), Box<dyn StdError>> {
        if env::var_os("XDG_SESSION_TYPE").is_some() {
        } else {
        }

        Ok(())
    }
}
