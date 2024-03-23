use std::collections::{HashMap, HashSet};

use gdnative::api::{Area2D, Light2D};
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use util::{fn_name, full_fn_name};
use util_gdnative::prelude::*;

use crate::misc;

use super::location::WorldLocation;
use super::WorldPath;

pub const SIGNAL_OPEN_SETTINGS_MENU: &str = "open_settings_menu";
pub const SIGNAL_OPEN_CHARACTER_MENU: &str = "open_character_menu";
pub const SIGNAL_MARKER_CLICKED: &str = "marker_clicked";
pub const SIGNAL_LINE_CLICKED: &str = "line_clicked";

#[extends(Node)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct WorldMapController {
	#[export_path] player_icon: Option<Ref<Node2D>>,
	player_location: Option<WorldLocation>,
	
	#[export_path] marker_chapel: Option<Ref<Area2D>>,
	#[export_path] marker_grove: Option<Ref<Area2D>>,
	#[export_path] marker_cave: Option<Ref<Area2D>>,
	#[export_path] marker_forest: Option<Ref<Area2D>>,
	mapped_markers: HashMap<WorldLocation, Ref<Area2D>>,

	#[export_path] light_chapel: Option<Ref<Light2D>>,
	#[export_path] light_grove: Option<Ref<Light2D>>,
	#[export_path] light_cave: Option<Ref<Light2D>>,
	#[export_path] light_forest: Option<Ref<Light2D>>,
	mapped_lights: HashMap<WorldLocation, Ref<Light2D>>,
	
	#[export_path] line_chapel_grove: Option<Ref<Area2D>>,
	#[export_path] line_grove_forest: Option<Ref<Area2D>>,
	#[export_path] line_forest_cave: Option<Ref<Area2D>>,
	#[export_path] line_cave_chapel: Option<Ref<Area2D>>,
	mapped_lines: HashMap<WorldPath, Ref<Area2D>>,
	
	#[export_path] button_settings_menu: Option<Ref<Button>>,
	#[export_path] button_character_menu: Option<Ref<Button>>,
}

#[methods]
impl WorldMapController {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_OPEN_SETTINGS_MENU).done();
		builder.signal(SIGNAL_OPEN_CHARACTER_MENU).done();
		builder.signal(SIGNAL_MARKER_CLICKED)
			.with_param("location", VariantType::Object)
			.done();
		builder.signal(SIGNAL_LINE_CLICKED)
			.with_param("path", VariantType::Object)
			.done();
	}
	
	#[method]
	fn _ready(&mut self, #[base] owner: &Node) {
		self.grab_nodes_by_path(owner);
		let owner_ref = unsafe { owner.assume_shared() };

		self.mapped_markers = [
			(WorldLocation::Chapel, self.marker_chapel.unwrap()),
			(WorldLocation::Grove, self.marker_grove.unwrap()),
			(WorldLocation::Forest, self.marker_forest.unwrap()),
			(WorldLocation::Cave, self.marker_cave.unwrap()),
		].into_iter().collect();
		
		self.mapped_markers.iter()
		    .for_each(|(location, button_ref)|
			    button_ref.touch_assert_sane(|button| {
				    button.connect("input_event", owner_ref, fn_name(&Self::_input_event_marker), 
					    location.to_shared_array(), Object::CONNECT_DEFERRED)
					    .log_if_err();
				    button.connect("mouse_entered", owner_ref, fn_name(&Self::_mouse_entered_marker), 
					    location.to_shared_array(), Object::CONNECT_DEFERRED)
					    .log_if_err();
				    button.connect("mouse_exited", owner_ref, fn_name(&Self::_mouse_exited_marker), 
					    location.to_shared_array(), Object::CONNECT_DEFERRED)
					    .log_if_err();
			    })
		    );
		
		self.mapped_lights = [
			(WorldLocation::Chapel, self.light_chapel.unwrap()),
			(WorldLocation::Grove, self.light_grove.unwrap()),
			(WorldLocation::Forest, self.light_forest.unwrap()),
			(WorldLocation::Cave, self.light_cave.unwrap()),
		].into_iter().collect();
		
		self.mapped_lines = [
			(WorldPath::new(WorldLocation::Chapel, WorldLocation::Grove).unwrap(), self.line_chapel_grove.unwrap()),
			(WorldPath::new(WorldLocation::Grove, WorldLocation::Forest).unwrap(), self.line_grove_forest.unwrap()),
			(WorldPath::new(WorldLocation::Forest, WorldLocation::Cave).unwrap(), self.line_forest_cave.unwrap()),
			(WorldPath::new(WorldLocation::Cave, WorldLocation::Chapel).unwrap(), self.line_cave_chapel.unwrap())
		].into_iter().collect();
		
		self.mapped_lines.iter()
			.for_each(|(path, line_ref)| 
				line_ref.touch_assert_sane(|line| {
					line.connect("input_event", owner_ref, fn_name(&Self::_input_event_line), 
						path.to_shared_array(), Object::CONNECT_DEFERRED)
						.log_if_err();
					line.connect("mouse_entered", owner_ref, fn_name(&Self::_mouse_entered_line), 
						path.to_shared_array(), Object::CONNECT_DEFERRED)
						.log_if_err();
					line.connect("mouse_exited", owner_ref, fn_name(&Self::_mouse_exited_line),
						path.to_shared_array(), Object::CONNECT_DEFERRED)
						.log_if_err();
				})
			);
		
		self.button_settings_menu.unwrap_manual()
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_settings_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		
		self.button_character_menu.unwrap_manual()
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_character_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
	
	#[method]
	fn initialize(&mut self, player_location: WorldLocation, unlocked_paths: HashSet<WorldPath>) {
		self.player_location = Some(player_location);
		let player_position = self.mapped_markers[&player_location]
			.unwrap_manual()
			.global_position();
		self.player_icon
			.unwrap_manual()
			.set_global_position(player_position);
		
		let unlocked_locations = 
			unlocked_paths.iter()
			.flat_map(|path| [path.point_a(), path.point_b()])
			.collect::<HashSet<WorldLocation>>();
		
		self.mapped_markers
			.iter()
			.for_each(|(button_location, button_ref)| {
				let Some(button) = (unsafe { button_ref.assume_safe_if_sane() })
					else {
						godot_error!("{}(): button_ref is not sane.", full_fn_name(&Self::initialize));
						return;
					};
				
				let is_visible = unlocked_locations.contains(button_location);
				button.set_visible(is_visible);
				self.mapped_lights.get(button_location)
					.unwrap_manual()
					.set_visible(is_visible);
				
				let path_to_player = WorldPath::new(player_location, *button_location);
				let is_available = path_to_player.is_some_and(|path| 
					unlocked_paths.contains(&path));
				button.set_pickable(is_available);
				
				let color =
					if is_available {
						Color::from_rgb(1.0, 1.0, 1.0)
					} else {
						Color::from_rgba(1.0, 1.0, 1.0, 0.5)
					};
				button.set_modulate(color);
			});
		
		self.mapped_lines
			.iter()
			.for_each(|(path, line_ref)| {
				let Some(line) = (unsafe { line_ref.assume_safe_if_sane() })
					else {
						godot_error!("{}(): line_ref is not sane.", full_fn_name(&Self::initialize));
						return;
					};
				
				let is_visible = unlocked_paths.contains(path);
				line.set_visible(is_visible);

				let is_available = path.contains(player_location);
				line.set_pickable(is_available);
				
				let color =
					if is_available {
						Color::from_rgb(1.0, 1.0, 1.0)
					} else {
						Color::from_rgba(1.0, 1.0, 1.0, 0.5)
					};
				line.set_modulate(color);
			});
	}
	
	#[method]
	fn _input_event_marker(&self, #[base] owner: &Node, _viewport: Ref<Node>, input_event: Ref<InputEvent>, _shape_idx: i64, location: WorldLocation) {
		if misc::is_confirm_input(input_event) {
			owner.emit_signal(SIGNAL_MARKER_CLICKED, &[location.to_variant()]);
		}
	}
	
	#[method]
	fn _input_event_line(&self, #[base] owner: &Node, _viewport: Ref<Node>, input_event: Ref<InputEvent>, _shape_idx: i64, path: WorldPath) {
		if misc::is_confirm_input(input_event) {
			owner.emit_signal(SIGNAL_LINE_CLICKED, &[path.to_variant()]);
		}
	}
	
	fn active_line_adjacent_to_player(&self, marker_location: WorldLocation) -> Option<&Ref<Area2D>> {
		let Some(player_location) = self.player_location
			else {
				godot_error!("{}(): player_location is None.", full_fn_name(&Self::_mouse_entered_marker));
				return None;
			};

		if player_location == marker_location {
			return None;
		}
		
		return WorldPath::new(player_location, marker_location)
			.and_then(|path| 
				self.mapped_lines.get(&path));
	}
	
	#[method]
	fn _mouse_entered_marker(&self, location: WorldLocation) {
		self.mapped_markers[&location]
			.unwrap_manual()
			.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
		
		if let Some(line_ref) = self.active_line_adjacent_to_player(location) {
			line_ref.touch_assert_sane(|line| 
				line.set_modulate(Color::from_rgb(0.8, 0.3, 0.8)));
		}
	}

	#[method]
	fn _mouse_exited_marker(&self, location: WorldLocation) {
		self.mapped_markers[&location]
			.unwrap_manual()
			.set_modulate(Color::from_rgb(1., 1., 1.));

		if let Some(line_ref) = self.active_line_adjacent_to_player(location) {
			line_ref.touch_assert_sane(|line|
				line.set_modulate(Color::from_rgb(1., 1., 1.)));
		}
	}
	
	fn active_marker_adjacent_to_player(&self, line_path: WorldPath) -> Option<&Ref<Area2D>> {
		let Some(player_location) = self.player_location
			else {
				godot_error!("{}(): player_location is None.", full_fn_name(&Self::_mouse_entered_line));
				return None;
			};
		
		if !line_path.contains(player_location) {
			return None;
		}
		
		let Some(line) = self.mapped_lines
			.get(&line_path)
			.and_then(|line_ref| unsafe { line_ref.assume_safe_if_sane() })
			else {
				godot_error!("{}(): line_path is not mapped.", full_fn_name(&Self::_mouse_entered_line));
				return None;
			};
		
		return if line.is_pickable() {
			let not_player_location = 
				if line_path.point_a() == player_location {
					line_path.point_b()
				} else {
					line_path.point_a()
				};
			
			self.mapped_markers.get(&not_player_location)
		} else {
			None
		};
	}
	
	#[method]
	fn _mouse_entered_line(&self, path: WorldPath) {
		self.mapped_lines[&path]
			.unwrap_manual()
			.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
		
		if let Some(marker_ref) = self.active_marker_adjacent_to_player(path) {
			marker_ref.touch_assert_sane(|marker| 
				marker.set_modulate(Color::from_rgb(0.8, 0.3, 0.8)));
		}
	}
	
	#[method]
	fn _mouse_exited_line(&self, path: WorldPath) {
		self.mapped_lines[&path]
			.unwrap_manual()
			.set_modulate(Color::from_rgb(1., 1., 1.));
		
		if let Some(marker_ref) = self.active_marker_adjacent_to_player(path) {
			marker_ref.touch_assert_sane(|marker| 
				marker.set_modulate(Color::from_rgb(1., 1., 1.)));
		}
	}
	
	#[method]
	fn _button_pressed_settings_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_SETTINGS_MENU, &[]);
	}
	
	#[method]
	fn _button_pressed_character_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_CHARACTER_MENU, &[]);
	}
}