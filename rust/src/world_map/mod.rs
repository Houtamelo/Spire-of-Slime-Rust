mod controller;
mod location;
mod path;

pub use controller::WorldMapController;
pub use location::WorldLocation;
pub use path::WorldPath;

pub const SIGNAL_OPEN_SETTINGS_MENU: &str = "open_settings_menu";
pub const SIGNAL_OPEN_CHARACTER_MENU: &str = "open_character_menu";
pub const SIGNAL_MARKER_CLICKED: &str = "marker_clicked";
pub const SIGNAL_LINE_CLICKED: &str = "line_clicked";

