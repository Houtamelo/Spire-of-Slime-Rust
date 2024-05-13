#![feature(anonymous_lifetime_in_impl_trait)]

mod easters_save_name;
mod easters_iron_gauntlet;

mod load_button;

pub use load_button::LoadButton;

mod controller;

pub use controller::MainMenuController;
pub use controller::{
	SIGNAL_NEW_GAME,
	SIGNAL_LOAD_GAME,
	SIGNAL_DELETE_SAVE,
	SIGNAL_OVERWRITE_SAVE_AND_START,
	SIGNAL_OPEN_SETTINGS_MENU,
};

pub(crate) mod internal_prelude {
	pub use util_gdnative::prelude::*;
	pub use util::prelude::*;
	pub use shared::panel_are_you_sure::PanelAreYouSure;
	pub use shared::panel_are_you_sure;
}
