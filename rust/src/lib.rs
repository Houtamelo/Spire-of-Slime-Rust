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

mod audio;
pub use audio::bus;

mod combat;
mod util;
mod main_menu;
mod settings_menu;
mod game_states;
pub mod saves;

use gdnative::prelude::*;
use gdnative::api::*;
use gdrust_export_nodepath::extends;
use houta_utils_gdnative::prelude::*;
use game_states::GameState;
use game_states::main_menu::MainMenuState;

pub const MAX_CHARACTERS_PER_TEAM: usize = 4;
pub static config_path: &str = "user://config.cfg";

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	handle.add_class::<GameManager>();
	handle.add_class::<util::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenu>();
	handle.add_class::<main_menu::LoadButton>();
	handle.add_class::<saves::SavesManager>();
}

godot_init!(init);

#[extends(Node)]
pub struct GameManager {
	state: GameState,
	#[export_path] settings_menu: Option<Instance<settings_menu::SettingsMenu>>,
	#[export_path] fade_screen  : Option<Ref<Control>>,
	#[export_path] session_load_sound: Option<Ref<AudioStreamPlayer>>,
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
			GameState::StartScreen => { godot_error!("main_menu_to_settings: Trying to open SettingsMenu from MainMenu but state is StartScreen"); },
			GameState::MainMenu(main_menu, main_menu_state) => {
				match main_menu_state {
					MainMenuState::Idle => {
						unsafe { self.settings_menu.unwrap_inst().base().call_deferred(settings_menu::CALL_OPEN_PANEL, &[]); };
						self.state = GameState::MainMenu(main_menu.clone(), MainMenuState::Settings);
					},
					MainMenuState::LoadingSave { .. } => { godot_warn!("main_menu_to_settings: Trying to open SettingsMenu from MainMenu but it's already loading a save.") },
					MainMenuState::Settings  => { godot_warn!("main_menu_to_settings: Trying to open SettingsMenu from MainMenu but it's already open.") },
				}
			}
		}
	}

	#[method]
	fn main_menu_load_game(&mut self, #[base] owner: &Node, save_name: GodotString) {
		match &self.state {
			GameState::StartScreen => {}
			GameState::MainMenu(main_menu, state) => {
				match state {
					MainMenuState::LoadingSave { .. } => { godot_warn!("main_menu_load_game: Trying to load a save from MainMenu but it's already loading a save."); }
					MainMenuState::Settings => { godot_warn!("main_menu_load_game: Trying to load a save from MainMenu but SettingsMenu is open."); }
					MainMenuState::Idle => {
						let self_ref = unsafe { owner.assume_shared() };
						
						self.session_load_sound.unwrap_manual().play(0.0);
						
						let fade_screen = self.fade_screen.unwrap_manual();
						fade_screen.show();
						fade_screen.set_modulate(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 });
						
						let main_loop = Engine::godot_singleton().get_main_loop();
						let scene_tree: TRef<SceneTree> = main_loop.unwrap_manual().cast::<SceneTree>().unwrap();
						let tween_option = scene_tree.create_tween();
						let tween: TRef<SceneTreeTween> = tween_option.unwrap_refcount();
						tween.tween_property(fade_screen, "modulate", Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }, 2.0);
						
						tween.connect("finished", self_ref, "_on_main_menu_session_load_fade_complete", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
						
						self.state = GameState::MainMenu(main_menu.clone(), MainMenuState::LoadingSave { save_name: save_name.to_string()})
					}
				}
			}
		}
	}
	
	#[method]
	fn _on_main_menu_session_load_fade_complete(&mut self) {
		match &self.state {
			GameState::StartScreen => {}
			GameState::MainMenu(main_menu, state) => {
				match state {
					MainMenuState::Idle => { godot_warn!("_on_main_menu_session_load_fade_complete: MainMenuState::Idle"); }
					MainMenuState::Settings => { godot_warn!("_on_main_menu_session_load_fade_complete: MainMenuState::Settings"); }
					MainMenuState::LoadingSave { save_name } => {
						let main_menu_inst = main_menu.unwrap_inst();
						let main_menu_base = main_menu_inst.base();
						if let Some(parent) = main_menu_base.get_parent().map(|parent| unsafe { parent.assume_safe() }) {
							parent.remove_child(main_menu_base);
						}

						main_menu_base.queue_free();
						// todo!
					}
				}
			}
		}
	}
}