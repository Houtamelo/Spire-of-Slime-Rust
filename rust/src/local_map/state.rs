#[allow(unused_imports)]
use crate::*;

use crate::combat::CombatState;
use crate::local_map::coordinates::axial::Axial;
use crate::local_map::tile;
use crate::local_map::tile::{EnemyGroup, EventID, Tile, TileContents, TileMistStatus};
use crate::local_map::tile::TileScoutStatus::ContentsRevealed;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalMapState {
	tiles: HashMap<Axial, Tile>,
	party_pos: Axial,
	party_state: PartyState,
	ethel_exhaustion: PercentageU8,
	nema_exhaustion: PercentageU8,
}

impl LocalMapState {
	const DEFAULT_TURN_TIME: u8 = 100;
	const COST_SCOUT: u8 = 3;
	const COST_CLEAR_MIST: u8 = 3;

	pub fn input_walk(&mut self, tile_pos: Axial) -> Vec<InputResult> {
		const COST_WALK: u8 = 1;

		if (self.ethel_exhaustion.get() + COST_WALK) > 100 
			|| (self.nema_exhaustion.get() + COST_WALK) > 100 {
			return Vec::new();
		}

		let player_pos = self.party_pos;

		let Some(tile) = self.tiles.get_mut(&tile_pos)
			else {
				godot_warn!("LocalMapState::tile_left_click(): Warning: Position of tile clicked  is not mapped: {tile_pos:?}");
				return Vec::new();
			};

		if Axial::manhattan_distance(&player_pos, &tile_pos) != 1 {
			return Vec::new();
		}

		return match &self.party_state {
			state @ (PartyState::InCombat(_) | PartyState::InEvent(_))
			=> {
				godot_warn!(
					"LocalMapState::tile_left_click(): Warning: Received walk input while in combat or event!\
					state: {state:?}\
					pos: {player_pos:?}");
				Vec::new()
			}
			PartyState::Idle => {
				if let TileContents::Obstacle = tile.contents {
					return Vec::new();
				}

				self.ethel_exhaustion += COST_WALK;
				self.nema_exhaustion += COST_WALK;
				self.party_pos = tile_pos;

				let mut results = vec![InputResult::Animation_Walk(tile_pos)];
				match mem::take(&mut tile.contents) {
					TileContents::Enemies(enemies) => { results.push(InputResult::Combat(enemies)); }
					TileContents::RestSite => { results.push(InputResult::LongRest); }
					TileContents::Empty => { results.extend(self.pass_time(Self::DEFAULT_TURN_TIME)); }
					TileContents::Trap(event_id)
					| TileContents::Event(event_id) => {
						results.push(InputResult::Event(event_id));
					}
					TileContents::Obstacle => {
						unreachable!()
					}
				}

				results
			}
		};
	}

	pub fn input_run(&mut self, tile_pos: Axial) -> Vec<InputResult> {
		const COST_RUN: u8 = 4;

		if (self.ethel_exhaustion.get() + COST_RUN) > 100
			|| (self.nema_exhaustion.get() + COST_RUN) > 100 {
			return Vec::new();
		}

		let player_pos = self.party_pos;

		let Some(tile) = self.tiles.get_mut(&tile_pos)
			else {
				godot_warn!("LocalMapState::tile_left_click(): Warning: Position of tile clicked  is not mapped: {tile_pos:?}");
				return Vec::new();
			};

		if Axial::manhattan_distance(&player_pos, &tile_pos) != 1 {
			return Vec::new();
		}

		return match &self.party_state {
			state @ (PartyState::InCombat(_) | PartyState::InEvent(_))
			=> {
				godot_warn!("{}(): Received run input while in combat or event!\n\
					state: {state:?}\n
					pos: {player_pos:?}",
					full_fn_name(&Self::input_run));
				Vec::new()
			}
			PartyState::Idle => {
				if let TileContents::Obstacle = tile.contents {
					return Vec::new();
				}

				self.ethel_exhaustion += COST_RUN;
				self.nema_exhaustion += COST_RUN;
				self.party_pos = tile_pos;

				let mut results = vec![InputResult::Animation_Run(tile_pos)];
				match mem::take(&mut tile.contents) {
					TileContents::Enemies(enemies) => { results.push(InputResult::Combat(enemies)); }
					TileContents::RestSite => { results.push(InputResult::LongRest); }
					TileContents::Empty => {}
					TileContents::Trap(event_id)
					| TileContents::Event(event_id) => {
						results.push(InputResult::Event(event_id));
					}
					TileContents::Obstacle => {
						unreachable!()
					}
				}

				results
			}
		};
	}

	pub fn input_scout(&mut self) -> Vec<InputResult> {
		const COST_SCOUT: u8 = 3;

		if (self.ethel_exhaustion.get() + COST_SCOUT) > 100 {
			return Vec::new();
		}

		let player_pos = self.party_pos;

		if let state @ (PartyState::InCombat(_) | PartyState::InEvent(_)) = &self.party_state {
			godot_warn!(
					"LocalMapState::input_scout(): Warning: Received scout input while in combat or event!\
					state: {state:?}\
					pos: {player_pos:?}");
			return Vec::new();
		}

		let mut any_changed = false;
		reveal_adjacents(player_pos, &mut self.tiles, 3, &mut HashSet::new(), &mut any_changed);

		return if any_changed {
			self.ethel_exhaustion += COST_SCOUT;
			let mut results = vec![InputResult::Animation_Scout];
			results.extend(self.pass_time(Self::DEFAULT_TURN_TIME));
			results
		} else {
			Vec::new()
		};

		fn reveal_adjacents(axial: Axial, map: &mut HashMap<Axial, Tile>, steps: usize, already_checked: &mut HashSet<Axial>, any_changed: &mut bool) {
			axial.neighbors().iter()
			     .for_each(|(_, pos)| {
				     let Some(tile) = map.get_mut(pos)
				        else { return; };
				     
				     if tile.scout_status != ContentsRevealed {
					     *any_changed = true;
					     tile.scout_status = ContentsRevealed;
				     }

				     if steps > 0
					     && !matches!(tile.contents, TileContents::Obstacle)
					     && !already_checked.contains(&axial) {
					     already_checked.insert(*pos);
					     reveal_adjacents(*pos, map, steps - 1, already_checked, any_changed);
				     }
			     });
		}
	}

	pub fn input_clear_mist(&mut self) -> Vec<InputResult> {
		const COST_CLEAR_MIST: u8 = 3;

		if (self.nema_exhaustion.get() + COST_CLEAR_MIST) > 100 {
			return Vec::new();
		}

		let player_pos = self.party_pos;

		if let state @ (PartyState::InCombat(_) | PartyState::InEvent(_)) = &self.party_state {
			godot_warn!(
					"LocalMapState::input_clear_mist(): Warning: Received clear mist input while in combat or event!\
					state: {state:?}\
					pos: {player_pos:?}");
			return Vec::new();
		}


		let mut any_changed = false;
		clear_mist(player_pos, &mut self.tiles, 2, &mut HashSet::new(), &mut any_changed);

		return if any_changed {
			self.nema_exhaustion += COST_CLEAR_MIST;
			let mut results = vec![InputResult::Animation_ClearMist];
			results.extend(self.pass_time(Self::DEFAULT_TURN_TIME));
			results
		} else {
			Vec::new()
		};

		fn clear_mist(axial: Axial, map: &mut HashMap<Axial, Tile>, steps: usize, already_checked: &mut HashSet<Axial>, any_changed: &mut bool) {
			axial.neighbors().iter()
			     .for_each(|(_, pos)| {
				     let Some(tile) = map.get_mut(pos)
					     else { return; };
				     
				     match &mut tile.mist_status {
					     TileMistStatus::Mist_Soft {..}
					     | TileMistStatus::Mist_Hard => {
						     *any_changed = true;
						     tile.mist_status = TileMistStatus::NoMist_Temporary { turns_until_soft: tile::DEFAULT_TURNS_UNTIL_SOFT };
					     }
					     TileMistStatus::NoMist_Temporary { turns_until_soft } => {
						     *any_changed = true;
						     *turns_until_soft = u8::max(*turns_until_soft, tile::DEFAULT_TURNS_UNTIL_SOFT);
					     }
					     TileMistStatus::NoMist_Permanent => {}
				     }

				     if steps > 0
					     && !matches!(tile.contents, TileContents::Obstacle)
					     && !already_checked.contains(&axial) {
					     already_checked.insert(*pos);
					     clear_mist(*pos, map, steps - 1, already_checked, any_changed);
				     }
			     });
		}
	}

	pub fn input_short_rest(&mut self) -> Vec<InputResult> {
		const RESTORE_SHORT_REST: u8 = 2;

		if (u8::checked_sub(self.ethel_exhaustion.get(), RESTORE_SHORT_REST).is_none())
			|| (u8::checked_sub(self.nema_exhaustion.get(), RESTORE_SHORT_REST).is_none()) {
			return Vec::new();
		}

		if let state @ (PartyState::InCombat(_) | PartyState::InEvent(_)) = &self.party_state {
			godot_warn!(
					"LocalMapState::input_short_rest(): Warning: Received short rest input while in combat or event!\
					state: {state:?}");
			return Vec::new();
		}

		self.ethel_exhaustion -= RESTORE_SHORT_REST;
		self.nema_exhaustion -= RESTORE_SHORT_REST;

		let mut results = vec![InputResult::Animation_ShortRest];
		results.extend(self.pass_time(Self::DEFAULT_TURN_TIME));
		return results;
	}

	fn pass_time(&mut self, _amount: u8) -> Vec<InputResult> {
		//todo! make enemies move around
		//todo! maybe make mist move around too
		todo!();
	}
}

pub(super) enum InputResult {
	Animation_PassTurn,
	Animation_ClearMist,
	Animation_Scout,
	Animation_Walk(Axial),
	Animation_Run(Axial),
	Animation_ShortRest,
	Event(EventID),
	Combat(EnemyGroup),
	LongRest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PartyState {
	Idle,
	InCombat(Box<CombatState>), // box because combat state is big
	InEvent(EventID),
}