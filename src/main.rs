use std::error::Error as StdError;

use waymac_launcher::WayXApp;

fn main() -> Result<(), Box<dyn StdError>> {
    WayXApp::init();
    Ok(())
}
