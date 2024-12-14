use bracket_noise::prelude::FastNoise;

use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct NoiseInfo {
	pub altitude: f32,
	pub biome: f32,
}

#[derive(Default, Debug, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = u8)]
#[repr(u8)]
pub enum GridShape {
	#[default]
	Hexagon = 0,
	Rectangle = 1,
	Parallelogram = 2,
	Triangle = 3,
}

impl TryFrom<u8> for GridShape {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(GridShape::Hexagon),
			1 => Ok(GridShape::Rectangle),
			2 => Ok(GridShape::Parallelogram),
			3 => Ok(GridShape::Triangle),
			_ => Err(format!("Invalid GridShape value: {value}")),
		}
	}
}

pub fn generate_grid(
	width: i32,
	height: i32,
	shape: GridShape,
	altitude_generator: FastNoise,
	biome_generator: FastNoise,
) -> HashMap<Axial, NoiseInfo> {
	let base_grid = match shape {
		GridShape::Hexagon => hexagon_grid(width, height),
		GridShape::Rectangle => rectangle_grid(width, height),
		GridShape::Parallelogram => parallel_gram_grid(width, height),
		GridShape::Triangle => triangle_grid(width, height),
	};

	map_noise(base_grid, altitude_generator, biome_generator)
}

fn hexagon_grid(width: i32, height: i32) -> HashSet<Axial> {
	let mut grid = HashSet::new();

	let map_size = i32::max(width, height) / 2;
	let offset: Axial = Axial::from(Offset {
		col: 8 + (-map_size / 2),
		row: 8 + (-map_size / 2),
	});

	for q in (-map_size)..=map_size {
		let r1 = i32::max(-map_size, -q - map_size);
		let r2 = i32::min(map_size, -q + map_size);

		for r in r1..=r2 {
			grid.insert(Axial { q, r } + offset);
		}
	}

	grid
}

fn rectangle_grid(width: i32, height: i32) -> HashSet<Axial> {
	let mut tiles = HashSet::new();
	let offset: Axial = Axial::from(Offset {
		col: -width / 2,
		row: -height / 2,
	});

	for r in 0..height {
		let r_off = r >> 1;
		for q in (-r_off)..(width - r_off) {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}

	tiles
}

fn parallel_gram_grid(width: i32, height: i32) -> HashSet<Axial> {
	let mut tiles = HashSet::new();
	let offset: Axial = Axial::from(Offset {
		col: -width / 2,
		row: -height / 2,
	});

	for q in 0..=width {
		for r in 0..=height {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}

	tiles
}

fn triangle_grid(width: i32, height: i32) -> HashSet<Axial> {
	let map_size = i32::max(width, height);
	let offset: Axial = Axial::from(Offset {
		col: -map_size / 2,
		row: -map_size / 2,
	});

	let mut tiles = HashSet::new();
	for q in 0..=map_size {
		for r in 0..=(map_size - q) {
			let tile = Axial { q, r } + offset;
			tiles.insert(tile);
		}
	}

	tiles
}

fn map_noise(
	grid: HashSet<Axial>,
	altitude_generator: FastNoise,
	biome_generator: FastNoise,
) -> HashMap<Axial, NoiseInfo> {
	grid.into_iter()
		.map(|hex| {
			let (x, y) = hex.to_cartesian(1.);
			let altitude = sample_noise(&altitude_generator, x, y, 1.);
			let biome = sample_noise(&biome_generator, x, y, 1.);
			(hex, NoiseInfo { altitude, biome })
		})
		.collect()
}

fn sample_noise(noise: &FastNoise, x: f32, y: f32, radius: f32) -> f32 {
	0.5 + noise.get_noise(x / (100. * radius), y / (100. * radius))
}
