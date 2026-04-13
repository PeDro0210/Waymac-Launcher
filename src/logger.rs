use ftail::{Ftail, error::FtailError};
use log::{LevelFilter, warn};
use std::{error::Error as StdError, path::Path};

pub fn init_logger(log_path: Option<&str>) -> Result<(), Box<dyn StdError>> {
    Ftail::new()
        .formatted_console_env_level()
        .single_file_env_level(Path::new(log_path.unwrap_or_default()), true);
    Ok(())
}
