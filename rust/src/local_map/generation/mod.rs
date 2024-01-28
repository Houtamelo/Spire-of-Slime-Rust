mod grid;

use std::collections::HashMap;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils_gdnative::prelude::*;
use rand::Rng;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::local_map::coordinates::axial::Axial;
use crate::util;

#[extends(Node)]
pub struct MapGenerator {
	#[export_path] hexagon_parent: Option<Ref<Node>>,
	#[export_path] button_generate: Option<Ref<Button>>,
	#[property] hexagon_prefab: Option<Ref<PackedScene>>,
	#[property] hexagon_radius: f32,
	#[property] altitude_threshold: f32,
	#[property] map_width: i16,
	#[property] map_height: i16,
	spawned_hexagons: Vec<Ref<Sprite>>,
	#[property] terrain_type_0_25_walkable: VariantArray,
	#[property] terrain_type_0_25_obstacle: VariantArray,
	#[property] terrain_type_25_50_walkable: VariantArray,
	#[property] terrain_type_25_50_obstacle: VariantArray,
	#[property] terrain_type_50_75_walkable: VariantArray,
	#[property] terrain_type_50_75_obstacle: VariantArray,
	#[property] terrain_type_75_100_walkable: VariantArray,
	#[property] terrain_type_75_100_obstacle: VariantArray,
}

#[methods]
impl MapGenerator {
	#[method]
	fn _ready(&mut self, #[base] owner: &Node) {
		self.grab_nodes_by_path(owner);
		
		self.button_generate.unwrap_manual()
			.connect("pressed", unsafe { owner.assume_shared() }, util::fn_name(&Self::_button_generate_pressed), 
			         VariantArray::new_shared(), 0)
			.log_if_err();
	}
	
	#[method]
	fn _button_generate_pressed(&mut self) {
		for hexagon in self.spawned_hexagons.drain(..) {
			hexagon.touch_if_sane(|hex| { 
				self.hexagon_parent.unwrap_manual().remove_child(hex);
				hex.queue_free();
			});
		}
		
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		
		let prefab = self.hexagon_prefab.unwrap_refcount();
		let hex_parent = self.hexagon_parent.unwrap_manual();
		
		for (pos, info) in generate(self.map_width, self.map_height) {
			let hexagon_variant = prefab.instance(0);
			let hexagon = hexagon_variant
				.unwrap_manual()
				.cast::<Sprite>()
				.unwrap();
			
			hex_parent.add_child(hexagon, false);
			self.spawned_hexagons.push(unsafe { hexagon.assume_shared() });
			
			let (x, y) = pos.to_cartesian(self.hexagon_radius);
			hexagon.set_position(Vector2::new(x as f32, y as f32));
			
			let is_obstacle = info.altitude > self.altitude_threshold;
			
			let texture_variant = match (info.biome, is_obstacle) {
				(0.0..=0.25, false) => Self::random_sprite(&self.terrain_type_0_25_walkable, &mut rng),
				(0.0..=0.25, true) => Self::random_sprite(&self.terrain_type_0_25_obstacle, &mut rng),
				(0.0..=0.50, false) => Self::random_sprite(&self.terrain_type_25_50_walkable, &mut rng),
				(0.0..=0.50, true) => Self::random_sprite(&self.terrain_type_25_50_obstacle, &mut rng),
				(0.0..=0.75, false) => Self::random_sprite(&self.terrain_type_50_75_walkable, &mut rng),
				(0.0..=0.75, true) => Self::random_sprite(&self.terrain_type_50_75_obstacle, &mut rng),
				(_, false) => Self::random_sprite(&self.terrain_type_75_100_walkable, &mut rng),
				(_, true) => Self::random_sprite(&self.terrain_type_75_100_obstacle, &mut rng),
			};
			
			let texture_cast = texture_variant.to_object::<Texture>();
			let texture = texture_cast.unwrap_refcount();
			hexagon.set_texture(texture);
		}
	}
	
	fn random_sprite(collection: &VariantArray, rng: &mut Xoshiro256PlusPlus) -> Variant {
		return collection.get(rng.gen_range(0..collection.len()));
	}
}

#[derive(Debug, Clone, Copy, Default)]
struct HexInfo { pub altitude: f32, pub biome: f32 }

fn generate(width: i16, height: i16) -> HashMap<Axial, HexInfo> {
	let mut hexagons = HashMap::new();
	let _ = grid::fill(&mut hexagons, width, height, 1.);
	
	let (altitude, biome) = noise_map();
	
	for (hex, info) in hexagons.iter_mut() {
		let (x, y) = hex.to_cartesian(1.);
		info.altitude = sample_noise(&altitude, x, y, 1.);
		info.biome = sample_noise(&biome, x, y, 1.);
	}
	
	return hexagons;
}

fn sample_noise(noise: &FastNoise, x: f32, y: f32, radius: f32) -> f32 {
	return 0.5 + noise.get_noise(x / radius, y / radius);
}

fn noise_map() -> (FastNoise, FastNoise) {
	let mut rng = Xoshiro256PlusPlus::from_entropy();

	let altitude = {
		let mut temp = FastNoise::seeded(rng.next_u64());
		temp.set_noise_type(NoiseType::PerlinFractal);
		temp.set_fractal_type(FractalType::FBM);
		temp.set_fractal_octaves(3);
		temp.set_fractal_gain(0.5);
		temp.set_fractal_lacunarity(3.);
		temp.set_frequency(16.0);
		temp
	};
	
	let biome = {
		let mut temp = FastNoise::seeded(rng.next_u64());
		temp.set_noise_type(NoiseType::PerlinFractal);
		temp.set_fractal_type(FractalType::FBM);
		temp.set_fractal_octaves(3);
		temp.set_fractal_gain(0.5);
		temp.set_fractal_lacunarity(0.25);
		temp.set_frequency(2.0);
		temp
	};
	
	return (altitude, biome);
}

