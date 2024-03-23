use std::collections::HashSet;
use gdnative::prelude::*;
use util::{fn_name, full_fn_name};
use util_gdnative::prelude::ErrInspector;
use crate::game_manager::{GameManager, GameState};
use crate::WorldLocation;
use crate::save::file::SaveFile;
use crate::world_map::{SIGNAL_MARKER_CLICKED, SIGNAL_LINE_CLICKED, SIGNAL_OPEN_SETTINGS_MENU, SIGNAL_OPEN_CHARACTER_MENU, WorldPath, WorldMapController};

fn get_unlocked_paths(save_file: &SaveFile) -> HashSet<WorldPath> {
	return save_file
		.unlocked_locations()
		.iter()
		.flat_map(|location|
			location.available_connections(save_file)
			        .iter()
			        .filter_map(|destination| WorldPath::new(*location, *destination)))
		.collect();
}

#[derive(Debug)]
pub enum WorldMapState {
	Idle(SaveFile),
	SettingsMenu(SaveFile),
	CharacterMenu(SaveFile),
	Event(SaveFile, String),
	Combat { save: SaveFile, scene_on_win: String, scene_on_loss: String },
	LoadingLocalMap(SaveFile),
}

#[methods(mixin = "GM", pub)]
impl GameManager {
	fn world_map_register_signals(gm_owner_ref: Ref<Node>, world_map_owner: TRef<Control>) {
		world_map_owner.connect(SIGNAL_MARKER_CLICKED, gm_owner_ref, fn_name(&Self::world_map_marker_clicked), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		world_map_owner.connect(SIGNAL_LINE_CLICKED, gm_owner_ref, fn_name(&Self::world_map_line_clicked), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		world_map_owner.connect(SIGNAL_OPEN_SETTINGS_MENU, gm_owner_ref, fn_name(&Self::world_map_open_settings_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		world_map_owner.connect(SIGNAL_OPEN_CHARACTER_MENU, gm_owner_ref, fn_name(&Self::world_map_open_character_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}

	fn load_local_map(&mut self, save: SaveFile, world_map_instance: Instance<WorldMapController>, _path: WorldPath) {
		self.state = GameState::WorldMap(world_map_instance, WorldMapState::LoadingLocalMap(save));
		// todo! load local map
	}

	#[method]
	fn world_map_marker_clicked(&mut self, location: WorldLocation) {
		let (world_map, save) =
			match std::mem::take(&mut self.state) {
				GameState::WorldMap(world_map,
					WorldMapState::Idle(save)) => (world_map, save),
				other_state => {
					godot_warn!("{}():\n Cannot process marker clicked from WorldMap when state is: {other_state:?}.",
						full_fn_name(&Self::world_map_open_settings_menu));
					self.state = other_state;
					return;
				}
			};
		
		if location == save.map_location() 
			&& let Some(path) = get_unlocked_paths(&save)
			.iter()
			.find(|path| path.contains(location))
			.cloned() {
			self.load_local_map(save, world_map, path);
		} else {
			self.state = GameState::WorldMap(world_map, WorldMapState::Idle(save));
		}
	}
	
	#[method]
	fn world_map_line_clicked(&mut self, path: WorldPath) {
		let (world_map, save) =
			match std::mem::take(&mut self.state) {
				GameState::WorldMap(world_map,
					WorldMapState::Idle(save)) => (world_map, save),
				other_state => {
					godot_warn!("{}():\n Cannot process line clicked from WorldMap when state is: {other_state:?}.",
						full_fn_name(&Self::world_map_open_settings_menu));
					self.state = other_state;
					return;
				}
			};
		
		if path.contains(save.map_location()) 
			&& get_unlocked_paths(&save).contains(&path) {
			self.load_local_map(save, world_map, path);
		} else {
			self.state = GameState::WorldMap(world_map, WorldMapState::Idle(save));
		}
	}
	
	#[method]
	fn world_map_open_settings_menu(&mut self) {
		let (world_map, save) =
			match std::mem::take(&mut self.state) {
				GameState::WorldMap(world_map, 
					WorldMapState::Idle(save) | WorldMapState::CharacterMenu(save)) => (world_map, save),
				other_state => {
					godot_warn!("{}():\n Cannot open SettingsMenu from WorldMap when state is: {other_state:?}.",
						full_fn_name(&Self::world_map_open_settings_menu));
					self.state = other_state;
					return;
				}
			};

		self.state = GameState::WorldMap(world_map, WorldMapState::SettingsMenu(save));
		self.open_settings_menu();
	}

	#[method]
	fn world_map_open_character_menu(&mut self) {
		let (world_map, save) =
			match std::mem::take(&mut self.state) {
				GameState::WorldMap(world_map, 
					WorldMapState::Idle(save) | WorldMapState::SettingsMenu(save)) => (world_map, save),
				other_state => {
					godot_warn!("{}():\n Cannot open CharacterMenu from WorldMap when state is: {other_state:?}.",
						full_fn_name(&Self::world_map_open_settings_menu));
					self.state = other_state;
					return;
				}
			};

		self.state = GameState::WorldMap(world_map, WorldMapState::CharacterMenu(save));
		// todo! open character menu
	}
}
