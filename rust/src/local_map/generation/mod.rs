use std::collections::HashMap;
use std::ops::RangeInclusive;

use bracket_noise::prelude::FastNoise;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use map::HexagonMap;

use crate::local_map::coordinates::axial::Axial;
use crate::local_map::coordinates::direction::HexagonDirection;
use crate::local_map::generation::noise_grid::{GridShape, NoiseInfo};
use crate::local_map::generation::pathing::ensure_open_areas_are_connected_to_start;
use crate::local_map::tile::{Biome, Tile, TileContents};

mod noise_grid;
mod pathing;
mod map;
pub mod generator_ui;

#[derive(Debug, Default)]
pub struct BiomeData {
	pub weight: f32,
	pub altitude_threshold: f32,
}

pub fn generate_full(width: i16, height: i16, shape: GridShape, biomes: &[BiomeData], 
                     end_direction: &HexagonDirection, altitude_generator: FastNoise, biome_generator: FastNoise) 
	-> (HexagonMap, Axial, Axial, Xoshiro256PlusPlus) 
{
	let (mut base_map, start, end, rng) = generate_base(width, height,
		shape, biomes, end_direction, altitude_generator, biome_generator);
	ensure_open_areas_are_connected_to_start(&mut base_map, start, end);
	return (base_map, start, end, rng);
}

pub fn generate_base(width: i16, height: i16, shape: GridShape, biomes: &[BiomeData],
                     end_direction: &HexagonDirection, altitude_generator: FastNoise, biome_generator: FastNoise) 
	-> (HexagonMap, Axial, Axial, Xoshiro256PlusPlus)
{
	let mut rng = Xoshiro256PlusPlus::from_entropy();
	
	let base_grid: HashMap<Axial, NoiseInfo> = noise_grid::generate_grid(width, height, shape, altitude_generator, biome_generator);

	let mut tile_map = tiled_deserted_map(width, height, biomes, base_grid);
	
	let (start, _) = pathing::pick_start(&tile_map, &mut rng, end_direction);
	let (end, _) = pathing::pick_end(&tile_map, &mut rng, end_direction);
	tile_map.start_pos = start;
	tile_map.end_pos = end;
	
	tile_map.tiles.get_mut(&start).unwrap().contents = TileContents::Empty;
	tile_map.tiles.get_mut(&end).unwrap().contents = TileContents::Empty;
	return (tile_map, start, end, rng);
}

fn tiled_deserted_map(width: i16, height: i16, biomes: &[BiomeData],
                      base_grid: HashMap<Axial, NoiseInfo>) -> HexagonMap {

	let weight_sum = biomes.iter()
		.fold(0., |mut sum, biome| {
			sum += biome.weight;
			sum
		});

	let biome_thresholds: Vec<(&BiomeData, RangeInclusive<f32>)> = {
		let mut current = 0.;
		biomes.iter()
			.map(|biome| {
				let end = current + biome.weight / weight_sum;
				let range = current..=end;
				current = end;
				(biome, range)
			}).collect()
	};
	
	let tiles = base_grid.into_iter()
		.map(|(pos, info)| {
			let (index, (biome_data, _)) = biome_thresholds.iter()
				.enumerate()
				.find(|(_, (_, range))| range.contains(&info.biome))
				.unwrap_or((0, &biome_thresholds[0]));

			let tile = {
				let mut temp = Tile::default();
				temp.biome = Biome { id: index as u8 };
				if info.altitude >= biome_data.altitude_threshold {
					temp.contents = TileContents::Obstacle;
				}
				
				temp
			};

			(pos, tile)
		}).collect();
	

	return HexagonMap {
		width,
		height,
		tiles,
		start_pos: Axial::ZERO,
		end_pos: Axial::ZERO,
	};
}


