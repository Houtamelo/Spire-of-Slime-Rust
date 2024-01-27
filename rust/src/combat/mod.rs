mod effects;
mod skill_types;
mod timeline;
mod skill_resolving;
mod perk;
pub mod entity;

use std::collections::{HashMap, HashSet};
use comfy_bounded_ints::prelude::{Bound_u8, SqueezeTo, SqueezeTo_u64};
use gdnative::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use entity::position::Position;
use crate::combat::entity::*;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::girl::*;
use crate::combat::entity::stat::{CurrentStamina, Speed};
use crate::combat::timeline::TimelineEvent;
use crate::util::{SaturatedU64, ToSaturatedI64};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatState {
	entities: HashMap<Uuid, Entity>,
	seed: Xoshiro256PlusPlus,
	elapsed_ms: SaturatedU64,
}

impl CombatState {
	pub fn run(&mut self) {
		let events = self.get_timeline_events();
		if events.len() > 0 {
			let next_event = &events[0];
			self.tick(next_event.time_frame_ms); 
		}
	}

	fn tick(&mut self, delta_time_ms: SaturatedU64) {
		self.elapsed_ms += delta_time_ms;
		
		let guids_to_tick: HashSet<Uuid> = self.entities.values()
			.map(|entity| entity.guid())
			.collect();
		
		guids_to_tick.iter().for_each(|guid| {
			match self.entities.remove(guid) {
				Some(Entity::Character(character)) => { 
					PersistentEffect::tick_all(character, &mut self.entities, &mut self.seed, delta_time_ms);
				},
				Some(entity) => { 
					self.entities.insert(*guid, entity);
				},
				None => {},
			}
		});

		guids_to_tick.into_iter().for_each(|guid| {
			match self.entities.remove(&guid) {
				Some(Entity::Character(character)) => { 
					tick_character(character, &mut self.entities, delta_time_ms);
				},
				Some(entity) => { 
					self.entities.insert(guid, entity);
				},
				None => {
					godot_warn!("Warning: Trying to tick character with guid {guid:?}, but it was not found in the left or right entities!");
				}
			}
		});
		
		return;
		
		fn tick_character(mut character: CombatCharacter, others: &mut HashMap<Uuid, Entity>, delta_time_ms: SaturatedU64) {
			let charge_ms = CharacterState::spd_charge_ms(
				delta_time_ms, character.dyn_stat::<Speed>());
			let recovery_ms = CharacterState::spd_recovery_ms(
				delta_time_ms, character.dyn_stat::<Speed>());
			
			match &mut character.state {
				CharacterState::Idle => {
					// todo! run AI here
					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Grappling(..) => {
					let CharacterState::Grappling(grapplindetached_state) = std::mem::replace(&mut character.state, CharacterState::Idle)
						else { unreachable!(); };
					
					tick_grappled_girl(character, grapplindetached_state, others, delta_time_ms);
				}
				CharacterState::Downed { ticks } => {
					ticks.remaining_ms -= delta_time_ms;
					
					if ticks.remaining_ms.get() <= 0 {
						character.state = CharacterState::Idle;
						
						let stamina = {
							let mut temp = character.max_stamina().to_sat_i64();
							temp *= 5;
							temp /= 10;
							CurrentStamina::new(temp.squeeze_to())
						};
						
						if character.stamina_cur.get() < stamina.get() {
							character.stamina_cur = stamina;
						}
					}

					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Stunned { ticks, state_before_stunned } => {
					ticks.remaining_ms -= delta_time_ms;
					
					if ticks.remaining_ms.get() <= 0 {
						character.state = match std::mem::replace(state_before_stunned, StateBeforeStunned::Idle) {
							StateBeforeStunned::Recovering { ticks } =>
								{ CharacterState::Recovering { ticks } },
							StateBeforeStunned::Charging { skill_intention } => 
								{ CharacterState::Charging { skill_intention } },
							StateBeforeStunned::Idle =>
								{ CharacterState::Idle },
						}
					}

					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Charging { skill_intention } => {
					skill_intention.charge_ticks.remaining_ms -= charge_ms;
					
					if skill_intention.charge_ticks.remaining_ms.get() <= 0 {
						//todo! Cast skill
						character.state = match skill_intention.recovery_after_complete {
							Some(ticks) => { CharacterState::Recovering { ticks } }
							None => { CharacterState::Idle }
						}
					}

					others.insert(character.guid, Entity::Character(character));
				},
				CharacterState::Recovering { ticks } => {
					ticks.remaining_ms -= recovery_ms;
					
					if ticks.remaining_ms.get() <= 0 {
						character.state = CharacterState::Idle;
						//todo! run AI here
					}

					others.insert(character.guid, Entity::Character(character));
				}
			}
		}
		
		fn tick_grappled_girl(mut grappler: CombatCharacter, mut detached_state: GrapplingState, 
		                      others: &mut HashMap<Uuid, Entity>, delta_time_ms: SaturatedU64) {
			const INTERVAL_MS: u64 = 1000;
			
			match detached_state.victim {
				GrappledGirlEnum::Alive(mut girl_grappled) => {
					if detached_state.duration_ms <= delta_time_ms { // duration is over so time to cum!
						let remaining_intervals = (detached_state.accumulated_ms.get() + detached_state.duration_ms.get()) / INTERVAL_MS;
						*girl_grappled.lust += remaining_intervals * detached_state.lust_per_interval.get().squeeze_to_u64();
						*girl_grappled.temptation += remaining_intervals * detached_state.temptation_per_interval.get().squeeze_to_u64();
						
						let mut girl_released = girl_grappled.into_non_grappled();
						grappler.state = CharacterState::Idle;

						let girl_size = girl_released.position.size();
						let girl_position = match grappler.position {
							Position::Left  { .. } => { Position::Right { order: Bound_u8::new(0), size: girl_size } }
							Position::Right { .. } => { Position::Left  { order: Bound_u8::new(0), size: girl_size } }
						};

						// shift all allies of the released girl to the edge, to make space for her at the front
						iter_mut_allies_of!(girl_released, others).for_each(|ally|
							*ally.position_mut().order_mut() += girl_size);

						girl_released.position = girl_position;
						others.insert(girl_released.guid, Entity::Character(girl_released));
						others.insert(grappler.guid, Entity::Character(grappler));
						return;
					}
					
					detached_state.accumulated_ms += delta_time_ms;
					
					if detached_state.accumulated_ms.get() < INTERVAL_MS {
						detached_state.victim = GrappledGirlEnum::Alive(girl_grappled);
						grappler.state = CharacterState::Grappling(detached_state);
						others.insert(grappler.guid, Entity::Character(grappler));
						return;
					}

					let interval_count = detached_state.accumulated_ms.get() / INTERVAL_MS;
					detached_state.accumulated_ms -= interval_count * INTERVAL_MS;
					
					*girl_grappled.lust += interval_count * detached_state.lust_per_interval.get().squeeze_to_u64();
					*girl_grappled.temptation += interval_count * detached_state.temptation_per_interval.get().squeeze_to_u64();
					
					if girl_grappled.lust.get() < MAX_LUST {
						detached_state.victim = GrappledGirlEnum::Alive(girl_grappled);
						grappler.state = CharacterState::Grappling(detached_state);
						others.insert(grappler.guid, Entity::Character(grappler));
					} else {
						girl_grappled.lust.set(0);
						let orgasm_count = girl_grappled.orgasm_count.get();
						girl_grappled.orgasm_count.set(u8::clamp(orgasm_count + 1, 0, girl_grappled.orgasm_limit.get()));
						*girl_grappled.temptation -= 40;

						if girl_grappled.orgasm_count.get() >= girl_grappled.orgasm_limit.get() {
							let girl_defeated = girl_grappled.into_defeated();
							detached_state.victim = girl_defeated;
							grappler.state = CharacterState::Grappling(detached_state);
						} else {
							detached_state.victim = GrappledGirlEnum::Alive(girl_grappled);
							grappler.state = CharacterState::Grappling(detached_state);
						}

						others.insert(grappler.guid, Entity::Character(grappler));
					}
				}
				GrappledGirlEnum::Defeated(mut girl_defeated) => {
					detached_state.accumulated_ms += delta_time_ms;
					
					if detached_state.accumulated_ms.get() >= INTERVAL_MS {
						let interval_count = detached_state.accumulated_ms.get() / INTERVAL_MS;
						detached_state.accumulated_ms -= interval_count * INTERVAL_MS;
						*girl_defeated.lust += interval_count * detached_state.lust_per_interval.get().squeeze_to_u64();
						*girl_defeated.temptation += interval_count * detached_state.temptation_per_interval.get().squeeze_to_u64();
					}
					
					if girl_defeated.lust.get() >= MAX_LUST {
						girl_defeated.lust.set(0);
						let orgasm_count = girl_defeated.orgasm_count.get();
						girl_defeated.orgasm_count.set(u8::clamp(orgasm_count + 1, 0, girl_defeated.orgasm_limit.get()));
						*girl_defeated.temptation -= 40;
					}
					
					detached_state.victim = GrappledGirlEnum::Defeated(girl_defeated);
					grappler.state = CharacterState::Grappling(detached_state);
					others.insert(grappler.guid, Entity::Character(grappler));
				}
			}
		}
	}
	
	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		let mut events= self.entities.values()
			.filter_map(|entity| if let Entity::Character(character) = entity { Some(character) } else { None })
			.flat_map(|character| TimelineEvent::generate_events(character))
			.collect::<Vec<_>>();
		
		events.sort_by(|a, b| a.time_frame_ms.cmp(&b.time_frame_ms));
		return events;
	}
}