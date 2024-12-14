use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use godot::global::{Key, MouseButton};
use noise_grid::GridShape;
use rand_xoshiro::{
	Xoshiro256PlusPlus,
	rand_core::{RngCore, SeedableRng},
};

use super::*;

#[derive(Debug, Default, GodotClass)]
#[class(init, base = Resource)]
pub struct BiomeDataResource {
	#[export]
	weight: f32,
	#[export]
	altitude_threshold: f32,
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct MapGeneratorUI {
	base: Base<Control>,

	spawned_hexagons: HashMap<Axial, Gd<Sprite2D>>,
	current_map: Option<HexagonMap>,

	#[export]
	hexagon_prefab: RequiredGd<PackedScene>,
	#[export]
	hexagon_parent_prefab: RequiredGd<PackedScene>,
	#[export]
	hexagon_parent: RequiredGd<Node>,
	#[export]
	button_generate_full: RequiredGd<Button>,
	#[export]
	button_generate_base: RequiredGd<Button>,
	#[export]
	button_ensure_connecteds: RequiredGd<Button>,
	#[export]
	camera: RequiredGd<Camera2D>,

	#[export]
	hexagon_radius: f64,
	#[export]
	map_width: i32,
	#[export]
	map_height: i32,
	#[export]
	map_shape: GridShape,
	#[export]
	end_direction: HexagonDirection,
	#[export]
	hexagon_radius_spin_box: RequiredGd<SpinBox>,
	#[export]
	map_width_spin_box: RequiredGd<SpinBox>,
	#[export]
	map_height_spin_box: RequiredGd<SpinBox>,
	#[export]
	map_shape_option_button: RequiredGd<OptionButton>,
	#[export]
	end_direction_option_button: RequiredGd<OptionButton>,

	#[export]
	altitude_octaves: i32,
	#[export]
	altitude_lacunarity: f32,
	#[export]
	altitude_frequency: f32,
	#[export]
	altitude_spin_box_octaves: RequiredGd<SpinBox>,
	#[export]
	altitude_spin_box_lacunarity: RequiredGd<SpinBox>,
	#[export]
	altitude_spin_box_frequency: RequiredGd<SpinBox>,

	#[export]
	biome_octaves: i32,
	#[export]
	biome_lacunarity: f32,
	#[export]
	biome_frequency: f32,
	#[export]
	biome_spin_box_octaves: RequiredGd<SpinBox>,
	#[export]
	biome_spin_box_lacunarity: RequiredGd<SpinBox>,
	#[export]
	biome_spin_box_frequency: RequiredGd<SpinBox>,

	#[export]
	biome_1: RequiredGd<BiomeDataResource>,
	#[export]
	biome_2: RequiredGd<BiomeDataResource>,
	#[export]
	biome_3: RequiredGd<BiomeDataResource>,
	#[export]
	biome_4: RequiredGd<BiomeDataResource>,

	#[export]
	biome_1_weight: f32,
	#[export]
	biome_1_altitude_threshold: f32,
	#[export]
	biome_2_weight: f32,
	#[export]
	biome_2_altitude_threshold: f32,
	#[export]
	biome_3_weight: f32,
	#[export]
	biome_3_altitude_threshold: f32,
	#[export]
	biome_4_weight: f32,
	#[export]
	biome_4_altitude_threshold: f32,
	#[export]
	biome_1_weight_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_1_altitude_threshold_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_2_weight_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_2_altitude_threshold_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_3_weight_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_3_altitude_threshold_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_4_weight_spin_box: RequiredGd<SpinBox>,
	#[export]
	biome_4_altitude_threshold_spin_box: RequiredGd<SpinBox>,
}

#[godot_api]
impl IControl for MapGeneratorUI {
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if !event.is_pressed() {
			return;
		}

		let mouse_button = event.try_cast::<InputEventMouseButton>().ok()?;

		const ZOOM_STEP: f32 = 1.06;

		let zoom_step = if mouse_button.get_button_index() == MouseButton::WHEEL_UP {
			1. / ZOOM_STEP
		} else if mouse_button.get_button_index() == MouseButton::WHEEL_DOWN {
			ZOOM_STEP
		} else {
			return;
		};

		let cam = &mut *self.camera;

		let old_cam_offset = cam.get_offset();
		let scale = {
			let vec_int = cam.get_window().unwrap().get_content_scale_size();
			Vector2 {
				x: vec_int.x as real,
				y: vec_int.y as real,
			}
		};

		let old_zoom = cam.get_zoom();
		let new_zoom = old_zoom * zoom_step;
		let new_cam_offset =
			old_cam_offset + (scale * -0.5 + mouse_button.get_position()) * (old_zoom - new_zoom);

		cam.set_zoom(new_zoom);
		cam.set_offset(new_cam_offset);
	}

	fn process(&mut self, _delta: f64) {
		let input = Input::singleton();

		if input.is_key_pressed(Key::A) {
			move_camera(&mut self.camera, Vector2::new(-10., 0.));
		}

		if input.is_key_pressed(Key::D) {
			move_camera(&mut self.camera, Vector2::new(10., 0.));
		}

		if input.is_key_pressed(Key::W) {
			move_camera(&mut self.camera, Vector2::new(0., -10.));
		}

		if input.is_key_pressed(Key::S) {
			move_camera(&mut self.camera, Vector2::new(0., 10.));
		}

		if input.is_key_pressed(Key::R) {
			self.camera.set_position(Vector2::ZERO);
			self.camera.set_zoom(Vector2::new(1., 1.));
			self.camera.set_offset(Vector2::ZERO);
		}

		fn move_camera(camera: &mut Camera2D, delta: Vector2) {
			let new_pos = camera.get_position() + delta;
			camera.set_position(new_pos);
		}
	}

	fn ready(&mut self) {
		self.connect_child("button_generate_full", "pressed", |this, _| {
			this.delete_old();

			let (altitude_noise, biome_noise) = this.noise_map();
			let biomes = this.biomes();
			let (base_map, start, end, _) = generate_full(
				this.map_width,
				this.map_height,
				this.map_shape,
				&biomes,
				&this.end_direction,
				altitude_noise,
				biome_noise,
			);

			this.current_map = Some(base_map);
			this.set_hexagon_colors(start, end);
		})
		.log_if_err();

		self.connect_child("button_generate_base", "pressed", |this, _| {
			this.delete_old();

			let biomes = [
				BiomeData {
					weight: this.biome_1_weight,
					altitude_threshold: this.biome_1_altitude_threshold,
				},
				BiomeData {
					weight: this.biome_2_weight,
					altitude_threshold: this.biome_2_altitude_threshold,
				},
				BiomeData {
					weight: this.biome_3_weight,
					altitude_threshold: this.biome_3_altitude_threshold,
				},
				BiomeData {
					weight: this.biome_4_weight,
					altitude_threshold: this.biome_4_altitude_threshold,
				},
			];

			let (altitude_noise, biome_noise) = this.noise_map();

			let (base_map, start, end, _) = generate_base(
				this.map_width,
				this.map_height,
				this.map_shape,
				&biomes,
				&this.end_direction,
				altitude_noise,
				biome_noise,
			);

			this.current_map = Some(base_map);
			this.set_hexagon_colors(start, end);
		})
		.log_if_err();

		self.connect_child("button_ensure_connecteds", "pressed", |this, _| {
			if let Some(map) = &mut this.current_map {
				let start_pos = map.start_pos;
				let end_pos = map.end_pos;
				pathing::ensure_open_areas_are_connected_to_start(map, start_pos, end_pos);
				this.set_hexagon_colors(start_pos, end_pos);
			}
		})
		.log_if_err();

		let self_gd = self.to_gd();

		self.hexagon_radius_spin_box.set_value(self.hexagon_radius);
		self.hexagon_radius_spin_box
			.connect("value_changed", &self_gd.callable("set_hexagon_radius"));

		self.map_width_spin_box.set_value(self.map_width as f64);
		self.map_width_spin_box
			.connect("value_changed", &self_gd.callable("set_map_width"));

		self.map_height_spin_box.set_value(self.map_height as f64);
		self.map_height_spin_box
			.connect("value_changed", &self_gd.callable("set_map_height"));

		add_items(
			&mut self.map_shape_option_button,
			&[
				("Hexagon", 0),
				("Rectangle", 1),
				("Parallelogram", 2),
				("Triangle", 3),
			],
		);

		self.map_shape_option_button.select(self.map_shape as i32);
		self.map_shape_option_button
			.connect_deferred("item_selected", {
				let mut this = self_gd.clone();
				move |args| {
					let value = args.first()?.try_to::<i32>().ok()?;
					GridShape::try_from(value as u8)
						.map(|shape| this.bind_mut().map_shape = shape)
						.log_if_err();
				}
			});

		add_items(
			&mut self.end_direction_option_button,
			&[
				("SouthEast", 0),
				("East", 1),
				("NorthEast", 2),
				("NorthWest", 3),
				("West", 4),
				("SouthWest", 5),
			],
		);

		self.end_direction_option_button
			.select(self.end_direction as i32);
		self.end_direction_option_button
			.connect_deferred("item_selected", {
				let mut this = self_gd.clone();
				move |args| {
					let value = args.first()?.try_to::<i32>().ok()?;
					HexagonDirection::try_from(value as u8)
						.map(|direction| this.bind_mut().end_direction = direction)
						.log_if_err();
				}
			});

		fn add_items(button: &mut OptionButton, items: &[(&str, i32)]) {
			for (label, id) in items {
				button.add_item_ex(*label).id(*id).done();
			}
		}

		self.altitude_spin_box_octaves
			.set_value(self.altitude_octaves as f64);
		self.altitude_spin_box_octaves
			.connect("value_changed", &self_gd.callable("set_altitude_octaves"));

		self.altitude_spin_box_lacunarity
			.set_value(self.altitude_lacunarity as f64);
		self.altitude_spin_box_lacunarity
			.connect("value_changed", &self_gd.callable("set_altitude_lacunarity"));

		self.altitude_spin_box_frequency
			.set_value(self.altitude_frequency as f64);
		self.altitude_spin_box_frequency
			.connect("value_changed", &self_gd.callable("set_altitude_frequency"));

		self.biome_spin_box_octaves
			.set_value(self.biome_octaves as f64);
		self.biome_spin_box_octaves
			.connect("value_changed", &self_gd.callable("set_biome_octaves"));

		self.biome_spin_box_lacunarity
			.set_value(self.biome_lacunarity as f64);
		self.biome_spin_box_lacunarity
			.connect("value_changed", &self_gd.callable("set_biome_lacunarity"));

		self.biome_spin_box_frequency
			.set_value(self.biome_frequency as f64);
		self.biome_spin_box_frequency
			.connect("value_changed", &self_gd.callable("set_biome_frequency"));

		{
			let bind = self.biome_1.bind();
			self.biome_1_weight = bind.weight;
			self.biome_1_altitude_threshold = bind.altitude_threshold;

			let bind = self.biome_2.bind();
			self.biome_2_weight = bind.weight;
			self.biome_2_altitude_threshold = bind.altitude_threshold;

			let bind = self.biome_3.bind();
			self.biome_3_weight = bind.weight;
			self.biome_3_altitude_threshold = bind.altitude_threshold;

			let bind = self.biome_4.bind();
			self.biome_4_weight = bind.weight;
			self.biome_4_altitude_threshold = bind.altitude_threshold;
		}

		self.biome_1_weight_spin_box
			.set_value(self.biome_1_weight as f64);
		self.biome_1_weight_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_1_weight"));

		self.biome_1_altitude_threshold_spin_box
			.set_value(self.biome_1_altitude_threshold as f64);
		self.biome_1_altitude_threshold_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_1_altitude_threshold"));

		self.biome_2_weight_spin_box
			.set_value(self.biome_2_weight as f64);
		self.biome_2_weight_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_2_weight"));

		self.biome_2_altitude_threshold_spin_box
			.set_value(self.biome_2_altitude_threshold as f64);
		self.biome_2_altitude_threshold_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_2_altitude_threshold"));

		self.biome_3_weight_spin_box
			.set_value(self.biome_3_weight as f64);
		self.biome_3_weight_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_3_weight"));

		self.biome_3_altitude_threshold_spin_box
			.set_value(self.biome_3_altitude_threshold as f64);
		self.biome_3_altitude_threshold_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_3_altitude_threshold"));

		self.biome_4_weight_spin_box
			.set_value(self.biome_4_weight as f64);
		self.biome_4_weight_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_4_weight"));

		self.biome_4_altitude_threshold_spin_box
			.set_value(self.biome_4_altitude_threshold as f64);
		self.biome_4_altitude_threshold_spin_box
			.connect("value_changed", &self_gd.callable("set_biome_4_altitude_threshold"));
	}
}

impl MapGeneratorUI {
	fn delete_old(&mut self) -> Gd<Node> {
		let mut old_parent = (*self.hexagon_parent).clone();
		self.base_mut().remove_child(&old_parent);
		old_parent.queue_free();

		let hex_parent = self.hexagon_parent_prefab.instantiate().unwrap();
		self.base_mut().add_child(&hex_parent);
		self.base_mut().move_child(&hex_parent, 0);
		self.hexagon_parent = RequiredGd::new(hex_parent.clone());
		self.spawned_hexagons.clear();

		hex_parent
	}

	fn noise_map(&self) -> (FastNoise, FastNoise) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();

		let altitude = {
			let mut temp = FastNoise::seeded(rng.next_u64());
			temp.set_noise_type(NoiseType::PerlinFractal);
			temp.set_fractal_type(FractalType::FBM);
			temp.set_fractal_octaves(self.altitude_octaves);
			temp.set_fractal_gain(0.5);
			temp.set_fractal_lacunarity(self.altitude_lacunarity);
			temp.set_frequency(self.altitude_frequency);
			temp
		};

		let biome = {
			let mut temp = FastNoise::seeded(rng.next_u64());
			temp.set_noise_type(NoiseType::PerlinFractal);
			temp.set_fractal_type(FractalType::FBM);
			temp.set_fractal_octaves(self.biome_octaves);
			temp.set_fractal_gain(0.5);
			temp.set_fractal_lacunarity(self.biome_lacunarity);
			temp.set_frequency(self.biome_frequency);
			temp
		};

		(altitude, biome)
	}

	fn set_hexagon_colors(&mut self, start: Axial, end: Axial) {
		let Some(full_map) = &self.current_map
		else {
			godot_error!("{}(): current_map is None", full_fn_name(&Self::set_hexagon_colors));
			return;
		};

		let available_colors = get_available_colors();

		let hexagon_radius = self.hexagon_radius as f32;
		let black = Color::from_rgb(0., 0., 0.);
		for (pos, tile) in full_map.tiles.iter() {
			let mut hexagon = self.hexagon_prefab.instantiate_as::<Sprite2D>();

			self.hexagon_parent.add_child(&hexagon);
			self.spawned_hexagons.insert(*pos, hexagon.clone());

			let (x, y) = pos.to_cartesian(hexagon_radius);
			hexagon.set_position(Vector2::new(x, y));

			let color = if tile.is_obstacle() {
				black
			} else {
				available_colors[tile.biome.id as usize]
			};

			hexagon.set_self_modulate(color);
		}

		let start_color = Color::from_rgb(1., 0., 1.);
		if let Some(node) = self.spawned_hexagons.get_mut(&start) {
			node.set_self_modulate(start_color);
		}

		let end_color = Color::from_rgb(1., 0., 1.);
		if let Some(node) = self.spawned_hexagons.get_mut(&end) {
			node.set_self_modulate(end_color)
		}
	}

	fn biomes(&self) -> [BiomeData; 4] {
		[
			BiomeData {
				weight: self.biome_1_weight,
				altitude_threshold: self.biome_1_altitude_threshold,
			},
			BiomeData {
				weight: self.biome_2_weight,
				altitude_threshold: self.biome_2_altitude_threshold,
			},
			BiomeData {
				weight: self.biome_3_weight,
				altitude_threshold: self.biome_3_altitude_threshold,
			},
			BiomeData {
				weight: self.biome_4_weight,
				altitude_threshold: self.biome_4_altitude_threshold,
			},
		]
	}
}

fn get_available_colors() -> [Color; 5] {
	[
		Color::from_rgb(1., 0., 0.),
		Color::from_rgb(0., 1., 0.),
		Color::from_rgb(0., 0., 1.),
		Color::from_rgb(1., 1., 0.),
		Color::from_rgb(0., 1., 1.),
	]
}
