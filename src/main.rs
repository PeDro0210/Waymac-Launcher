use log::{info, warn};
use std::error::Error as StdError;
use waymac_launcher::logger;

fn main() -> Result<(), Box<dyn StdError>> {
    logger::init_logger(Some("txt.txt"))?;
    warn!("hi");
    Ok(())
}
