//TODO: node paths
use super::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct WorldMapController {
	base: Base<Node2D>,
	#[init(node = "")]
	player_icon: OnReady<Gd<Node2D>>,
	player_location: Option<WorldLocation>,

	#[init(node = "")]
	marker_chapel: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_grove: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_cave: OnReady<Gd<Area2D>>,
	#[init(node = "")]
	marker_forest: OnReady<Gd<Area2D>>,
	mapped_markers: HashMap<WorldLocation, Gd<Area2D>>,

	#[init(node = "")]
	light_chapel: OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_grove: OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_cave: OnReady<Gd<Light2D>>,
	#[init(node = "")]
	light_forest: OnReady<Gd<Light2D>>,
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

#[godot_api]
impl INode2D for WorldMapController {
	fn ready(&mut self) {
		self.mapped_markers = HashMap::from([
			(WorldLocation::Chapel, self.marker_chapel.clone()),
			(WorldLocation::Grove, self.marker_grove.clone()),
			(WorldLocation::Forest, self.marker_forest.clone()),
			(WorldLocation::Cave, self.marker_cave.clone()),
		]);

		for (location, bttn) in self.mapped_markers.clone() {
			self.connect_with_deferred(&bttn, "input_event", move |this, args| {
				let input_event = args.try_var_at::<Gd<InputEvent>>(0).ok_log_err()?;
				if is_confirm_input(&input_event) {
					this.base_mut()
						.emit_signal(Self::SIGNAL_MARKER_CLICKED, &[location.to_variant()]);
				}
			});

			self.connect_with_deferred(&bttn, "mouse_entered", move |this, _| {
				if let Some(marker) = this.mapped_markers.get_mut(&location) {
					marker.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
				}

				if let Some(line) = this.active_line_adjacent_to_player(location) {
					line.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
				}
			});

			self.connect_with_deferred(&bttn, "mouse_exited", move |this, _| {
				if let Some(marker) = this.mapped_markers.get_mut(&location) {
					marker.set_modulate(Color::from_rgb(1., 1., 1.));
				}

				if let Some(line) = this.active_line_adjacent_to_player(location) {
					line.set_modulate(Color::from_rgb(1., 1., 1.));
				}
			});
		}

		self.mapped_lights = HashMap::from([
			(WorldLocation::Chapel, (*self.light_chapel).clone()),
			(WorldLocation::Grove, (*self.light_grove).clone()),
			(WorldLocation::Forest, (*self.light_forest).clone()),
			(WorldLocation::Cave, (*self.light_cave).clone()),
		]);

		self.mapped_lines = HashMap::from([
			(
				const { WorldPath::new(WorldLocation::Chapel, WorldLocation::Grove).unwrap() },
				self.line_chapel_grove.clone(),
			),
			(
				const { WorldPath::new(WorldLocation::Grove, WorldLocation::Forest).unwrap() },
				self.line_grove_forest.clone(),
			),
			(
				const { WorldPath::new(WorldLocation::Forest, WorldLocation::Cave).unwrap() },
				self.line_forest_cave.clone(),
			),
			(
				const { WorldPath::new(WorldLocation::Cave, WorldLocation::Chapel).unwrap() },
				self.line_cave_chapel.clone(),
			),
		]);

		for (path, line) in self.mapped_lines.clone() {
			self.connect_with_deferred(&line, "input_event", move |this, args| {
				let input_event = args.try_var_at::<Gd<InputEvent>>(0).ok_log_err()?;

				if is_confirm_input(&input_event) {
					this.base_mut()
						.emit_signal(Self::SIGNAL_LINE_CLICKED, &[path.to_variant()]);
				}
			});

			self.connect_with_deferred(&line, "mouse_entered", move |this, _| {
				if let Some(line) = this.mapped_lines.get_mut(&path) {
					line.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
				}

				if let Some(marker) = this.active_marker_adjacent_to_player(path) {
					marker.set_modulate(Color::from_rgb(0.8, 0.3, 0.8));
				}
			});

			self.connect_with_deferred(&line, "mouse_exited", move |this, _| {
				if let Some(line) = this.mapped_lines.get_mut(&path) {
					line.set_modulate(Color::from_rgb(1., 1., 1.));
				}

				if let Some(marker) = this.active_marker_adjacent_to_player(path) {
					marker.set_modulate(Color::from_rgb(1., 1., 1.));
				}
			});
		}

		self.connect_with_deferred(&self.button_settings_menu.clone(), "pressed", |this, _| {
			this.base_mut()
				.emit_signal(Self::SIGNAL_OPEN_SETTINGS_MENU, &[]);
		});

		self.connect_with_deferred(&self.button_character_menu.clone(), "pressed", |this, _| {
			this.base_mut()
				.emit_signal(Self::SIGNAL_OPEN_CHARACTER_MENU, &[]);
		});
	}
}

impl WorldMapController {
	pub const SIGNAL_OPEN_SETTINGS_MENU: &'static str = "open_settings_menu";
	pub const SIGNAL_OPEN_CHARACTER_MENU: &'static str = "open_character_menu";
	pub const SIGNAL_MARKER_CLICKED: &'static str = "marker_clicked";
	pub const SIGNAL_LINE_CLICKED: &'static str = "line_clicked";
}

#[godot_api]
impl WorldMapController {
	#[signal]
	fn open_settings_menu() {}
	#[signal]
	fn open_character_menu() {}
	#[signal]
	fn marker_clicked(location: WorldLocation) {}
	#[signal]
	fn line_clicked(path: WorldPath) {}

	pub fn initialize(
		&mut self,
		player_location: WorldLocation,
		unlocked_paths: HashSet<WorldPath>,
	) {
		self.player_location = Some(player_location);
		let player_position = self.mapped_markers[&player_location].get_global_position();
		self.player_icon.set_global_position(player_position);

		let unlocked_locations = unlocked_paths
			.iter()
			.flat_map(|path| [path.point_a(), path.point_b()])
			.collect::<HashSet<WorldLocation>>();

		for (location, button) in &mut self.mapped_markers {
			let is_visible = unlocked_locations.contains(location);
			button.set_visible(is_visible);
			if let Some(light) = self.mapped_lights.get_mut(location) {
				light.set_visible(is_visible)
			}

			let path_to_player = WorldPath::new(player_location, *location);
			let is_available = path_to_player.is_some_and(|path| unlocked_paths.contains(&path));
			button.set_pickable(is_available);
			button.set_modulate(if is_available {
				Color::from_rgb(1.0, 1.0, 1.0)
			} else {
				Color::from_rgba(1.0, 1.0, 1.0, 0.5)
			});
		}

		for (path, line) in &mut self.mapped_lines {
			line.set_visible(unlocked_paths.contains(path));

			let is_available = path.contains(player_location);
			line.set_pickable(is_available);
			line.set_modulate(if is_available {
				Color::from_rgb(1.0, 1.0, 1.0)
			} else {
				Color::from_rgba(1.0, 1.0, 1.0, 0.5)
			});
		}
	}

	fn active_line_adjacent_to_player(
		&mut self,
		marker_location: WorldLocation,
	) -> Option<&mut Gd<Area2D>> {
		let player_location = self.player_location?;

		if player_location == marker_location {
			return None;
		}

		WorldPath::new(player_location, marker_location)
			.and_then(|path| self.mapped_lines.get_mut(&path))
	}

	fn active_marker_adjacent_to_player(
		&mut self,
		line_path: WorldPath,
	) -> Option<&mut Gd<Area2D>> {
		let player_location = self.player_location?;

		if !line_path.contains(player_location) {
			return None;
		}

		let line = self.mapped_lines.get(&line_path)?;

		if line.is_pickable() {
			let not_player_location = if line_path.point_a() == player_location {
				line_path.point_b()
			} else {
				line_path.point_a()
			};

			self.mapped_markers.get_mut(&not_player_location)
		} else {
			None
		}
	}
}
