mod controller;
mod location;

pub use controller::WorldMapController;
pub use location::WorldLocation;

pub const SIGNAL_OPEN_SETTINGS_MENU: &str = "open_settings_menu";
pub const SIGNAL_OPEN_CHARACTER_MENU: &str = "open_character_menu";
pub const SIGNAL_LOCATION_CLICKED: &str = "location_clicked";

