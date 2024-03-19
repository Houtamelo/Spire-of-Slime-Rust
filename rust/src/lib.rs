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
#![warn(clippy::missing_const_for_fn)]
#![feature(step_trait)]
#![feature(let_chains)]
#![feature(const_type_name)]
#![feature(const_option)]
#![feature(hash_extract_if)]
#![feature(ascii_char)]
#![feature(variant_count)]

use gdnative::prelude::*;

pub use audio::bus;
pub use world_map::WorldLocation;

mod audio;
mod combat;
mod util;
mod main_menu;
mod settings_menu;
mod save;
mod world_map;
mod local_map;
mod game_manager;
mod start_screen;
pub mod gdnative_macros;

pub const MAX_CHARACTERS_PER_TEAM: u8 = 4;
pub static CONFIG_PATH: &str = "user://config.cfg";

fn init_panic_hook() {
	let old_hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |panic_info| {
		let loc_string;
		if let Some(location) = panic_info.location() {
			loc_string = format!("file '{}' at line {}", location.file(), location.line());
		} else {
			loc_string = "unknown location".to_owned()
		}

		let error_message;
		if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else {
			error_message = format!("[RUST] {}: unknown panic occurred", loc_string);
		}
		godot_error!("{}", error_message);
		(*(old_hook.as_ref()))(panic_info);

		unsafe {
			if let Some(gd_panic_hook) = autoload::<Node>("rust_panic_hook") {
				gd_panic_hook.call("rust_panic_hook", &[GodotString::from_str(error_message).to_variant()]);
			}
		}
	}));
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	init_panic_hook();
	handle.add_class::<start_screen::StartScreenController>();
	handle.add_class::<combat::ui::SpeedButtons>();
	handle.add_class::<combat::ui::UI_CharacterStats>();
	handle.add_class::<combat::ui::TargetingTooltip>();
	handle.add_class::<game_manager::GameManager>();
	handle.add_class::<util::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenuController>();
	handle.add_class::<main_menu::LoadButton>();
	handle.add_class::<world_map::WorldMapController>();
	handle.add_class::<local_map::generation::generator_ui::MapGeneratorUI>();
	handle.add_class::<local_map::generation::generator_ui::BiomeDataResource>();
	handle.add_class::<houta_utils_gdnative::prelude::PitchRandomizer>();
	handle.add_class::<houta_utils_gdnative::prelude::PlayOnClickAndPitchRandomizer>();
	handle.add_class::<houta_utils_gdnative::prelude::PlayOnHoverAndPitchRandomizer>();
	handle.add_class::<houta_utils_gdnative::prelude::DisallowClickFocusOnParent>();
	handle.add_class::<houta_utils_gdnative::prelude::AutoTextResize>();
}

godot_init!(init);