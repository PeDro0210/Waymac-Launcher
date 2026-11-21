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

pub fn create_default_dump_dir() {
    let exist_dump_dir = exists(DEFAULT_WAYMAC_DUMPS_PATH);

    if !exist_dump_dir.expect(
        format!(
            "Couldn't check for path '{}' which is needed for execution for 'waymac_launcher'",
            DEFAULT_WAYMAC_DUMPS_PATH
        )
        .as_str(),
    ) {
        create_dir(DEFAULT_WAYMAC_DUMPS_PATH);
    }
}
