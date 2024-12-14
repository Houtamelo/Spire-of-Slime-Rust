use std::fmt::{Debug, Formatter};

use godot::classes::resource_loader::CacheMode;
use main_menu::prelude::MainMenuController;
use serialization::prelude::*;
use settings_menu::prelude::SettingsMenuController;
use state_main_menu::MainMenuState;
use state_world_map::WorldMapState;
use world_map::prelude::WorldMapController;

use super::*;

mod state_main_menu;
mod state_world_map;

#[allow(unused)] //todo!
const SCENE_PATH_CHARACTER_MENU: &str = "res://Core/Character Menu/scene_character-menu.tscn";
#[allow(unused)] //todo!
const SCENE_PATH_WORLD_MAP: &str = "res://Core/World Map/scene_world-map.tscn";

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct GameManager {
	base: Base<Node>,
	#[init(node = "fade_screen")]
	fade_screen: OnReady<Gd<Control>>,
	#[init(node = "session_load_sound")]
	session_load_sound: OnReady<Gd<AudioStreamPlayer>>,
	#[init(node = "scenes")]
	scenes_container: OnReady<Gd<Node>>,
	#[init(node = "settings_menu")]
	settings_menu: OnReady<Gd<SettingsMenuController>>,
	state: GameState,
	save_files: SaveFilesController,
}

#[godot_api]
impl INode for GameManager {
	fn ready(&mut self) {
		self.save_files.load_saves_from_disk();
		self.load_main_menu();

		self.fade_screen
			.do_fade(1., 2.)
			.on_finish_callable(Callable::from_object_method(&self.fade_screen, "hide"))
			.register();
	}
}

impl GameManager {
	pub fn open_settings_menu(&mut self) { self.settings_menu.bind_mut().open_panel(); }
}

#[derive(Default)]
enum GameState {
	#[default]
	AfterStartScreen,
	MainMenu(Gd<MainMenuController>, MainMenuState),
	#[allow(unused)] //todo!
	WorldMap(Gd<WorldMapController>, WorldMapState),
}

impl Debug for GameState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			GameState::AfterStartScreen => write!(f, "AfterStartScreen"),
			GameState::MainMenu(_, main_menu_state) => write!(f, "MainMenu({main_menu_state:?})"),
			GameState::WorldMap(_, world_map_state) => write!(f, "WorldMap({world_map_state:?})"),
		}
	}
}
