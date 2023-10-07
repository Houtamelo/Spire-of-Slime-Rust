use std::iter::{FilterMap, Map};
use std::ops::IndexMut;
use std::slice::{Iter, IterMut};
use gdnative::{godot_error, godot_print};
use rand::prelude::StdRng;
use crate::combat::entity::{CombatCharacter, CharacterState, Position, MAX_LUST, GrappledGirl, Entity, StateBeforeStunned};
use crate::combat::timeline::{TimelineEvent};
use crate::{CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, STANDARD_INTERVAL_MS};
use crate::combat::ModifiableStat::SPD;
use crate::combat::skills::{DefensiveSkill, PositionMatrix, SkillType};
use crate::util::{Base100ChanceGenerator, TrackedTicks};

mod effects;
mod skills;
mod timeline;
mod entity;
mod skill_resolving;

include!("stat.rs");

pub struct CombatState {
	entities: Vec<Entity>,
	seed: StdRng,
	elapsed_ms: i64,
}

impl CombatState {
	pub fn run(&mut self) {
		let events = self.get_timeline_events();
		if events.len() > 0 {
			let next_event = &events[0];
			self.tick(next_event.time_frame_ms);
		}
	}

	fn tick(&mut self, delta_time_ms: i64) {
		self.elapsed_ms += delta_time_ms;
		
		let mut guids_to_tick : Vec<usize>  = Vec::new();
		let mut left_entities : Vec<Entity> = Vec::new();
		let mut right_entities: Vec<Entity> = Vec::new();

		while let Some(entity) = self.entities.pop() {
			if let Entity::Character(character) = &entity {
				guids_to_tick.push(character.guid);
			}
			
			match entity.position() {
				Position::Left  { .. } => { left_entities .push(entity); }
				Position::Right { .. } => { right_entities.push(entity); }
			}
		}
		
		while let Some(guid) = guids_to_tick.pop() {
			if let Some(position) = left_entities.iter().position(
				|entity| match entity {
					Entity::Character(character) => { character.guid == guid }
					_ => { false } })
			{
				let mut entity = left_entities.remove(position);
				let Entity::Character(character) = &mut entity else { panic!() };
				tick_character(character, &mut left_entities, &mut right_entities, delta_time_ms); 
				left_entities.push(entity);
				continue;
			}
			
			if let Some(position) = right_entities.iter().position(
				|entity| match entity {
					Entity::Character(character) => { character.guid == guid }
					_ => { false } }) 
			{
				let mut entity = right_entities.remove(position);
				let Entity::Character(character) = &mut entity else { panic!() };
				tick_character(character, &mut right_entities, &mut left_entities, delta_time_ms); 
				right_entities.push(entity);
				continue;
			}
			
			godot_error!("Warning: Trying to tick character with guid {guid:?}, but it was not found in the left or right entities!");
		}
		
		for entity in left_entities  { self.entities.push(entity); }
		for entity in right_entities { self.entities.push(entity); }
		
		return;
		
		fn tick_character(ticked_character: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, delta_time_ms: i64) {
			let state = &mut ticked_character.state;
			match state {
				CharacterState::Idle => {
					// todo! run AI here
				}
				CharacterState::Grappling { .. } => {
					tick_grappled_girl(state, &ticked_character.position, delta_time_ms, enemies);
				}
				CharacterState::Downed { ticks } => {
					ticks.remaining_ms -= delta_time_ms;
					if ticks.remaining_ms <= 0 {
						*state = CharacterState::Idle;
						ticked_character.stamina_cur = isize::max(ticked_character.stamina_cur, (ticked_character.stamina_max * 5) / 10)
					}
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
				}
				CharacterState::Charging { .. } => {
					let spd_delta_time_ms = CharacterState::spd_charge_ms(delta_time_ms, ticked_character.stat(SPD));

					// we cannot use skill_intention from the first match because we borrow ticked_character mutably to calculate it's SPD
					let CharacterState::Charging { skill_intention} = &mut ticked_character.state else { panic!() };

					skill_intention.charge_ticks.remaining_ms -= spd_delta_time_ms;
					if skill_intention.charge_ticks.remaining_ms <= 0 {
						//todo! Cast skill
						ticked_character.state = match skill_intention.recovery_after_complete {
							Some(ticks) => { CharacterState::Recovering { ticks } }
							None => { CharacterState::Idle }
						}
					}
				},
				CharacterState::Recovering { ticks } => {
					let mut ticks_clone = ticks.clone(); // clone is needed because we need to pass ticked_character as immutable to calculate it's SPD
					let spd_delta_time_ms = CharacterState::spd_recovery_ms(delta_time_ms, ticked_character.stat(SPD));
					ticks_clone.remaining_ms -= spd_delta_time_ms;
					if ticks_clone.remaining_ms <= 0 {
						ticked_character.state = CharacterState::Idle;
						//todo! run AI here
					}
					else {
						ticked_character.state = CharacterState::Recovering { ticks: ticks_clone };
					}
				}
			}
		}
		
		fn tick_grappled_girl(state: &mut CharacterState, ticked_position: &Position, delta_time_ms: i64, enemies: &mut Vec<Entity>) {
			let CharacterState::Grappling {victim, lust_per_sec, temptation_per_sec, duration_ms, accumulated_ms} = state else { panic!() };

			match victim {
				GrappledGirl::Alive(girl_alive) => {
					if *duration_ms > delta_time_ms {
						*accumulated_ms += delta_time_ms;
						
						if *accumulated_ms >= STANDARD_INTERVAL_MS {
							let interval_count = *accumulated_ms / STANDARD_INTERVAL_MS;
							*accumulated_ms -= interval_count * STANDARD_INTERVAL_MS;
							let seconds = (interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as isize;
							girl_alive.lust += seconds * (*lust_per_sec as isize);
							girl_alive.temptation += seconds * (*temptation_per_sec as isize);
						}

						if girl_alive.lust >= MAX_LUST {
							const temptation_delta_on_orgasm: isize = -40;
							girl_alive.lust = 0;
							girl_alive.orgasm_count = isize::clamp(girl_alive.orgasm_count + 1, 0, girl_alive.orgasm_limit);
							girl_alive.temptation   = isize::clamp(girl_alive.temptation + temptation_delta_on_orgasm, 0, 100);

							if girl_alive.orgasm_count == girl_alive.orgasm_limit {
								*victim = girl_alive.to_defeated();
							}
						}
					}
					else {
						let seconds = ((*accumulated_ms + *duration_ms) / 1000) as isize;
						girl_alive.lust += seconds * (*lust_per_sec as isize);
						girl_alive.temptation += seconds * (*temptation_per_sec as isize);
						*accumulated_ms = 0;
						
						let mut girl_released: CombatCharacter = girl_alive.to_non_grappled();
						*state = CharacterState::Idle;

						let girl_size = girl_released.size;
						let girl_position = match ticked_position {
							Position::Left  { .. } => { Position::Right { order: 0, size: girl_size } }
							Position::Right { .. } => { Position::Left  { order: 0, size: girl_size } }
						};

						// shift all allies of the released girl to the edge, to make space for her at the front
						for girl_ally in enemies.iter_mut() {
							match &mut girl_ally.position_mut() {
								Position::Left  { order, .. } => { *order += girl_size; }
								Position::Right { order, .. } => { *order += girl_size; }
							}
						}

						girl_released.position = girl_position;
						enemies.push(Entity::Character(girl_released));
					}
				}
				GrappledGirl::Defeated(girl_defeated) => {
					*accumulated_ms += delta_time_ms;
					
					if *accumulated_ms >= STANDARD_INTERVAL_MS {
						let interval_count = *accumulated_ms / STANDARD_INTERVAL_MS;
						*accumulated_ms -= interval_count * STANDARD_INTERVAL_MS;
						let seconds = (interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as isize;
						girl_defeated.lust += seconds * (*lust_per_sec as isize);
						girl_defeated.temptation += seconds * (*temptation_per_sec as isize);
					}

					if girl_defeated.lust >= MAX_LUST {
						const temptation_delta_on_orgasm: isize = -40;
						girl_defeated.lust = 0;
						girl_defeated.orgasm_count = isize::clamp(girl_defeated.orgasm_count + 1, 0, girl_defeated.orgasm_limit);
						girl_defeated.temptation = isize::clamp(girl_defeated.temptation + temptation_delta_on_orgasm, 0, 100);
					}
				}
			}
		}
	}
	
	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
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
	}
}