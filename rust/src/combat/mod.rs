use std::collections::HashMap;
use crate::util::bounded_integer_traits::*;
use gdnative::godot_error;
use gdnative::prelude::godot_warn;
use rand::prelude::StdRng;
use entity::position::Position;
use crate::combat::entity::*;
use crate::{CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, STANDARD_INTERVAL_MS};
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::girl::*;
use crate::combat::ModifiableStat::SPD;
use crate::util::GUID;

mod effects;
mod skills;
mod timeline;
mod entity;
mod skill_resolving;

include!("stat.rs");

pub struct CombatState {
	entities: HashMap<GUID, Entity>,
	seed: StdRng,
	elapsed_ms: i64,
}

impl CombatState {
	pub fn run(&mut self) {
		//let events = self.get_timeline_events();
		//if events.len() > 0 {
		//	let next_event = &events[0];
	//		self.tick(next_event.time_frame_ms);
	//	}todo!
	}

	fn tick(&mut self, delta_time_ms: i64) {
		self.elapsed_ms += delta_time_ms;
		
		let mut guids_to_tick : Vec<GUID>  = Vec::new();
		let mut left_entities : HashMap<GUID, Entity> = HashMap::new();
		let mut right_entities: HashMap<GUID, Entity> = HashMap::new();

		for (guid, entity) in self.entities.drain() {
			match (*entity.position(), entity) {
				(Position::Left { ..}, Entity::Character(character)) => {
					guids_to_tick.push(character.guid);
					PersistentEffect::tick_all(character, &mut  left_entities, delta_time_ms);
				}
				(Position::Right { .. }, Entity::Character(character)) => {
					guids_to_tick.push(character.guid);
					PersistentEffect::tick_all(character, &mut right_entities, delta_time_ms);
				}
				(Position::Left  { .. }, entity)  => {  left_entities.insert(guid, entity); }
				(Position::Right { .. }, entity) => { right_entities.insert(guid, entity); }
			}
		}
		
		while let Some(guid) = guids_to_tick.pop() {
			if let Some(left_entity) = left_entities.remove(&guid) {
				if let Entity::Character(character) = left_entity {
					tick_character(character, &mut left_entities, &mut right_entities, delta_time_ms);
				} else { 
					left_entities.insert(guid, left_entity);
				}
			} 
			else if let Some(right_entity) = right_entities.remove(&guid) {
				if let Entity::Character(character) = right_entity {
					tick_character(character, &mut left_entities, &mut right_entities, delta_time_ms);
				} else { 
					right_entities.insert(guid, right_entity);
				}
			} else {
				godot_warn!("Warning: Trying to tick character with guid {guid:?}, but it was not found in the left or right entities!");
			}
		}
		
		for entity in left_entities  { self.entities.insert(entity.0, entity.1); }
		for entity in right_entities { self.entities.insert(entity.0, entity.1); }
		
		return;
		
		fn tick_character(mut character: CombatCharacter, allies: &mut HashMap<GUID, Entity>, enemies: &mut HashMap<GUID, Entity>, delta_time_ms: i64) {
			let state = &mut character.state;
			match state {
				CharacterState::Idle => {
					// todo! run AI here
					allies.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Grappling(_) => {
					tick_grappled_girl(character, allies, enemies, delta_time_ms);
				}
				CharacterState::Downed { ticks } => {
					ticks.remaining_ms -= delta_time_ms;
					if ticks.remaining_ms <= 0 {
						*state = CharacterState::Idle;
						character.stamina_cur = isize::max(character.stamina_cur, (character.stamina_max * 5) / 10)
					}
					
					allies.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Stunned { ticks, state_before_stunned } => {
					ticks.remaining_ms -= delta_time_ms;
					if ticks.remaining_ms <= 0 {
						*state = match state_before_stunned {
							StateBeforeStunned::Recovering { ticks } => { CharacterState::Recovering { ticks: ticks.to_owned() } }
							StateBeforeStunned::Charging   { skill_intention } => { CharacterState::Charging { skill_intention: skill_intention.to_owned() } }
							StateBeforeStunned::Idle => { CharacterState::Idle }
						}
					}

					allies.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Charging { .. } => {
					let spd_delta_time_ms = CharacterState::spd_charge_ms(delta_time_ms, character.stat(SPD));

					// we cannot use skill_intention from the first match because we borrow ticked_character mutably to calculate it's SPD
					let CharacterState::Charging { skill_intention} = &mut character.state else { panic!() };

					skill_intention.charge_ticks.remaining_ms -= spd_delta_time_ms;
					if skill_intention.charge_ticks.remaining_ms <= 0 {
						//todo! Cast skill
						character.state = match skill_intention.recovery_after_complete {
							Some(ticks) => { CharacterState::Recovering { ticks } }
							None => { CharacterState::Idle }
						}
					} else {
						allies.insert(character.guid, Entity::Character(character));
					}
				},
				CharacterState::Recovering { ticks } => {
					let mut ticks_clone = ticks.clone(); // clone is needed because we need to pass ticked_character as immutable to calculate it's SPD
					let spd_delta_time_ms = CharacterState::spd_recovery_ms(delta_time_ms, character.stat(SPD));
					ticks_clone.remaining_ms -= spd_delta_time_ms;
					if ticks_clone.remaining_ms <= 0 {
						character.state = CharacterState::Idle;
						//todo! run AI here
					}
					else {
						character.state = CharacterState::Recovering { ticks: ticks_clone };
					}

					allies.insert(character.guid, Entity::Character(character));
				}
			}
		}
		
		fn tick_grappled_girl(mut grappler: CombatCharacter, allies: &mut HashMap<GUID, Entity>, enemies: &mut HashMap<GUID, Entity>, delta_time_ms: i64) {
			let CharacterState::Grappling(mut g_state) = grappler.state else { panic!() };
			
			match g_state.victim {
				GrappledGirl::Alive(mut girl_alive) => {
					if g_state.duration_ms <= delta_time_ms { // duration is over so time to cum!
						let seconds = ((g_state.accumulated_ms + g_state.duration_ms) / 1000) as isize;
						girl_alive.lust += seconds * (g_state.lust_per_sec as isize);
						girl_alive.temptation += seconds * (g_state.temptation_per_sec as isize);
						
						g_state.accumulated_ms = 0;
						
						let mut girl_released: CombatCharacter = girl_alive.to_non_grappled();
						grappler.state = CharacterState::Idle;

						let girl_size = *girl_released.position.size();
						let girl_position = match grappler.position {
							Position::Left  { .. } => { Position::Right { order: 0, size: girl_size } }
							Position::Right { .. } => { Position::Left  { order: 0, size: girl_size } }
						};

						// shift all allies of the released girl to the edge, to make space for her at the front
						for ally_of_girl in enemies.values_mut() {
							match &mut ally_of_girl.position_mut() {
								Position::Left  { order, .. } => { *order += girl_size; }
								Position::Right { order, .. } => { *order += girl_size; }
							}
						}

						girl_released.position = girl_position;
						enemies.insert(girl_released.guid, Entity::Character(girl_released));
						allies.insert(grappler.guid, Entity::Character(grappler));
						return;
					}
					
					g_state.accumulated_ms += delta_time_ms;

					// early return
					if g_state.accumulated_ms < STANDARD_INTERVAL_MS {
						g_state.victim = GrappledGirl::Alive(girl_alive);
						grappler.state = CharacterState::Grappling(g_state);
						allies.insert(grappler.guid, Entity::Character(grappler));
						return;
					}

					let interval_count = g_state.accumulated_ms / STANDARD_INTERVAL_MS;
					g_state.accumulated_ms -= interval_count * STANDARD_INTERVAL_MS;
					let seconds = (interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as isize;
					girl_alive.lust += seconds * (g_state.lust_per_sec as isize);
					girl_alive.temptation += seconds * (g_state.temptation_per_sec as isize);

					// early return
					if girl_alive.lust < MAX_LUST {
						g_state.victim = GrappledGirl::Alive(girl_alive);
						grappler.state = CharacterState::Grappling(g_state);
						allies.insert(grappler.guid, Entity::Character(grappler));
						return;
					}

					girl_alive.lust = 0.bind_0_p200();
					girl_alive.orgasm_count = isize::clamp(girl_alive.orgasm_count + 1, 0, girl_alive.orgasm_limit);
					girl_alive.temptation = isize::clamp(girl_alive.temptation.get() - 40, 0, 100).bind_0_p100();

					if girl_alive.orgasm_count == girl_alive.orgasm_limit {
						if let Some(defeated_girl) = girl_alive.to_defeated() {
							g_state.victim = defeated_girl;
							grappler.state = CharacterState::Grappling(g_state);
						} else { // if None then the girl should vanish and the monster get back to idle
							grappler.state = CharacterState::Idle;
						}
					} else {
						g_state.victim = GrappledGirl::Alive(girl_alive);
						grappler.state = CharacterState::Grappling(g_state);
					}

					allies.insert(grappler.guid, Entity::Character(grappler));
				}
				GrappledGirl::Defeated(mut girl_defeated) => {
					g_state.accumulated_ms += delta_time_ms;
					
					if g_state.accumulated_ms >= STANDARD_INTERVAL_MS {
						let interval_count = g_state.accumulated_ms / STANDARD_INTERVAL_MS;
						g_state.accumulated_ms -= interval_count * STANDARD_INTERVAL_MS;
						let seconds = (interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as isize;
						girl_defeated.lust += seconds * (g_state.lust_per_sec as isize);
						girl_defeated.temptation += seconds * (g_state.temptation_per_sec as isize);
					}
					
					if girl_defeated.lust >= MAX_LUST {
						const temptation_delta_on_orgasm: isize = -40;
						girl_defeated.lust = 0.bind_0_p200();
						girl_defeated.orgasm_count = isize::clamp(girl_defeated.orgasm_count + 1, 0, girl_defeated.orgasm_limit);
						girl_defeated.temptation = isize::clamp(girl_defeated.temptation.get() + temptation_delta_on_orgasm, 0, 100).bind_0_p100();
					}
					
					g_state.victim = GrappledGirl::Defeated(girl_defeated);
					grappler.state = CharacterState::Grappling(g_state);
					allies.insert(grappler.guid, Entity::Character(grappler));
				}
			}
		}
	}
	
	//todo!
	/*fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		let mut all_events: Vec<TimelineEvent> = Vec::new();
		self.all_characters().for_each(|character| TimelineEvent::register_character(character, &mut all_events));
		all_events.sort_by(|a, b| a.time_frame_ms.cmp(&b.time_frame_ms));
		return all_events;
	}

	pub fn left_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.entities.iter().filter_map(|entity| match entity {
			Entity::Character(character) => {
				if let Position::Left { .. } = character.position { Some(character) }
				else { None }
			}
			_ => { None }
		});
	}
		
	pub fn left_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.entities.iter_mut().filter_map(|entity| match entity {
			Entity::Character(character) => {
				if let Position::Left { .. } = character.position { Some(character) }
				else { None }
			}
			_ => { None }
		});
	}
	
	pub fn left_entities(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&Entity>> {
		return self.entities.iter()
		           .filter_map(|entity|
						if let Position::Left { .. } = entity.position() {
							Some(entity)
						} else { None });
	}

	pub fn left_entities_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut Entity>> {
		return self.entities.iter_mut().filter_map(|entity|
				if let Position::Left { .. } = entity.position() {
					Some(entity)
				} else { None });
	}
	
	pub fn right_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.entities.iter().filter_map(|entity| match entity {
			Entity::Character(character) => { 
				if let Position::Right { .. } = character.position { Some(character) }
				else { None }
			}
			_ => { None }
		});
	}
	
	pub fn right_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.entities.iter_mut().filter_map(|entity| match entity {
			Entity::Character(character) => { 
				if let Position::Right { .. } = character.position { Some(character) }
				else { None }
			}
			_ => { None }
		});
	}
	
	pub fn right_entities(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&Entity>> {
		return self.entities.iter().filter_map(|entity|
				if let Position::Right { .. } = entity.position() {
					Some(entity)
				} else { None });
	}
	
	pub fn right_entities_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut Entity>> {
		return self.entities.iter_mut().filter_map(|entity|
				if let Position::Right { .. } = entity.position() {
					Some(entity)
				} else { None });
	}
	
	pub fn all_characters(&self) -> FilterMap<Iter<Entity>, fn(&Entity) -> Option<&CombatCharacter>> {
		return self.entities.iter().filter_map(|entity| match entity {
			Entity::Character(character) => { Some(character) }
			_ => { None }
		});
	}
	
	pub fn all_characters_mut(&mut self) -> FilterMap<IterMut<Entity>, fn(&mut Entity) -> Option<&mut CombatCharacter>> {
		return self.entities.iter_mut().filter_map(|entity| match entity {
			Entity::Character(character) => { Some(character) }
			_ => { None }
		});
	}*/
}