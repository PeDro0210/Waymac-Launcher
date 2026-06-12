use std::{env::home_dir, error::Error, ffi::OsString};

// given a extension having in mind that the "~" goes in the start, returns the complete path in
// it's absolute form for the user running the program
pub fn expand_default_arg_paths(ext: &str) -> String {
    let mut home_dir = home_dir().unwrap_or_default();
    home_dir.push(ext);
    // we won't check if the path is correct or anything, we let the other parts of the program
    // check for failures
    home_dir.to_string_lossy().to_string()
}
