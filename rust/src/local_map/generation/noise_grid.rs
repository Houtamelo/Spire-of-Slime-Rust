use std::collections::{HashMap, HashSet};
use bracket_noise::prelude::FastNoise;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use gdnative::prelude::{ExportInfo, FromVariant, ToVariant};
use crate::local_map::coordinates::axial::Axial;
use crate::local_map::coordinates::offset::Offset;

#[derive(Debug, Clone, Copy, Default)]
pub struct NoiseInfo {
	pub altitude: f32,
	pub biome: f32
}

#[derive(Debug, Clone, Copy, FromVariant, ToVariant)]
#[repr(u8)]
pub enum GridShape {
	Hexagon = 0,
	Rectangle = 1,
	Parallelogram = 2,
	Triangle = 3,
}

impl TryFrom<u8> for GridShape {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		return match value {
			0 => Ok(GridShape::Hexagon),
			1 => Ok(GridShape::Rectangle),
			2 => Ok(GridShape::Parallelogram),
			3 => Ok(GridShape::Triangle),
			_ => Err(format!("Invalid GridShape value: {value}")),
		};
	}
}

impl Default for GridShape {
	fn default() -> Self {
		return GridShape::Hexagon;
	}
}

impl Export for GridShape {
	type Hint = IntHint<u8>;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		return Self::Hint::Enum(EnumHint::new(vec![
			"Hexagon".to_owned(),
			"Rectangle".to_owned(),
			"Parellelogram".to_owned(),
			"Triangle".to_owned()]
		)).export_info();
	}
}

pub fn generate_grid(width: i16, height: i16, shape: GridShape,
                     altitude_generator: FastNoise, biome_generator: FastNoise)
                     -> HashMap<Axial, NoiseInfo> {
	let base_grid = match shape {
		GridShape::Hexagon => hexagon_grid(width, height),
		GridShape::Rectangle => rectangle_grid(width, height),
		GridShape::Parallelogram => parallelo_gram_grid(width, height),
		GridShape::Triangle => triangle_grid(width, height),
	};
	
	return map_noise(base_grid, altitude_generator, biome_generator);
}

fn hexagon_grid(width: i16, height: i16) -> HashSet<Axial> {
	let mut grid = HashSet::new();

	let map_size = i16::max(width, height) / 2;
	let offset: Axial = Axial::from(Offset { col: 8 + (-map_size / 2), row: 8 + (-map_size / 2) });
	
	for q in (-map_size)..=map_size {
		let r1 = i16::max(-map_size, -q - map_size);
		let r2 = i16::min(map_size, -q + map_size);
		
		for r in r1..=r2 {
			grid.insert(Axial { q, r } + offset);
		}
	}
	
	return grid;
}

fn rectangle_grid(width: i16, height: i16) -> HashSet<Axial> {
	let mut tiles = HashSet::new();
	let offset: Axial = Axial::from(Offset { col: -width / 2, row: -height / 2 });
	
	for r in 0..height {
		let r_off = r >> 1;
		for q in (-r_off)..(width - r_off) {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}
	
	return tiles;
}

fn parallelo_gram_grid(width: i16, height: i16) -> HashSet<Axial> {
	let mut tiles = HashSet::new();
	let offset: Axial = Axial::from(Offset { col: -width / 2, row: -height / 2 });
	
	for q in 0..=width {
		for r in 0..=height {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}
	
	return tiles;
}

fn triangle_grid(width: i16, height: i16) -> HashSet<Axial> {
	let map_size = i16::max(width, height);
	let offset: Axial = Axial::from(Offset { col: -map_size / 2, row: -map_size / 2 });
	
	let mut tiles = HashSet::new();
	for q in 0..=map_size {
		for r in 0..=(map_size - q) {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}
	
	return tiles;
}

fn map_noise(grid: HashSet<Axial>, altitude_generator: FastNoise, biome_generator: FastNoise) 
	-> HashMap<Axial, NoiseInfo> {
	return grid.into_iter()
		.map(|hex| {
			let (x, y) = hex.to_cartesian(1.);
			let altitude = sample_noise(&altitude_generator, x, y, 1.);
			let biome = sample_noise(&biome_generator, x, y, 1.);
			(hex, NoiseInfo { altitude, biome }) })
		.collect();
}


fn sample_noise(noise: &FastNoise, x: f32, y: f32, radius: f32) -> f32 {
	return 0.5 + noise.get_noise(x / (100. * radius), y / (100. * radius));
}
