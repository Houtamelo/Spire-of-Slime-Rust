mod easters_save_name;
mod easters_iron_gauntlet;
mod load_button;
mod controller;

pub use load_button::LoadButton;
pub use controller::MainMenuController;

pub const SIGNAL_NEW_GAME: &str = "new_game";
pub const SIGNAL_LOAD_GAME: &str = "load_game";
pub const SIGNAL_OPEN_SETTINGS_MENU: &str = "open_settings_menu";
