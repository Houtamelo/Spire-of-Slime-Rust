use bracket_pathfinding::prelude::{BaseMap, SmallVec};
use gdnative::godot_error;
use util::prelude::IndexedHashMap;
use serde::{Deserialize, Serialize};

use crate::local_map::coordinates::axial::Axial;
use crate::local_map::tile::Tile;

#[derive(Debug, Serialize, Deserialize)]
pub struct HexagonMap {
	pub width: i16,
	pub height: i16,
	pub tiles: IndexedHashMap<Axial, Tile>,
	pub start_pos: Axial,
	pub end_pos: Axial,
}

impl BaseMap for HexagonMap {
	fn is_opaque(&self, _idx: usize) -> bool {
		return true;
	}

	fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
		let Some(center) = self.tiles.index_to_key(&index)
			else {
				godot_error!("{}(): center is None", util::full_fn_name(&Self::get_available_exits));
				return SmallVec::new();
			};
		
		return center.neighbors()
			.iter()
			.filter_map(|(_, neighbor)|
				if self.tiles.get(neighbor).is_some_and(|tile| !tile.is_obstacle()) {
					let neighbor_index = *self.tiles.key_to_index(neighbor).unwrap();
					Some((neighbor_index, 1.))
				} else {
					None
				})
			.collect();
	}

	fn get_pathing_distance(&self, origin_index: usize, destination_index: usize) -> f32 {
		let Some(origin) = self.tiles.index_to_key(&origin_index)
			else {
				godot_error!("{}(): origin is None", util::full_fn_name(&Self::get_pathing_distance));
				return 1000.;
			};
		let Some(destination) = self.tiles.index_to_key(&destination_index)
			else {
				godot_error!("{}(): destination is None", util::full_fn_name(&Self::get_pathing_distance));
				return 1000.;
			};
		
		return origin.manhattan_distance(destination) as f32;
	}
}