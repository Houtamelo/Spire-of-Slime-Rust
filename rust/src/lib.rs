#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(clippy::needless_return)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::len_zero)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::never_loop)]
#![allow(clippy::clone_on_copy)]
#![feature(step_trait)]
#![feature(const_trait_impl)]
#![feature(let_chains)]
#![feature(result_option_inspect)]

mod combat;
mod util;
mod main_menu;
pub mod save;

pub const MAX_CHARACTERS_PER_TEAM: usize = 4;
pub const config_path: &str              = "user://config.cfg";

use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	handle.add_class::<GameManager>();
	handle.add_class::<util::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenu>();
	handle.add_class::<main_menu::load_button::LoadButton>();
	handle.add_class::<save::SavesManager>();
}

godot_init!(init);

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameManager {
}

#[methods]
impl GameManager {
	fn new(_owner: &Node) -> Self {
		Self { }
	}

	#[method]
	fn time_planner_button_pressed(&mut self) {

	}
}