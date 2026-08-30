use std::{fs::File, io::Read, panic, process::exit};

use log::{error, trace};
use regex::regex;
pub type WaymacErrorVector = Vec<String>;

pub fn find_errors_in_stdout(log_path: &String) -> WaymacErrorVector {
    // this can't fail by any
    // means, if it does will make it panic
    let mut file = match File::open(log_path) {
        Ok(file) => file,
        Err(err) => {
            error!("{err}, critic error detected");
            exit(1);
        }
    };

    // not letting anything interfer with the writing for the log
    let _ = file.lock().unwrap();

    let content = &mut String::new();
    let _ = file.read_to_string(content);

    let re = regex!(
        r"(([0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]\s[0-9][0-9]:[0-9][0-9]:[0-9][0-9]\sERROR\s([\w]|::+)+\s)((.|\s)+))(--WAYMAC_ERROR--)"
    );

    let mut errors = vec![];
    for (_, [_, _, _, error_detected, _, _]) in re.captures_iter(content).map(|c| c.extract()) {
        trace!("error lines: {error_detected}");
        errors.push(error_detected.to_string());
    }

    return errors;
}
