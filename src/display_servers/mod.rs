use std::env::{self, consts::OS};

pub mod quartz;
pub mod wayland;

pub enum SupportedDisplayServer {
    Wayland,
    Xorg, // will also map for the macos target
    Quartz,
}

pub fn get_supported_display_server_target() -> SupportedDisplayServer {
    match env::var_os("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
    {
        "wayland" => SupportedDisplayServer::Wayland,
        "x11" => SupportedDisplayServer::Xorg,
        "tty" => panic!("Couldn't get valid XDG_SESSION_TYPE, found tty"),
        _ => {
            return match OS {
                "linux" => panic!("Couldn't get valid XDG_SESSION_TYPE on linux target"),
                "macos" => SupportedDisplayServer::Quartz, // will treat it as an XORG app
                _ => panic!("OS not supported"),
            };
        }
    }
}
