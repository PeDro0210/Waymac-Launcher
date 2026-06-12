use std::{env::home_dir, error::Error, ffi::OsString};

use crate::Args;

pub fn expand_args_paths(args: Args) -> Args {
    Args {
        config_path: shellexpand::tilde(args.config_path.as_str()).to_string(),
        debug_dump_path: shellexpand::tilde(args.debug_dump_path.as_str()).to_string(),
    }
}
