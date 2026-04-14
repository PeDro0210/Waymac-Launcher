mod common;
mod display_servers;

pub mod logger;

use std::error::Error as StdError;

pub struct WayXDaemon;

impl WayXDaemon {
    pub fn init() -> Result<(), Box<dyn StdError>> {
        //TODO: setup iced daemon
        //iced::daemon(move || {});
        todo!();
    }
}
