// for parsing Toml file to the WayMac config
pub struct TomlConfig {
    main_window: MainWindow,
    inputbar: InputBar,
    entry: Entry,
}

// not a simple way to be explicit with things in here

struct MainWindow {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: String, // will be parse for RGB triplet

    pub font: String, // will fallback incase of not founding desire font

    pub location: String, // will be parsed after in the "Location Enum"

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_radius: f32,
}

struct InputBar {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: String, // will be parse for RGB triplet

    pub font: String, // will fallback incase of not founding desire font

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_radius: f32,
}

struct Entry {
    pub height: f32,
    pub width: f32,

    pub padding: f32,
    pub spacing: f32,

    pub text_color: String,       // will be parse for RGB triplet
    pub focus_text_color: String, // will be parse for RGB triplet

    pub font: String, // will fallback incase of not founding desire font

    pub background_image: Option<String>, // the path of the image will be parsed
    pub background_color: Option<String>, // same as the text color

    pub border_radius: f32,
}
