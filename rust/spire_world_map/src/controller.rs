#![allow(unused)]

use super::*;

lazy_stringname! { pub SIGNAL_OPEN_SETTINGS_MENU = "open_settings_menu" }
lazy_stringname! { pub SIGNAL_OPEN_CHARACTER_MENU = "open_character_menu" }
lazy_stringname! { pub SIGNAL_MARKER_CLICKED = "marker_clicked" }
lazy_stringname! { pub SIGNAL_LINE_CLICKED = "line_clicked" }

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct WorldMapController {
	base: Base<Node2D>,
	#[init(node = "")]
	player_icon: OnReady<Gd<Node2D>>,
	player_location: Option<WorldLocation>,

	#[init(node = "")]
	marker_chapel:  OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_grove:   OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_cave:    OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_forest:  OnReady<Gd<Area2D>>,
	mapped_markers: HashMap<WorldLocation, Gd<Area2D>>,

	#[init(node = "")]
	light_chapel:  OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_grove:   OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_cave:    OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_forest:  OnReady<Gd<Light2D>>,
	mapped_lights: HashMap<WorldLocation, Gd<Light2D>>,

	#[init(node = "")]
	line_chapel_grove: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	line_grove_forest: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	line_forest_cave: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	line_cave_chapel: OnReady<Gd<Area2D>>,
	mapped_lines: HashMap<WorldPath, Gd<Area2D>>,

	#[init(node = "")]
	button_settings_menu:  OnReady<Gd<Button>>,
	#[init(node = "")]
	button_character_menu: OnReady<Gd<Button>>,
}

/*
#[godot_api]
impl INode2D for WorldMapController {
	fn ready(&mut self) {
		todo!()

		self.mapped_markers = HashMap::from([
			(WorldLocation::Chapel, self.marker_chapel.clone()),
			(WorldLocation::Grove, self.marker_grove.clone()),
			(WorldLocation::Forest, self.marker_forest.clone()),
			(WorldLocation::Cave, self.marker_cave.clone()),
		]);

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
			(crate::internal_prelude::WorldPath(WorldLocation::Chapel, WorldLocation::Grove).unwrap(), self.line_chapel_grove.unwrap()),
			(crate::internal_prelude::WorldPath(WorldLocation::Grove, WorldLocation::Forest).unwrap(), self.line_grove_forest.unwrap()),
			(crate::internal_prelude::WorldPath(WorldLocation::Forest, WorldLocation::Cave).unwrap(), self.line_forest_cave.unwrap()),
			(crate::internal_prelude::WorldPath(WorldLocation::Cave, WorldLocation::Chapel).unwrap(), self.line_cave_chapel.unwrap())
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

		self.button_settings_menu
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_settings_menu),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();

		self.button_character_menu
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_character_menu),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
}


#[godot_api]
impl WorldMapController {
	#[signal] fn open_settings_menu() {}
	#[signal] fn open_character_menu() {}
	#[signal] fn marker_clicked(location: WorldLocation) {}
	#[signal] fn line_clicked(path: WorldPath) {}

	#[func]
	fn initialize(&mut self, player_location: WorldLocation, unlocked_paths: HashSet<WorldPath>) {
		todo!()
		/*
		self.player_location = Some(player_location);
		let player_position = self.mapped_markers[&player_location]

			.global_position();
		self.player_icon

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
		*/
	}

	#[func]
	fn _input_event_marker(
		&self,
		_viewport: Gd<Node>,
		input_event: Gd<InputEvent>,
		_shape_idx: i64,
		location: WorldLocation
	) {
		todo!()
		/*
		if shared::input::is_confirm_input(unsafe { &input_event.assume_safe() }) {
			owner.emit_signal(SIGNAL_MARKER_CLICKED, &[location.to_variant()]);
		}
		*/
	}

	#[func]
	fn _input_event_line(
		&self,
		_viewport: Gd<Node>,
		input_event: Gd<InputEvent>,
		_shape_idx: i64,
		path: WorldPath
	) {
		todo!()
		/*
		if shared::input::is_confirm_input(unsafe { &input_event.assume_safe() }) {
			owner.emit_signal(SIGNAL_LINE_CLICKED, &[path.to_variant()]);
		}
		*/
	}

	fn active_line_adjacent_to_player(&self, marker_location: WorldLocation) -> Option<&Gd<Area2D>> {
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

	#[func]
	fn _mouse_entered_marker(&self, location: WorldLocation) {
		self.mapped_markers[&location]

			.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));

		if let Some(line_ref) = self.active_line_adjacent_to_player(location) {
			line_ref.touch_assert_sane(|line|
				line.set_modulate(Color::from_rgb(0.8, 0.3, 0.8)));
		}
	}

	#[func]
	fn _mouse_exited_marker(&self, location: WorldLocation) {
		self.mapped_markers[&location]

			.set_modulate(Color::from_rgb(1., 1., 1.));

		if let Some(line_ref) = self.active_line_adjacent_to_player(location) {
			line_ref.touch_assert_sane(|line|
				line.set_modulate(Color::from_rgb(1., 1., 1.)));
		}
	}

	fn active_marker_adjacent_to_player(&self, line_path: WorldPath) -> Option<&Gd<Area2D>> {
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

	#[func]
	fn _mouse_entered_line(&self, path: WorldPath) {
		self.mapped_lines[&path]

			.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));

		if let Some(marker_ref) = self.active_marker_adjacent_to_player(path) {
			marker_ref.touch_assert_sane(|marker|
				marker.set_modulate(Color::from_rgb(0.8, 0.3, 0.8)));
		}
	}

	#[func]
	fn _mouse_exited_line(&self, path: WorldPath) {
		self.mapped_lines[&path]

			.set_modulate(Color::from_rgb(1., 1., 1.));

		if let Some(marker_ref) = self.active_marker_adjacent_to_player(path) {
			marker_ref.touch_assert_sane(|marker|
				marker.set_modulate(Color::from_rgb(1., 1., 1.)));
		}
	}

	#[func]
	fn _button_pressed_settings_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_SETTINGS_MENU, &[]);
	}

	#[func]
	fn _button_pressed_character_menu(&self, #[base] owner: &Node) {
		owner.emit_signal(SIGNAL_OPEN_CHARACTER_MENU, &[]);
	}
}
*/
