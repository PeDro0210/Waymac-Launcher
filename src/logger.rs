use ftail::{error::FtailError, Ftail};
use log::{warn, LevelFilter};
use std::{error::Error as StdError, path::Path};

// general Ftail initializer for console log and to a file if given
pub fn init_logger(log_path: Option<&str>) -> Result<(), Box<dyn StdError>> {
    Ftail::new()
        .formatted_console_env_level()
        .single_file_env_level(Path::new(log_path.unwrap_or_default()), true);
    Ok(())
}
