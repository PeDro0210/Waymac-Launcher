use std::{
    env::consts::OS,
    fs::{DirEntry, File, ReadDir, read_dir},
    io::{Error, Read},
    panic,
    path::Path,
};

use walkdir::WalkDir;

use crate::data::MAX_DEPTH_APPLICATION_DIR;

use log::{error, info, warn};

use crate::app_launcher::DesktopEntry;

pub enum DesktopEntriesTarget {
    MacOS,
    XDG,
}

pub fn get_desktop_entry_target() -> DesktopEntriesTarget {
    match OS {
        "linux" => DesktopEntriesTarget::XDG, // if u ain't using XDG, then this ain't your place
        "macos" => DesktopEntriesTarget::MacOS,
        _ => panic!("Desktop Entry Target not supported"),
    }
}

#[cfg(target_os = "linux")]
pub fn get_xdg_dir_entries(dir_result: Result<DirEntry, Error>) -> Vec<DesktopEntry> {
    let mut dir_desktop_entries = Vec::new();

    if let Ok(dir) = dir_result {
        let dir_contents = read_dir(dir.path()).into_iter();
        for dir in dir_contents {
            for desktop_entry_result in dir {
                let desktop_entry = desktop_entry_result.unwrap(); // if the desktop entry
                // exists, if cause of something (if it panics I'll see what to change in case)

                if let Ok(mut desktop_entry_file) = File::open(desktop_entry.path()) {
                    let file_contents = &mut String::new();

                    // getting the content from the entry
                    let _ = desktop_entry_file.read_to_string(file_contents);

                    let desktop_file =
                        freedesktop_file_parser::parse(file_contents).unwrap_or_default();

                    dir_desktop_entries.push(DesktopEntry {
                        name: desktop_file.entry.name.default,
                        icon: desktop_file.entry.icon.unwrap_or_default().content,
                        desktop_entry_path: Box::new(desktop_entry.path()),
                    });

                    info!("file for {:?}: {:?}", desktop_entry.path(), file_contents);
                }
            }
        }
        return dir_desktop_entries;
    }

    panic!("dir result couldn't be open");
}

#[cfg(target_os = "macos")]
pub fn get_application_desktop_entry(path: &Path) -> Vec<DesktopEntry> {
    let mut application_dir_entries = Vec::new();

    let walker = WalkDir::new(path).max_depth(MAX_DEPTH_APPLICATION_DIR);

    for entry in walker {
        if let Ok(entry) = entry {
            if entry.path().is_dir() {
                continue;
            }
            if let Some(ext) = entry.path().extension() {
                if ext != "plist" {
                    continue;
                }

                info!("path: {:?}", entry.path());
            }
        }
    }
    application_dir_entries
}
