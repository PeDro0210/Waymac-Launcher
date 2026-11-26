mod utils;

use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::io::{Read, Write};
use std::process::Command;
use std::{
    error::Error as StdError,
    fs::{File, ReadDir, read_dir},
    path::{Path, PathBuf},
    process::exit,
    vec::Vec,
};

#[cfg(target_os = "linux")]
use xdg::BaseDirectories;

use utils::*;

use crate::data::{DEFAULT_WAYMAC_DUMPS_PATH, DESKTOP_ENTRY_CACHE};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DesktopEntry {
    pub name: String,
    pub desktop_entry_path: Box<PathBuf>,

    // for ease of managing the DesktopEntry state in the view
    pub is_focus: bool,

    // rank for how above the entry shows
    pub rank: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct CachedEntriesTOML {
    entries_size: usize,
    entries: Vec<DesktopEntry>,
}

fn write_entries_file(entries: Vec<DesktopEntry>) -> Result<(), Box<dyn StdError>> {
    let desktop_entry_cache_path =
        shellexpand::tilde(format!("{DEFAULT_WAYMAC_DUMPS_PATH}{DESKTOP_ENTRY_CACHE}").as_str())
            .to_string();

    let mut entries_file_writer = File::create(desktop_entry_cache_path.clone())?;

    let entries_toml = toml::to_string(&CachedEntriesTOML {
        entries_size: entries.len(),
        entries,
    })?;

    entries_file_writer.write(entries_toml.as_bytes())?;

    Ok(())
}

pub async fn load_entries_file() -> Result<Vec<DesktopEntry>, Box<dyn StdError>> {
    let desktop_entry_cache_path =
        shellexpand::tilde(format!("{DEFAULT_WAYMAC_DUMPS_PATH}{DESKTOP_ENTRY_CACHE}").as_str())
            .to_string();

    let string_entries = &mut String::new();

    let mut entries_file = File::open(desktop_entry_cache_path)?;

    entries_file.read_to_string(string_entries)?;

    let entries = toml::from_str::<CachedEntriesTOML>(string_entries)?;

    debug!("entries loaded: {entries:?}");

    Ok(entries.entries)
}

//TODO: make matching for XDG and Macos
pub async fn get_desktop_entry() -> Box<Vec<DesktopEntry>> {
    let mut desktop_entries: Vec<DesktopEntry> = Vec::new();

    match get_desktop_entry_target() {
        DesktopEntriesTarget::XDG => {
            #[cfg(target_os = "linux")]
            {
                let mut xdg_data_dir = BaseDirectories::new().data_dirs;

                // for checking the home directory for user only applications
                let xdg_data_home_dir = BaseDirectories::new().data_home.unwrap_or_default();
                xdg_data_dir.push(xdg_data_home_dir);

                debug!("xdg data dirs: {:?}", xdg_data_dir);

                // we do the iteration for checking if the new entries aren't needed and just
                // falling back to the cache

                // LMAO, THIS IS TO NESTED
                // we iterate over all the applications for have a pretty robust entry
                for xdg_path in xdg_data_dir {
                    // per path, search for an inner application directory, if it doesn't exist, we just
                    // skipped
                    if let Ok(path_inners) = xdg_path.read_dir() {
                        let mut application_dir_results = path_inners
                            // well search all the applications instances
                            .filter(|path| match path {
                                Ok(dir_entry) => dir_entry
                                    .path()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap_or_default()
                                    .contains("applications"),
                                Err(_) => false, //fallback in weird case of not being able to open
                            })
                            .flat_map(|dir_result| match get_xdg_dir_entries(dir_result) {
                                Ok(entries) => entries,
                                Err(err) => {
                                    error!("{err}");
                                    vec![]
                                }
                            })
                            .collect::<Vec<DesktopEntry>>();

                        debug!("application_dir: {application_dir_results:?}");

                        desktop_entries.append(&mut application_dir_results);
                    }

                    // we open each of the desktop entrys and parse them to the struct
                }

                // now in case there's one new app

                let cached_entries = match load_entries_file().await {
                    Ok(entries) => entries,
                    Err(_) => {
                        error!("Couldn't load cached entries, ignoring ranking process");
                        return Box::new(desktop_entries);
                    }
                };

                if desktop_entries.len() > cached_entries.len() {
                    // find new entry and push it

                    debug!("desktop_entriess {:?}", desktop_entries);

                    return Box::new(desktop_entries);
                }

                debug!("desktop_entriess {:?}", desktop_entries);

                return Box::new(cached_entries);
            }
        }
        // TODO: add different paths for different dir with applications
        DesktopEntriesTarget::MacOS => {
            #[cfg(target_os = "macos")]
            {
                for path in vec!["/Applications", "/System/Applications"] {
                    desktop_entries.append(&mut get_application_desktop_entry(Path::new(path)));
                }

                return Box::new(desktop_entries);
            }

            panic!(
                "Desktop entry target was selected as Macos, when target Macos is not being supported."
            );
        }
    };
}

pub fn launch_application(entry: &DesktopEntry, hyper_wrapped_entries: Option<Vec<DesktopEntry>>) {
    let mut entries = hyper_wrapped_entries.unwrap().to_vec();

    // for changing rank for entries
    let change_entry = entries
        .iter_mut()
        .map(|comp_entry| {
            if comp_entry.name == entry.name {
                comp_entry.rank += 1;
            }
            return comp_entry.clone();
        })
        .collect();

    match write_entries_file(change_entry) {
        Ok(_) => {
            info!("saved file");
        }
        Err(err) => {
            error!("found {err}");
        }
    }

    match get_desktop_entry_target() {
        DesktopEntriesTarget::XDG => {
            #[cfg(target_os = "linux")]
            {
                let _ = Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "$(dex --term $TERMINAL {})",
                        entry.desktop_entry_path.to_str().unwrap()
                    ))
                    .spawn();
            }
        }
        DesktopEntriesTarget::MacOS => {
            #[cfg(target_os = "macos")]
            {
                let _ = Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "$(open '{}')",
                        entry.desktop_entry_path.to_str().unwrap()
                    ))
                    .spawn();
            }
        }
    }
    exit(1);
}
