use ftail::Ftail;
use std::{error::Error as StdError, path::Path};

// general Ftail initializer for console log and to a file if given
pub fn init_logger(log_path: Option<&str>) -> Result<(), Box<dyn StdError>> {
    let logger = Ftail::new().formatted_console_env_level();

    if log_path.is_some() {
        logger
            .single_file_env_level(Path::new(log_path.unwrap_or_default()), false)
            .init()?;
    } else {
        logger.init()?;
    }

    Ok(())
}
