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

mod audio;
pub use audio::bus;

mod combat;
mod util;
mod main_menu;
mod settings_menu;
mod game_states;
pub mod save;

use gdnative::prelude::*;
use gdnative::api::*;
use gdrust_export_nodepath::extends;

pub const MAX_CHARACTERS_PER_TEAM: usize = 4;
pub static config_path: &str = "user://config.cfg";

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	handle.add_class::<GameManager>();
	handle.add_class::<util::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenu>();
	handle.add_class::<main_menu::LoadButton>();
	handle.add_class::<save::SavesManager>();
}

godot_init!(init);

#[extends(Node)]
pub struct GameManager {
	state: game_states::GameState,
}

#[methods]
impl GameManager {
	pub fn godot_singleton() -> TInstance<'static, GameManager, Shared> {
		let engine = Engine::godot_singleton();
		let singleton_base_obj = engine.get_singleton("SavesManager").expect("Failed to get singleton SavesManager");
		let singleton_tref = unsafe { singleton_base_obj.assume_safe() };
		let singleton_node = singleton_tref.cast::<Node>().expect("Failed to cast singleton object GameManager to Node");
		return singleton_node.cast_instance::<GameManager>().expect("Failed to cast singleton node to GameManager");
	}

	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.grab_nodes_by_path(_owner);
	}

	#[method]
	fn main_menu_to_settings(&mut self) {
		match &self.state {
			game_states::GameState::StartScreen => { godot_error!("main_menu_to_settings: Trying to open SettingsMenu from MainMenu but state is StartScreen"); },
			game_states::GameState::MainMenu(main_menu_state) => {
				match main_menu_state {
					game_states::main_menu::MainMenuState::MainMenu => todo!(),
					game_states::main_menu::MainMenuState::Settings  => { godot_warn!("main_menu_to_settings: Trying to open SettingsMenu from MainMenu but it's already open.") },
				}
			},
		}
	}
}