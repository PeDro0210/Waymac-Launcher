use std::error::Error as StdError;

pub struct WaylandApp;

// Implementation for the just initialzation for the daemon
impl WaylandApp {
    pub fn init() -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        todo!()
    }
}
