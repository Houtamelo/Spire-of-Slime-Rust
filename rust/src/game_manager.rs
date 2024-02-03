use gdnative::api::*;
use gdnative::prelude::*;
use houta_utils_gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use crate::game_states::GameState;
use crate::game_states::state_main_menu::MainMenuState;
use crate::{settings_menu, util};
use crate::game_states::state_world_map::WorldMapState;
use crate::settings_menu::SettingsMenu;

#[extends(Node)]
#[derive(Debug)]
pub struct GameManagerSingleton {
	state: GameState,
	#[export_path] settings_menu: Option<Instance<SettingsMenu>>,
	#[export_path] fade_screen  : Option<Ref<Control>>,
	#[export_path] session_load_sound: Option<Ref<AudioStreamPlayer>>,
}

#[methods]
impl GameManagerSingleton {
	pub fn godot_singleton() -> TInstance<'static, GameManagerSingleton, Shared> {
		let engine = Engine::godot_singleton();
		let singleton_base_obj = engine.get_singleton("SavesManager").expect("Failed to get singleton SavesManager");
		let singleton_tref = unsafe { singleton_base_obj.assume_safe() };
		let singleton_node = singleton_tref.cast::<Node>().expect("Failed to cast singleton object GameManager to Node");
		return singleton_node.cast_instance::<GameManagerSingleton>().expect("Failed to cast singleton node to GameManager");
	}

	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		self.grab_nodes_by_path(_owner);
	}
	
	// todo! When entering main menu, register signals
	// todo! When entering world map, register signals

	#[method]
	fn main_menu_to_settings_menu(&mut self) {
		let GameState::MainMenu(main_menu, main_menu_state) = std::mem::take(&mut self.state)
			else {
				godot_error!("GameManager: main_menu_to_settings(): Called open SettingsMenu from MainMenu but state is: {:?}", self.state);
				return;
			};

		match main_menu_state {
			MainMenuState::Idle => {
				self.state = GameState::MainMenu(main_menu, MainMenuState::SettingsMenu);
				unsafe { 
					self.settings_menu
						.unwrap_inst()
						.base()
						.call_deferred(settings_menu::CALL_OPEN_PANEL, &[]); 
				};
			}
			any_state  => {
				godot_warn!("GameManager: main_menu_to_settings(): \n\
					Called open SettingsMenu from MainMenu but state is: {any_state:?}.");
				self.state = GameState::MainMenu(main_menu, any_state);
			}
		}
	}

	#[method]
	fn main_menu_load_game(&mut self, #[base] owner: &Node, save_name: GodotString) {
		let GameState::MainMenu(main_menu, main_menu_state) = std::mem::take(&mut self.state)
			else {
				godot_error!("GameManager: main_menu_to_settings(): Called open SettingsMenu from MainMenu but state is: {:?}", self.state);
				return;
			};
		
		match main_menu_state {
			MainMenuState::Idle => {
				self.state = GameState::MainMenu(main_menu, MainMenuState::LoadingSave { save_name: save_name.to_string()});
				
				let game_manager_ref = unsafe { owner.assume_shared() };
				
				self.session_load_sound.unwrap_manual().play(0.0);
				
				let fade_screen = self.fade_screen.unwrap_manual();
				fade_screen.show();
				fade_screen.set_modulate(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 });
				
				let main_loop = Engine::godot_singleton().get_main_loop();
				let scene_tree: TRef<SceneTree> = main_loop.unwrap_manual()
					.cast::<SceneTree>()
					.unwrap();
				let tween_option = scene_tree.create_tween();
				let tween: TRef<SceneTreeTween> = tween_option.unwrap_refcount();
				tween.tween_property(fade_screen, "modulate", 
					Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }, 2.0);
				
				tween.connect("finished", game_manager_ref, util::fn_name(&Self::_main_menu_save_load_fade_complete),
						VariantArray::new_shared(), Object::CONNECT_DEFERRED)
					.log_if_err();
			}
			any_state  => {
				godot_warn!("GameManager: main_menu_to_settings(): \n\
					Called open SettingsMenu from MainMenu but state is: {any_state:?}.");
				self.state = GameState::MainMenu(main_menu, any_state);
			}
		}
	}
	
	#[method]
	fn _main_menu_save_load_fade_complete(&mut self) {
		let GameState::MainMenu(main_menu, main_menu_state) = std::mem::take(&mut self.state)
			else {
				godot_error!("GameManager: main_menu_to_settings(): Called open SettingsMenu from MainMenu but state is: {:?}", self.state);
				return;
			};
		
		match main_menu_state {
			MainMenuState::LoadingSave { .. } => {
				let main_menu_inst = main_menu.unwrap_inst();
				let main_menu_base = main_menu_inst.base();
				main_menu_base.get_parent()
					.unwrap_manual()
					.remove_child(main_menu_base);

				main_menu_base.queue_free();
				// todo! remember to set new state
			},
			any_state  => {
				godot_warn!("GameManager: main_menu_to_settings(): \n\
					Called open SettingsMenu from MainMenu but state is: {any_state:?}.");
				self.state = GameState::MainMenu(main_menu, any_state);
			}
		}
	}
	
	#[method]
	fn world_map_to_settings_menu(&mut self) {
		let GameState::WorldMap(world_map, world_map_state) = std::mem::take(&mut self.state)
			else {
				godot_error!("GameManager: world_map_to_settings_menu(): Called open SettingsMenu from WorldMap but state is: {:?}", self.state);
				return;
			};
		
		match world_map_state {
			WorldMapState::Idle => {
				self.state = GameState::WorldMap(world_map, WorldMapState::SettingsMenu);
				unsafe { 
					self.settings_menu
						.unwrap_inst()
						.base()
						.call_deferred(settings_menu::CALL_OPEN_PANEL, &[]); 
				};
			},
			any_state  => {
				godot_warn!("GameManager: world_map_to_settings_menu(): \n\
					Called open SettingsMenu from WorldMap but state is: {any_state:?}.");
				self.state = GameState::WorldMap(world_map, any_state);
			}
		}
	}
	
	#[method]
	fn world_map_to_character_menu(&mut self) {
		let GameState::WorldMap(world_map, world_map_state) = std::mem::take(&mut self.state)
			else {
				godot_error!("GameManager: world_map_to_character_menu(): Called open CharacterMenu from WorldMap but state is: {:?}", self.state);
				return;
			};
		
		match world_map_state {
			WorldMapState::Idle => {
				self.state = GameState::WorldMap(world_map, WorldMapState::CharacterMenu);
				// todo! call character menu
			},
			any_state  => {
				godot_warn!("GameManager: world_map_to_character_menu(): \n\
					Called open CharacterMenu from WorldMap but state is: {any_state:?}.");
				self.state = GameState::WorldMap(world_map, any_state);
			}
		}
	}
}
