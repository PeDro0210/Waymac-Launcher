use std::env::consts::OS;

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
