use iced::{Background, Color, Font, Size, advanced::graphics::Image};

// config struct for using directly in WayMacApp
pub struct WayMacConfig {
    pub main_font: Font,
    pub text_color: Color,
    pub main_window: ContainerConfig,
    pub input_bar: ContainerConfig,
    pub entry: ContainerConfig,
}

// each different type of container that WayMac has
pub enum ContainerType {
    MainWindow {
        // location depending the layer
        location: Location,
        padding: f32,
        spacing: f32,
    },
    InputBar,
    Entry {
        focus_text_color: Color,
    },
}

pub enum Location {
    Center,
    Top,
    Bottom,
    Right,
    Left,
}

// describes the behavoir of WayMac components
pub struct ContainerConfig {
    pub size: Size,

    pub text_color: Color,
    pub font: Font,

    pub background: Background,

    pub border_image: Option<Image>,
    pub border_radius: f32,

    // other configs depending which container is it
    pub specific: ContainerType,
}
