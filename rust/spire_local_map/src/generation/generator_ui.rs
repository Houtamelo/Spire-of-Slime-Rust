/*
todo!
use crate::internal_prelude::*;

use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use rand::Rng;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use noise_grid::GridShape;

use crate::coordinates::axial::Axial;
use crate::coordinates::direction::HexagonDirection;
use crate::generation::{BiomeData, noise_grid};
use crate::generation::map::HexagonMap;
use crate::generation::pathing;

#[derive(Debug, Default, ToVariant, FromVariant)]
#[derive(GodotClass)]
#[class(no_init, base = Resource)]
pub struct BiomeDataResource {
	#[property] weight: f32,
	#[property] altitude_threshold: f32,
}

#[godot_api]
impl BiomeDataResource {
	fn new(_owner: &Resource) -> Self {
		Self::default()
	}

	#[allow(unused)]
	fn to_rust(&self) -> BiomeData {
		return BiomeData {
			weight: self.weight,
			altitude_threshold: self.altitude_threshold,
		};
	}

	#[allow(unused)]
	pub(self) fn instance_to_rust<'a>(resource: &'a Gd<'a, BiomeDataResource>) -> BiomeData {
		return resource.map(|r, _| r.to_rust()).unwrap();
	}
}

#[derive(GodotClass)]
#[class(no_init, base = Node)]
pub struct MapGeneratorUI {
	spawned_hexagons: HashMap<Axial, Gd<Sprite>>,
	current_map: Option<HexagonMap>,

	#[property] hexagon_prefab: Option<Gd<PackedScene>>,
	#[property] hexagon_parent_prefab: Option<Gd<PackedScene>>,
	#[export_path] hexagon_parent: Option<Gd<Node>>,
	#[export_path] button_generate_full: Option<Gd<Button>>,
	#[export_path] button_generate_base: Option<Gd<Button>>,
	#[export_path] button_ensure_connecteds: Option<Gd<Button>>,
	#[export_path] camera: Option<Gd<Camera2D>>,

	#[property] hexagon_radius: f64,
	#[property] map_width : i32,
	#[property] map_height: i32,
	#[property] map_shape: GridShape,
	#[property] end_direction: HexagonDirection,
	#[export_path] hexagon_radius_spin_box: Option<Gd<SpinBox>>,
	#[export_path] map_width_spin_box : Option<Gd<SpinBox>>,
	#[export_path] map_height_spin_box: Option<Gd<SpinBox>>,
	#[export_path] map_shape_option_button: Option<Gd<OptionButton>>,
	#[export_path] end_direction_option_button: Option<Gd<OptionButton>>,

	#[property] altitude_octaves: i32,
	#[property] altitude_lacunarity: f32,
	#[property] altitude_frequency : f32,
	#[export_path] altitude_spin_box_octaves : Option<Gd<SpinBox>>,
	#[export_path] altitude_spin_box_lacunarity: Option<Gd<SpinBox>>,
	#[export_path] altitude_spin_box_frequency: Option<Gd<SpinBox>>,

	#[property] biome_octaves: i32,
	#[property] biome_lacunarity: f32,
	#[property] biome_frequency : f32,
	#[export_path] biome_spin_box_octaves : Option<Gd<SpinBox>>,
	#[export_path] biome_spin_box_lacunarity: Option<Gd<SpinBox>>,
	#[export_path] biome_spin_box_frequency: Option<Gd<SpinBox>>,

	#[property] biome_1: Option<Gd<BiomeDataResource>>,
	#[property] biome_2: Option<Gd<BiomeDataResource>>,
	#[property] biome_3: Option<Gd<BiomeDataResource>>,
	#[property] biome_4: Option<Gd<BiomeDataResource>>,

	#[property] biome_1_weight: f32,
	#[property] biome_1_altitude_threshold: f32,
	#[property] biome_2_weight: f32,
	#[property] biome_2_altitude_threshold: f32,
	#[property] biome_3_weight: f32,
	#[property] biome_3_altitude_threshold: f32,
	#[property] biome_4_weight: f32,
	#[property] biome_4_altitude_threshold: f32,
	#[export_path] biome_1_weight_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_1_altitude_threshold_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_2_weight_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_2_altitude_threshold_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_3_weight_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_3_altitude_threshold_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_4_weight_spin_box: Option<Gd<SpinBox>>,
	#[export_path] biome_4_altitude_threshold_spin_box: Option<Gd<SpinBox>>,
}

#[godot_api]
impl MapGeneratorUI {
	#[func]
	fn _unhandled_input(&self, event: Gd<InputEvent>) {
		let event_tref = unsafe { event.assume_safe() };
		if !event_tref.is_pressed() {
			return;
		}

		if let Some(event_mouse_button) = event_tref.cast::<InputEventMouseButton>() {
			const ZOOM_STEP: f32 = 1.06;

			let zoom_step = if event_mouse_button.button_index() == GlobalConstants::BUTTON_WHEEL_UP {
				1. / ZOOM_STEP
			} else if event_mouse_button.button_index() == GlobalConstants::BUTTON_WHEEL_DOWN {
				ZOOM_STEP
			} else {
				return;
			};

			self.camera.touch_assert_sane(|cam| {
				let old_cam_offset = cam.offset();
				let view_port = cam.get_viewport().size();
				let old_zoom = cam.zoom();
				let new_zoom = old_zoom * zoom_step;
				let new_cam_offset = old_cam_offset + (view_port * -0.5 + event_mouse_button.position()) * (old_zoom - new_zoom);

				cam.set_zoom(new_zoom);
				cam.set_offset(new_cam_offset);
			});
		}
	}

	#[func]
	fn _process(&self, _delta: f64) {
		let input = Input::singleton();
		if input.is_key_pressed(GlobalConstants::KEY_A) {
			move_camera(&self.camera, Vector2::new(-10.,0.));
		}

		if input.is_key_pressed(GlobalConstants::KEY_D) {
			move_camera(&self.camera, Vector2::new(10.,0.));
		}

		if input.is_key_pressed(GlobalConstants::KEY_W) {
			move_camera(&self.camera, Vector2::new(0.,-10.));
		}

		if input.is_key_pressed(GlobalConstants::KEY_S) {
			move_camera(&self.camera, Vector2::new(0.,10.));
		}

		if input.is_key_pressed(GlobalConstants::KEY_R) {
			self.camera.touch_assert_sane(|cam| {
				cam.set_position(Vector2::ZERO);
				cam.set_zoom(Vector2::new(1., 1.));
				cam.set_offset(Vector2::ZERO);
			});
		}

		fn move_camera(camera: &Option<Gd<Camera2D>>, delta: Vector2) {
			camera.touch_assert_sane(|cam| {
				cam.set_position(cam.position() + delta);
			});
		}
	}

	#[func]
	fn ready(&mut self, #[base] owner: &Node) {
		self.grab_nodes_by_path(owner);

		self.button_generate_full
			.connect("pressed", unsafe { owner.assume_shared() }, fn_name(&Self::_button_pressed_generate_full),
					 VariantArray::new_shared(), 0)
			.log_if_err();
		self.button_generate_base
			.connect("pressed", unsafe { owner.assume_shared() }, fn_name(&Self::_button_pressed_generate_base),
					 VariantArray::new_shared(), 0)
			.log_if_err();
		self.button_ensure_connecteds
			.connect("pressed", unsafe { owner.assume_shared() }, fn_name(&Self::_button_pressed_ensure_connecteds),
					 VariantArray::new_shared(), 0)
			.log_if_err();

		self.hexagon_radius_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.hexagon_radius);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_hexagon_radius_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.map_width_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.map_width as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_map_width_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.map_height_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.map_height as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_map_height_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.map_shape_option_button.touch_assert_sane(|option_button| {
			option_button.add_item("Hexagon", 0);
			option_button.add_item("Rectangle", 1);
			option_button.add_item("Parallelogram", 2);
			option_button.add_item("Triangle", 3);

			option_button.select(self.map_shape as i64);
			option_button.connect("item_selected", unsafe { owner.assume_shared() },
								  fn_name(&Self::_map_shape_changed), VariantArray::new_shared(), 0)
						 .log_if_err();
		});
		self.end_direction_option_button.touch_assert_sane(|option_button| {
			option_button.add_item("SouthEast", 0);
			option_button.add_item("East", 1);
			option_button.add_item("NorthEast", 2);
			option_button.add_item("NorthWest", 3);
			option_button.add_item("West", 4);
			option_button.add_item("SouthWest", 5);

			option_button.select(self.end_direction as i64);
			option_button.connect("item_selected", unsafe { owner.assume_shared() },
				fn_name(&Self::_end_direction_changed), VariantArray::new_shared(), 0)
						 .log_if_err();
		});

		self.altitude_spin_box_octaves.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.altitude_octaves as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_altitude_octaves_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.altitude_spin_box_lacunarity.touch_assert_sane(|slider| {
			slider.set_value(self.altitude_lacunarity as f64);
			slider.connect("value_changed", unsafe { owner.assume_shared() },
						   fn_name(&Self::_altitude_lacunarity_changed), VariantArray::new_shared(), 0)
				  .log_if_err();
		});
		self.altitude_spin_box_frequency.touch_assert_sane(|slider| {
			slider.set_value(self.altitude_frequency as f64);
			slider.connect("value_changed", unsafe { owner.assume_shared() },
						   fn_name(&Self::_altitude_frequency_changed), VariantArray::new_shared(), 0)
				  .log_if_err();
		});

		self.biome_spin_box_octaves.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_octaves as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_octaves_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_spin_box_lacunarity.touch_assert_sane(|slider| {
			slider.set_value(self.biome_lacunarity as f64);
			slider.connect("value_changed", unsafe { owner.assume_shared() },
						   fn_name(&Self::_biome_lacunarity_changed), VariantArray::new_shared(), 0)
				  .log_if_err();
		});
		self.biome_spin_box_frequency.touch_assert_sane(|slider| {
			slider.set_value(self.biome_frequency as f64);
			slider.connect("value_changed", unsafe { owner.assume_shared() },
						   fn_name(&Self::_biome_frequency_changed), VariantArray::new_shared(), 0)
				  .log_if_err();
		});

		self.biome_1.touch_assert_safe(|biome, _| {
			self.biome_1_weight = biome.weight;
			self.biome_1_altitude_threshold = biome.altitude_threshold;
		});
		self.biome_2.touch_assert_safe(|biome, _| {
			self.biome_2_weight = biome.weight;
			self.biome_2_altitude_threshold = biome.altitude_threshold;
		});
		self.biome_3.touch_assert_safe(|biome, _| {
			self.biome_3_weight = biome.weight;
			self.biome_3_altitude_threshold = biome.altitude_threshold;
		});
		self.biome_4.touch_assert_safe(|biome, _| {
			self.biome_4_weight = biome.weight;
			self.biome_4_altitude_threshold = biome.altitude_threshold;
		});

		self.biome_1_weight_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_1_weight as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_1_weight_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_1_altitude_threshold_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_1_altitude_threshold as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_1_altitude_threshold_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_2_weight_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_2_weight as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_2_weight_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_2_altitude_threshold_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_2_altitude_threshold as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_2_altitude_threshold_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_3_weight_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_3_weight as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_3_weight_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_3_altitude_threshold_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_3_altitude_threshold as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_3_altitude_threshold_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_4_weight_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_4_weight as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_4_weight_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
		self.biome_4_altitude_threshold_spin_box.touch_assert_sane(|spin_box| {
			spin_box.set_value(self.biome_4_altitude_threshold as f64);
			spin_box.connect("value_changed", unsafe { owner.assume_shared() },
							 fn_name(&Self::_biome_4_altitude_threshold_changed), VariantArray::new_shared(), 0)
					.log_if_err();
		});
	}

	#[func] fn _hexagon_radius_changed(&mut self, value: f64) { self.hexagon_radius = value; }
	#[func] fn _map_width_changed(&mut self, value: f64) { self.map_width = value as i32; }
	#[func] fn _map_height_changed(&mut self, value: f64) { self.map_height = value as i32; }
	#[func]
	fn _map_shape_changed(&mut self, value: i64) {
		GridShape::try_from(value as u8)
			.map(|shape| self.map_shape = shape)
			.log_if_err();
	}
	#[func]
	fn _end_direction_changed(&mut self, value: i64) {
		HexagonDirection::try_from(value as u8)
			.map(|direction| self.end_direction = direction)
			.log_if_err();
	}

	#[func] fn _altitude_octaves_changed(&mut self, value: f64) { self.altitude_octaves = value as i32; }
	#[func] fn _altitude_lacunarity_changed(&mut self, value: f64) { self.altitude_lacunarity = value as f32; }
	#[func] fn _altitude_frequency_changed(&mut self, value: f64) { self.altitude_frequency = value as f32; }

	#[func] fn _biome_octaves_changed(&mut self, value: f64) { self.biome_octaves = value as i32; }
	#[func] fn _biome_lacunarity_changed(&mut self, value: f64) { self.biome_lacunarity = value as f32; }
	#[func] fn _biome_frequency_changed(&mut self, value: f64) { self.biome_frequency = value as f32; }

	#[func] fn _biome_1_weight_changed(&mut self, value: f64) { self.biome_1_weight = value as f32; }
	#[func] fn _biome_2_weight_changed(&mut self, value: f64) { self.biome_2_weight = value as f32; }
	#[func] fn _biome_3_weight_changed(&mut self, value: f64) { self.biome_3_weight = value as f32; }
	#[func] fn _biome_4_weight_changed(&mut self, value: f64) { self.biome_4_weight = value as f32; }
	#[func] fn _biome_1_altitude_threshold_changed(&mut self, value: f64) { self.biome_1_altitude_threshold = value as f32; }
	#[func] fn _biome_2_altitude_threshold_changed(&mut self, value: f64) { self.biome_2_altitude_threshold = value as f32; }
	#[func] fn _biome_3_altitude_threshold_changed(&mut self, value: f64) { self.biome_3_altitude_threshold = value as f32; }
	#[func] fn _biome_4_altitude_threshold_changed(&mut self, value: f64) { self.biome_4_altitude_threshold = value as f32; }

	fn delete_old(&mut self, owner: &Node) -> Gd<Node, Shared> {
		let old_parent = self.hexagon_parent;
		owner.remove_child(old_parent);
		old_parent.queue_free();

		let hex_parent_option = self.hexagon_parent_prefab
			.unwrap_refcount();
		let instance = hex_parent_option
			.instance(0)
			.unwrap();
		let hex_parent = instance;
		owner.add_child(hex_parent, false);
		owner.move_child(hex_parent, 0);
		self.hexagon_parent = unsafe { Some(hex_parent.assume_shared()) };
		self.spawned_hexagons.clear();

		return instance;
	}

	#[allow(unused)]
	fn random_sprite(collection: &VariantArray, rng: &mut Xoshiro256PlusPlus) -> Variant {
		return collection.get(rng.gen_range(0..collection.len()));
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

		return (altitude, biome);
	}

	fn set_hexagon_colors(&mut self, start: Axial, end: Axial) {
		let Some(full_map) = &self.current_map
			else {
				godot_error!("{}(): current_map is None", full_fn_name(&Self::set_hexagon_colors));
				return;
			};
		let Some(hex_parent) = self.hexagon_parent.assert_tref_if_sane()
			else {
				godot_error!("{}(): hex_parent is None", full_fn_name(&Self::set_hexagon_colors));
				return;
			};

		let prefab = self.hexagon_prefab.unwrap_refcount();
		let available_colors = get_available_colors();

		let hexagon_radius = self.hexagon_radius as f32;
		let black = Color::from_rgb(0., 0., 0.);
		for (_, pos, tile) in full_map.tiles.iter() {
			let hexagon_variant = prefab
				.instance(0);
			let hexagon = hexagon_variant

				.cast::<Sprite>()
				.unwrap();

			hex_parent.add_child(hexagon, false);
			self.spawned_hexagons.insert(*pos, unsafe { hexagon.assume_shared() });

			let (x, y) = pos.to_cartesian(hexagon_radius);
			hexagon.set_position(Vector2::new(x, y));

			let color =
				if tile.is_obstacle(){
					black
				}
				else {
					available_colors[tile.biome.id as usize]
				};

			hexagon.set_self_modulate(color);
		}

		let start_color = Color::from_rgb(1., 0., 1.);
		self.spawned_hexagons
			.get(&start)

			.set_self_modulate(start_color);

		let end_color = Color::from_rgb(1., 0., 1.);
		self.spawned_hexagons
			.get(&end)

			.set_self_modulate(end_color);
	}

	fn biomes(&self) -> [BiomeData; 4] {
		return [
			BiomeData { weight: self.biome_1_weight, altitude_threshold: self.biome_1_altitude_threshold },
			BiomeData { weight: self.biome_2_weight, altitude_threshold: self.biome_2_altitude_threshold },
			BiomeData { weight: self.biome_3_weight, altitude_threshold: self.biome_3_altitude_threshold },
			BiomeData { weight: self.biome_4_weight, altitude_threshold: self.biome_4_altitude_threshold },
		];
	}

	#[func]
	fn _button_pressed_generate_full(&mut self, #[base] owner: &Node) {
		self.delete_old(owner);

		let (altitude_noise, biome_noise) = self.noise_map();
		let biomes = self.biomes();
		let (base_map, start, end, _) = super::generate_full(self.map_width, self.map_height,
			self.map_shape, &biomes, &self.end_direction, altitude_noise, biome_noise);

		self.current_map = Some(base_map);
		self.set_hexagon_colors(start, end);
	}

	#[func]
	fn _button_pressed_generate_base(&mut self, #[base] owner: &Node) {
		self.delete_old(owner);

		let biomes = [
			BiomeData { weight: self.biome_1_weight, altitude_threshold: self.biome_1_altitude_threshold },
			BiomeData { weight: self.biome_2_weight, altitude_threshold: self.biome_2_altitude_threshold },
			BiomeData { weight: self.biome_3_weight, altitude_threshold: self.biome_3_altitude_threshold },
			BiomeData { weight: self.biome_4_weight, altitude_threshold: self.biome_4_altitude_threshold },
		];
		let (altitude_noise, biome_noise) = self.noise_map();

		let (base_map, start, end, _) = super::generate_base(self.map_width, self.map_height,
			self.map_shape, &biomes, &self.end_direction, altitude_noise, biome_noise);

		self.current_map = Some(base_map);
		self.set_hexagon_colors(start, end);
	}

	#[func]
	fn _button_pressed_ensure_connecteds(&mut self) {
		if let Some(map) = &mut self.current_map {
			let start_pos = map.start_pos;
			let end_pos = map.end_pos;
			pathing::ensure_open_areas_are_connected_to_start(map, start_pos, end_pos);
			self.set_hexagon_colors(start_pos, end_pos);
		}
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

*/
