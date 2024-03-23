mod state_main_menu;
mod state_world_map;

use gdnative::api::*;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use util_gdnative::prelude::*;
use main_menu::MainMenuController;
use state_main_menu::MainMenuState;
use state_world_map::WorldMapState;
use world_map::WorldMapController;

use crate::{main_menu, world_map};
use crate::save::SaveFilesController;
use crate::gdnative_macros::seek_tree_and_create_tween;
use crate::settings_menu::SettingsMenuController;

macro_rules! spawn_scene {
    ($input: ident, $output_base: ty, $output_ty: ty) => {{
	    let resource = unsafe { $input.assume_safe() };
		let scene = resource
			.cast::<PackedScene>()
			.unwrap();
		let node_ref = scene
			.instance(0)
			.unwrap();
		let node = unsafe { node_ref.assume_safe() };
		let base_type = node
			.cast::<$output_base>()
			.unwrap();
		let instance = base_type
			.cast_instance::<$output_ty>()
			.unwrap();
		instance.claim()
    }};
}

const SCENE_PATH_MAIN_MENU: &str = "res://Core/Main Menu/scene_main-menu.tscn";
const SCENE_PATH_SETTINGS_MENU: &str = "res://Core/Settings Menu/scene_settings-menu.tscn";
const SCENE_PATH_CHARACTER_MENU: &str = "res://Core/Character Menu/scene_character-menu.tscn";
const SCENE_PATH_LOCAL_MAP: &str = "res://Core/Local Map/scene_local-map.tscn";
const SCENE_PATH_WORLD_MAP: &str = "res://Core/World Map/scene_world-map.tscn";

#[derive(Debug, Default)]
pub(super) enum GameState {
	#[default] AfterStartScreen,
	MainMenu(Instance<MainMenuController>, MainMenuState),
	WorldMap(Instance<WorldMapController>, WorldMapState),
}

#[extends(Node)]
#[derive(Debug)]
#[register_with(register)]
pub struct GameManager {
	#[export_path] fade_screen: Option<Ref<Control>>,
	#[export_path] session_load_sound: Option<Ref<AudioStreamPlayer>>,
	#[export_path] scenes_container: Option<Ref<Node>>,
	
	settings_menu: Option<Instance<SettingsMenuController>>,
	state: GameState,
	save_files: SaveFilesController,
}

fn register(builder: &ClassBuilder<GameManager>) {
	builder.mixin::<state_main_menu::GM>();
}

#[methods]
impl GameManager {
	#[method]
	fn _ready(&mut self, #[base] owner: &Node) {
		self.grab_nodes_by_path(owner);
		self.save_files.load_saves_from_disk();
		
		let owner_ref = unsafe { owner.assume_shared() };

		{
			let settings_menu_res_ref = ResourceLoader::godot_singleton()
				.load(SCENE_PATH_SETTINGS_MENU, "PackedScene", false)
				.unwrap();
			let settings_menu_ref = spawn_scene!(settings_menu_res_ref, CanvasLayer, SettingsMenuController);
			let settings_menu_base_ref = settings_menu_ref.base();
			let settings_menu_base = unsafe { settings_menu_base_ref.assume_safe() };
			settings_menu_base.hide();
			self.settings_menu = Some(settings_menu_ref);
		}

		{

			let main_menu_res_ref = ResourceLoader::godot_singleton()
				.load(SCENE_PATH_MAIN_MENU, "PackedScene", false)
				.unwrap();
			let main_menu_ref =
				spawn_scene!(main_menu_res_ref, Control, MainMenuController);

			main_menu_ref.touch_assert_safe_mut(|main_menu, main_menu_owner| {
				main_menu.create_and_assign_load_buttons(main_menu_owner, self.save_files.get_saves());
			});
			
			let main_menu_base_ref = main_menu_ref.base();
			let main_menu_base = unsafe { main_menu_base_ref.assume_safe() };
			Self::main_menu_register_signals(owner_ref, main_menu_base);
			self.state = GameState::MainMenu(main_menu_ref, MainMenuState::Idle);
		}

		{
			let fade_screen_ref = self.fade_screen.unwrap();
			let tween_ref = seek_tree_and_create_tween!(owner);
			let tween = unsafe { tween_ref.assume_safe() };
			tween.tween_property(fade_screen_ref, "modulate:a", 1., 2.);
			tween.connect("finished", fade_screen_ref, "hide", 
					VariantArray::new_shared(), 0)
				.log_if_err();
		}
	}
	
	pub fn open_settings_menu(&self) {
		self.settings_menu.touch_assert_safe_mut(|settings_menu, settings_menu_owner| {
			settings_menu._open_panel(settings_menu_owner);
		});
	}
}