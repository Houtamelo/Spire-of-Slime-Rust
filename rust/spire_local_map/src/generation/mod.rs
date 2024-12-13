#![allow(unused)] //todo!
pub use generator_ui::*;
pub use map::*;
pub use noise_grid::*;

use super::*;

mod generator_ui;
mod map;
mod noise_grid;
mod pathing;

#[derive(Debug, Default)]
pub struct BiomeData {
	pub weight: f32,
	pub altitude_threshold: f32,
}

pub(super) fn generate_full(
	width: i32,
	height: i32,
	shape: GridShape,
	biomes: &[BiomeData],
	end_direction: &HexagonDirection,
	altitude_generator: FastNoise,
	biome_generator: FastNoise,
) -> (HexagonMap, Axial, Axial, Xoshiro256PlusPlus) {
	let (mut base_map, start, end, rng) = generate_base(
		width,
		height,
		shape,
		biomes,
		end_direction,
		altitude_generator,
		biome_generator,
	);

	pathing::ensure_open_areas_are_connected_to_start(&mut base_map, start, end);
	(base_map, start, end, rng)
}

pub(super) fn generate_base(
	width: i32,
	height: i32,
	shape: GridShape,
	biomes: &[BiomeData],
	end_direction: &HexagonDirection,
	altitude_generator: FastNoise,
	biome_generator: FastNoise,
) -> (HexagonMap, Axial, Axial, Xoshiro256PlusPlus) {
	let mut rng = Xoshiro256PlusPlus::from_entropy();

	let base_grid: HashMap<Axial, NoiseInfo> =
		noise_grid::generate_grid(width, height, shape, altitude_generator, biome_generator);

	let mut tile_map = tiled_deserted_map(width, height, biomes, base_grid);

	let (start, _) = pathing::pick_start(&tile_map, &mut rng, end_direction);
	let (end, _) = pathing::pick_end(&tile_map, &mut rng, end_direction);
	tile_map.start_pos = start;
	tile_map.end_pos = end;

	tile_map.tiles.get_value_mut(&start).unwrap().contents = TileContents::Empty;
	tile_map.tiles.get_value_mut(&end).unwrap().contents = TileContents::Empty;
	(tile_map, start, end, rng)
}

fn tiled_deserted_map(
	width: i32,
	height: i32,
	biomes: &[BiomeData],
	base_grid: HashMap<Axial, NoiseInfo>,
) -> HexagonMap {
	let weight_sum = biomes.iter().fold(0., |mut sum, biome| {
		sum += biome.weight;
		sum
	});

	let biome_thresholds: Vec<(&BiomeData, RangeInclusive<f32>)> = {
		let mut current = 0.;
		biomes
			.iter()
			.map(|biome| {
				let end = current + biome.weight / weight_sum;
				let range = current..=end;
				current = end;
				(biome, range)
			})
			.collect()
	};

	let tiles = base_grid
		.into_iter()
		.map(|(pos, info)| {
			let (index, (biome_data, _)) = biome_thresholds
				.iter()
				.enumerate()
				.find(|(_, (_, range))| range.contains(&info.biome))
				.unwrap_or((0, &biome_thresholds[0]));

			let tile = {
				let mut temp = Tile {
					biome: Biome { id: index as u8 },
					..Tile::default()
				};

				if info.altitude >= biome_data.altitude_threshold {
					temp.contents = TileContents::Obstacle;
				}

				temp
			};

			(pos, tile)
		})
		.collect();

	HexagonMap {
		width,
		height,
		tiles,
		start_pos: Axial::ZERO,
		end_pos: Axial::ZERO,
	}
}
