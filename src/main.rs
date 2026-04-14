use std::error::Error as StdError;
use waymac_launcher::logger;

fn main() -> Result<(), Box<dyn StdError>> {
    // TODO: pass the path as an argument for the log_path (in case of one given)
    logger::init_logger(None)?;
    Ok(())
}
