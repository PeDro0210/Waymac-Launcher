use ftail::Ftail;
use log::info;
use std::{
    error::Error as StdError,
    fs::{File, canonicalize, exists},
    path::{Path, PathBuf},
};

// general Ftail initializer for console log and to a file if given
pub fn init_logger(log_path: Option<&str>) -> Result<(), Box<dyn StdError>> {
    let logger = Ftail::new().formatted_console_env_level();

    if let Some(path) = log_path {
        match exists(path) {
            Ok(_) => {
                logger
                    .single_file_env_level(Path::new(path), false)
                    .init()?;
            }
            Err(_) => {
                File::create_new(Path::new(path))?;
                logger
                    .single_file_env_level(Path::new(path), false)
                    .init()?;
            }
        }
    } else {
        logger.init()?;
    }

    Ok(())
}
