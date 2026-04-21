mod utils;

use std::{
    fs::{ReadDir, read_dir},
    io::Error as StdError,
    path::{Path, PathBuf},
    vec::Vec,
};

use iced::Error;
use log::{debug, info};
use xdg::BaseDirectories;

use utils::*;

pub struct DesktopEntry {
    pub name: String,
    pub desktop_entry_path: Box<PathBuf>,
    pub application_image: Box<Path>,
}

//TODO: make matching for XDG and Macos
pub fn get_desktop_entry() -> Result<Vec<DesktopEntry>, Error> {
    return match get_desktop_entry_target() {
        DesktopEntriesTarget::XDG => {
            let mut xdg_data_dir = BaseDirectories::new().data_dirs;

            // for checking the home directory for user only applications
            let xdg_data_home_dir = BaseDirectories::new().data_home.unwrap_or_default();
            xdg_data_dir.push(xdg_data_home_dir);

            info!("xdg data dirs: {:?}", xdg_data_dir);

            // LMAO, THIS IS TO NESTED
            // we iterate over all the applications for have a pretty robust entry
            for xdg_path in xdg_data_dir {
                // per path, search for an inner application directory, if it doesn't exist, we just
                // skipped
                if let Ok(path_inners) = xdg_path.read_dir() {
                    // well search all the applications instances
                    let application_dir_results = path_inners.filter(|path| match path {
                        Ok(dir_entry) => dir_entry
                            .path()
                            .into_os_string()
                            .into_string()
                            .unwrap_or_default()
                            .contains("applications"),
                        Err(_) => false,
                    });

                    for application_dir_result in application_dir_results {
                        if let Ok(application_dir) = application_dir_result {
                            let dir_contents = read_dir(application_dir.path()).into_iter();
                            for dir in dir_contents {
                                info!("dir path: {:?}", dir);
                            }
                        }
                    }

                    // we'll just have one entry
                }

                // we open each of the desktop entrys and parse them to the struct
            }

            Ok(Vec::new())
        }
        DesktopEntriesTarget::MacOS => {
            todo!()
        }
    };
}
