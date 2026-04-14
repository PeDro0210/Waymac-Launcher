use std::error::Error as StdError;

use crate::common::GlobalState;

pub struct WaylandApp;

// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn init(state: GlobalState) -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        todo!()
    }
}
