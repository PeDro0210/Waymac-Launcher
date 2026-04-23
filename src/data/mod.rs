/*Widgets Ids*/

use iced::Color;

pub const LAUNCHER_TEXT_INPUT_ID: &str = "LauncherTextInputID";
pub const LAUNCHER_CONTAINER_ID: &str = "LauncherID";

pub const LAUNCHER_SCROLLABLE_ID: &str = "LauncherScrollableID";

/*Widgets Ids*/

/*Widgets various*/

pub const MAIN_ENTRY_FOCUS_IDX: usize = 0;

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

// this is for general apps for macos
pub const MAX_DEPTH_MACOS_APPLICATION_DIR: usize = 1;
/* Mac related */
