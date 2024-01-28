use std::collections::HashMap;
use crate::local_map::coordinates::axial::Axial;
use crate::local_map::coordinates::offset::Offset;
use crate::local_map::generation::HexInfo;

pub(super) struct MapBounds {
	pub min_x: f32,
	pub min_y: f32,
	pub max_x: f32,
	pub max_y: f32,
}

impl MapBounds {
	pub fn width(&self) -> f32 { return self.max_x - self.min_x; }
	pub fn height(&self) -> f32 { return self.max_y - self.min_y; }
}

pub fn fill(fill_me: &mut HashMap<Axial, HexInfo>, width: i16, height: i16, hex_radius: f32) -> MapBounds {
	let offset: Axial = Axial::from(Offset { col: -width / 2, row: -height / 2 });
	
	let max_size = i16::max(width, height);
	
	for q in -max_size..=max_size {
		let r1 = i16::max(-max_size, -q - max_size);
		let r2 = i16::min(max_size, -q + max_size);
		
		for r in r1..=r2 {
			fill_me.insert(Axial { q, r } + offset, HexInfo::default());
		}
	}
	
	let min_x = fill_me.keys()
		.map(|hex| hex.to_cartesian(hex_radius).0)
		.reduce(|x, rhs| f32::min(x, rhs))
		.unwrap_or(0.);
	
	let min_y = fill_me.keys()
		.map(|hex| hex.to_cartesian(hex_radius).1)
		.reduce(|y, rhs| f32::min(y, rhs))
		.unwrap_or(0.);
	
	let max_x = fill_me.keys()
		.map(|hex| hex.to_cartesian(hex_radius).0)
		.reduce(|x, rhs| f32::max(x, rhs))
		.unwrap_or(0.);
	
	let max_y = fill_me.keys()
		.map(|hex| hex.to_cartesian(hex_radius).1)
		.reduce(|y, rhs| f32::max(y, rhs))
		.unwrap_or(0.);
	
	return MapBounds { min_x, min_y, max_x, max_y };
}