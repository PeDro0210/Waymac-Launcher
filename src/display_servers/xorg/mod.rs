use std::error::Error as StdError;

use crate::common::GlobalState;

pub struct XorgApp;

impl XorgApp {
    pub fn init(state: GlobalState) -> Result<(), Box<dyn StdError>> {
        //TODO: setup correctly for config take in mind
        todo!();
    }
}
