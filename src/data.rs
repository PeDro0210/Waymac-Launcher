/*Configs default variables*/

// this have take in mind being used with the push method of the PathBuf
pub const DEFAULT_CONFIG_PATH_EXTENSION: &str = "~/.config/waymac/waymac.toml";

pub const DEFAULT_DEBUG_DUMP_PATH_EXTENSION: &str = "~/.local/state/waymac.log";
// this have take in mind being used with the push method of the PathBuf

/*Configs default variables*/

/*Widgets Ids*/

use iced::Color;

pub const LAUNCHER_TEXT_INPUT_ID: &str = "LauncherTextInputID";
pub const LAUNCHER_CONTAINER_ID: &str = "LauncherID";

pub const LAUNCHER_SCROLLABLE_ID: &str = "LauncherScrollableID";

/*Widgets Ids*/

/*Widgets various*/

pub const MAIN_ENTRY_FOCUS_IDX: usize = 0;

pub const ENTRY_ELEMENTS_HEIGHT: f32 = 25.;

/*Widgets various*/

/*Widgets Colors*/

//TODO: implement config color toggle
pub const ENTRY_FOCUS_COLOR: Color = Color {
    r: 1.,
    b: 0.2,
    g: 0.5,
    a: 1.,
};

/*Widgets Colors*/

/* Mac related */

// being 2 levels normally and the third one for the Content dir
// for checking application directories
pub const MAX_DEPTH_APPLICATION_DIR: usize = 2;

/* Mac related */
