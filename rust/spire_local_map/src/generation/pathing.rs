use bracket_pathfinding::prelude::{BaseMap, NavigationPath, SmallVec};
use rand::prelude::IteratorRandom;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};

use super::*;

#[allow(unused)]
pub fn path_between(map: &HexagonMap, start_index: usize, end_index: usize) -> NavigationPath {
	bracket_pathfinding::prelude::a_star_search(start_index, end_index, map)
}

fn last_towards(map: &HexagonMap, current: Axial, direction_arc: [Axial; 3]) -> Axial {
	let forward = current + direction_arc[0];
	if map.tiles.contains_key(&forward) {
		return last_towards(map, forward, direction_arc);
	}

	let upward = current + direction_arc[1];
	if map.tiles.contains_key(&upward) {
		return last_towards(map, upward, direction_arc);
	}

	let downward = current + direction_arc[2];
	if map.tiles.contains_key(&downward) {
		return last_towards(map, downward, direction_arc);
	}

	current
}

fn map_distance<'a>(from: Axial, hexagons: impl Iterator<Item = &'a Axial>) -> HashMap<Axial, u16> {
	hexagons
		.map(|hex| (*hex, from.manhattan_distance(hex)))
		.collect()
}

pub fn pick_start(
	map: &HexagonMap,
	rng: &mut Xoshiro256PlusPlus,
	end_direction: &HexagonDirection,
) -> (Axial, usize) {
	let start_direction = end_direction.reverse();
	let first_tile = *map.tiles.keys().next().unwrap();

	let farthest_towards = last_towards(map, first_tile, start_direction.arc_axial());
	let farthest_index = map.tiles.key_index(&farthest_towards).unwrap();

	let distance_map = map_distance(farthest_towards, map.tiles.keys());

	let Some(highest_distance_from_farthest) = distance_map.values().max()
	else {
		godot_error!(
			"{}(): highest_distance_from_farthest is None",
			full_fn_name(&pick_start)
		);
		return (farthest_towards, farthest_index);
	};

	let max_candidate_distance = u16::min(6, highest_distance_from_farthest / 5);
	if max_candidate_distance == 0 {
		godot_error!(
			"{}(): max_candidate_distance is 0",
			full_fn_name(&pick_start)
		);
		return (farthest_towards, farthest_index);
	}

	let candidates: HashSet<Axial> = distance_map
		.into_iter()
		.map(|(hex, distance)| (hex, 1. - (distance as f32 / max_candidate_distance as f32)))
		.filter_map(|(hex, weight)| (weight > 0.).then_some(hex))
		.collect();

	candidates
		.into_iter()
		.choose(rng)
		.map(|chosen| (chosen, map.tiles.key_index(&chosen).unwrap()))
		.unwrap_or_else(|| {
			godot_error!("{}(): chosen is None", full_fn_name(&pick_start));
			(farthest_towards, farthest_index)
		})
}

pub fn pick_end(
	map: &HexagonMap,
	rng: &mut Xoshiro256PlusPlus,
	end_direction: &HexagonDirection,
) -> (Axial, usize) {
	let first_tile = *map.tiles.keys().next().unwrap();

	let farthest_towards = last_towards(map, first_tile, end_direction.arc_axial());
	let farthest_index = map.tiles.key_index(&farthest_towards).unwrap();

	let distance_map = map_distance(farthest_towards, map.tiles.keys());

	let Some(highest_distance_from_farthest) = distance_map.values().max()
	else {
		godot_error!(
			"{}(): highest_distance_from_farthest is None",
			full_fn_name(&pick_start)
		);
		return (farthest_towards, farthest_index);
	};

	let max_candidate_distance = u16::min(6, highest_distance_from_farthest / 5);
	if max_candidate_distance == 0 {
		godot_error!(
			"{}(): max_candidate_distance is 0",
			full_fn_name(&pick_start)
		);
		return (farthest_towards, farthest_index);
	}

	let candidates: HashSet<Axial> = distance_map
		.into_iter()
		.map(|(hex, distance)| (hex, 1. - (distance as f32 / max_candidate_distance as f32)))
		.filter_map(|(hex, weight)| (weight > 0.).then_some(hex))
		.collect();

	candidates
		.into_iter()
		.choose(rng)
		.map(|chosen| (chosen, map.tiles.key_index(&chosen).unwrap()))
		.unwrap_or_else(|| {
			godot_error!("{}(): chosen is None", full_fn_name(&pick_start));
			(farthest_towards, farthest_index)
		})
}

fn get_connecteds(map: &HexagonMap, start: Axial) -> HashSet<Axial> {
	let mut results = HashSet::new();
	recursive_add(start, &mut results, map);
	return results;

	fn recursive_add(current: Axial, results: &mut HashSet<Axial>, map: &HexagonMap) {
		results.insert(current);
		current.neighbors().iter().for_each(|(_, neighbor)| {
			if map
				.tiles
				.get_value(neighbor)
				.is_some_and(|tile| !tile.is_obstacle())
				&& !results.contains(neighbor)
			{
				recursive_add(*neighbor, results, map);
			}
		});
	}
}

struct FlyingPathfinder<'a> {
	map: &'a HexagonMap,
}

impl BaseMap for FlyingPathfinder<'_> {
	fn is_opaque(&self, _: usize) -> bool { false }

	fn get_available_exits(&self, center_index: usize) -> SmallVec<[(usize, f32); 10]> {
		let Some(center) = self.map.tiles.key_at(center_index)
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
			.filter_map(|(_, neighbor)| self.map.tiles.key_index(neighbor).map(|idx| (idx, 1.)))
			.collect()
	}

	fn get_pathing_distance(&self, origin_index: usize, destination_index: usize) -> f32 {
		let Some(origin) = self.map.tiles.key_at(origin_index)
		else {
			godot_error!(
				"{}(): origin is None",
				full_fn_name(&Self::get_pathing_distance)
			);
			return 1000.;
		};
		let Some(destination) = self.map.tiles.key_at(destination_index)
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

fn direct_path_between(start: Axial, end: Axial, map: &HexagonMap) -> Vec<Axial> {
	let start_index = map.tiles.key_index(&start).unwrap();
	let end_index = map.tiles.key_index(&end).unwrap();

	let path = bracket_pathfinding::prelude::a_star_search(
		start_index,
		end_index,
		&FlyingPathfinder { map },
	);
	assert!(
		path.success,
		"{}(): path.success is false",
		full_fn_name(&direct_path_between)
	);
	path.steps
		.into_iter()
		.map(|index| *map.tiles.key_at(index).unwrap())
		.collect()
}

pub fn ensure_open_areas_are_connected_to_start(
	tile_map: &mut HexagonMap,
	start: Axial,
	end: Axial,
) {
	let mut rng = Xoshiro256PlusPlus::from_entropy();

	let mut already_connected = get_connecteds(tile_map, start);

	let mut not_connected = tile_map
		.tiles
		.iter()
		.filter_map(|(pos, tile)| {
			(!tile.is_obstacle() && !already_connected.contains(pos)).then_some(*pos)
		})
		.collect::<HashSet<Axial>>();

	while let Some(next) = not_connected.take_any() {
		let connecteds_to_next = get_connecteds(tile_map, next);
		if next != end && connecteds_to_next.is_empty() {
			continue;
		}

		let chosen_unconnected_to_next = {
			let distance_map = {
				let mut temp = map_distance(next, already_connected.iter())
					.into_iter()
					.collect::<Vec<_>>();

				temp.sort_by(|(_, a_distance), (_, b_distance)| a_distance.cmp(b_distance));
				temp
			};

			distance_map
				.iter()
				.take(usize::max(1, distance_map.len() / 5))
				.map(|(hex, _)| *hex)
				.choose(&mut rng)
				.unwrap_or(start)
		};

		let chosen_connected_to_next = {
			let distance_map = {
				let mut temp = map_distance(chosen_unconnected_to_next, connecteds_to_next.iter())
					.into_iter()
					.collect::<Vec<_>>();

				temp.sort_by(|(_, a_distance), (_, b_distance)| a_distance.cmp(b_distance));
				temp
			};

			distance_map
				.iter()
				.take(usize::max(1, distance_map.len() / 5))
				.map(|(hex, _)| *hex)
				.choose(&mut rng)
				.unwrap_or(start)
		};

		direct_path_between(
			chosen_unconnected_to_next,
			chosen_connected_to_next,
			tile_map,
		)
		.into_iter()
		.for_each(|hex| {
			let contents = &mut tile_map.tiles.get_value_mut(&hex).unwrap().contents;
			*contents = TileContents::Empty;
			already_connected.insert(hex);
		});

		not_connected.remove_many(connecteds_to_next.iter());
	}
}