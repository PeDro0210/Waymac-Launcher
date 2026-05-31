use std::error::Error as StdError;

use waymac_launcher::WayMacApp;

fn main() -> Result<(), Box<dyn StdError>> {
    WayMacApp::init()?;
    Ok(())
}
