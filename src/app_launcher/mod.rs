mod utils;

use std::{
    fs::{File, ReadDir, read_dir},
    io::Error as StdError,
    path::{Path, PathBuf},
    vec::Vec,
};

use iced::Error;
use log::{debug, info, warn};

#[cfg(target_os = "linux")]
use xdg::BaseDirectories;

use utils::*;

#[derive(Debug)]
pub struct DesktopEntry {
    pub name: String,
    pub desktop_entry_path: Box<PathBuf>,
    pub icon: Option<String>,
}

//TODO: make matching for XDG and Macos
pub fn get_desktop_entry() -> Vec<DesktopEntry> {
    let mut desktop_entries: Vec<DesktopEntry> = Vec::new();

    return match get_desktop_entry_target() {
        DesktopEntriesTarget::XDG => {
            #[cfg(target_os = "linux")]
            {
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
                            Err(_) => false, //fallback in weird case of not being able to open
                        });

                        for application_dir_result in application_dir_results {
                            // append each different application entry in the big desktop_entries
                            desktop_entries
                                .append(&mut get_xdg_dir_entries(application_dir_result));
                        }
                    }

                    // we open each of the desktop entrys and parse them to the struct
                }

                desktop_entries
            }

            warn!(
                "Desktop entry target was selected as XDG, when target linux is not being supported."
            );
            desktop_entries
        }
        // TODO: add different paths for different dir with applications
        #[cfg(target_os = "macos")]
        DesktopEntriesTarget::MacOS => {
            let gen_apps = get_application_desktop_entry(Path::new("/Applications"));
            info!("gen_apps entries {:?}", gen_apps);
            gen_apps
        }
    };
}
