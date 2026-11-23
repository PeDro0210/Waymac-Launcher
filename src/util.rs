use crate::{Args, data::DEFAULT_WAYMAC_DUMPS_PATH};

use std::{
    error::Error as StdError,
    fs::{create_dir, exists},
};

pub fn expand_args_paths(args: Args) -> Args {
    Args {
        config_path: shellexpand::tilde(args.config_path.as_str()).to_string(),
        debug_dump_path: shellexpand::tilde(args.debug_dump_path.as_str()).to_string(),
    }
}

pub fn create_default_dump_dir() -> Result<(), Box<dyn StdError>> {
    let expanded_waymac_dumps_path = &shellexpand::tilde(DEFAULT_WAYMAC_DUMPS_PATH).to_string();

    let exist_dump_dir = exists(expanded_waymac_dumps_path)?;

    if !exist_dump_dir {
        let _ = create_dir(expanded_waymac_dumps_path);
    }

    Ok(())
}
