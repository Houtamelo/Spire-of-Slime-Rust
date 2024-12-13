use bracket_pathfinding::prelude::{BaseMap, SmallVec};

use super::*;

#[derive(Serialize, Deserialize)]
pub struct HexagonMap {
	pub width:     i32,
	pub height:    i32,
	pub tiles:     IndexedMap<Axial, Tile>,
	pub start_pos: Axial,
	pub end_pos:   Axial,
}

impl BaseMap for HexagonMap {
	fn is_opaque(&self, _idx: usize) -> bool { true }

	fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
		let Some(center) = self.tiles.key_at(index)
		else {
			godot_error!(
				"{}(): center is None",
				full_fn_name(&Self::get_available_exits)
			);
			return SmallVec::new();
		};

		center
			.neighbors()
			.iter()
			.filter_map(|(_, neighbor)| {
				if self
					.tiles
					.get_value(neighbor)
					.is_some_and(|tile| !tile.is_obstacle())
				{
					let neighbor_index = self.tiles.key_index(neighbor).unwrap();
					Some((neighbor_index, 1.))
				} else {
					None
				}
			})
			.collect()
	}

	fn get_pathing_distance(&self, origin_index: usize, destination_index: usize) -> f32 {
		let Some(origin) = self.tiles.key_at(origin_index)
		else {
			godot_error!(
				"{}(): origin is None",
				full_fn_name(&Self::get_pathing_distance)
			);
			return 1000.;
		};
		let Some(destination) = self.tiles.key_at(destination_index)
		else {
			godot_error!(
				"{}(): destination is None",
				full_fn_name(&Self::get_pathing_distance)
			);
			return 1000.;
		};

		origin.manhattan_distance(destination) as f32
	}
}
