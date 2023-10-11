use std::collections::{HashMap, HashSet};
use crate::util::bounded_integer_traits_ISize::*;
use gdnative::prelude::*;
use rand::prelude::StdRng;
use entity::position::Position;
use crate::combat::entity::*;
use crate::{CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, iter_mut_allies_of, STANDARD_INTERVAL_MS};
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::girl::*;
use crate::combat::ModifiableStat::SPD;
use crate::combat::timeline::TimelineEvent;
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
		let events = self.get_timeline_events();
		if events.len() > 0 {
			let next_event = &events[0];
			self.tick(next_event.time_frame_ms); 
		}
	}

	fn tick(&mut self, delta_time_ms: i64) {
		self.elapsed_ms += delta_time_ms;
		
		let guids_to_tick : HashSet<GUID> = self.entities.values().map(|entity| entity.guid()).collect();

		for guid in guids_to_tick.iter() {
			if let Some(Entity::Character(character)) = self.entities.remove(guid) {
				PersistentEffect::tick_all(character, &mut self.entities, delta_time_ms);
			}
		}

		for guid in guids_to_tick.iter() {
			if let Some(Entity::Character(character)) = self.entities.remove(&guid) {
				tick_character(character, &mut self.entities, delta_time_ms);
			} else {
				godot_warn!("Warning: Trying to tick character with guid {guid:?}, but it was not found in the left or right entities!");
			}
		}
		
		return;
		
		fn tick_character(mut character: CombatCharacter, others: &mut HashMap<GUID, Entity>, delta_time_ms: i64) {
			match character.state {
				CharacterState::Idle => {
					// todo! run AI here
					character.state = CharacterState::Idle;
					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Grappling(grappling) => {
					character.state = CharacterState::Grappling(grappling);
					tick_grappled_girl(character, others, delta_time_ms);
				}
				CharacterState::Downed { mut ticks } => {
					ticks.remaining_ms -= delta_time_ms;
					if ticks.remaining_ms <= 0 {
						character.state = CharacterState::Idle;
						character.stamina_cur = isize::max(character.stamina_cur, (character.stamina_max * 5) / 10)
					} else {
						character.state = CharacterState::Downed { ticks };
					}

					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Stunned { mut ticks, state_before_stunned } => {
					ticks.remaining_ms -= delta_time_ms;
					if ticks.remaining_ms <= 0 {
						character.state = match state_before_stunned {
							StateBeforeStunned::Recovering { ticks }             => { CharacterState::Recovering { ticks } }
							StateBeforeStunned::Charging   { skill_intention } => { CharacterState::Charging { skill_intention } }
							StateBeforeStunned::Idle                                           => { CharacterState::Idle }
						}
					} else {
						character.state = CharacterState::Stunned { ticks, state_before_stunned }
					}

					others.insert(character.guid, Entity::Character(character));
				}
				CharacterState::Charging { skill_intention } => {
					character.state = CharacterState::Charging { skill_intention }; // move it back to calculate SPD on next line
					let spd_delta_time_ms = CharacterState::spd_charge_ms(delta_time_ms, character.stat(SPD));
					let CharacterState::Charging { mut skill_intention} = character.state else { panic!() };

					skill_intention.charge_ticks.remaining_ms -= spd_delta_time_ms;
					if skill_intention.charge_ticks.remaining_ms <= 0 {
						//todo! Cast skill
						character.state = match skill_intention.recovery_after_complete {
							Some(ticks) => { CharacterState::Recovering { ticks } }
							None => { CharacterState::Idle }
						}
					} else {
						character.state = CharacterState::Charging { skill_intention };
					}

					others.insert(character.guid, Entity::Character(character));
				},
				CharacterState::Recovering { ticks } => {
					character.state = CharacterState::Recovering { ticks }; // move it back to calculate SPD on next line
					let spd_delta_time_ms = CharacterState::spd_recovery_ms(delta_time_ms, character.stat(SPD));
					let CharacterState::Recovering { mut ticks } = character.state else { panic!() };

					ticks.remaining_ms -= spd_delta_time_ms;
					if ticks.remaining_ms <= 0 {
						character.state = CharacterState::Idle;
						//todo! run AI here
					}
					else {
						character.state = CharacterState::Recovering { ticks };
					}

					others.insert(character.guid, Entity::Character(character));
				}
			}
		}
		
		fn tick_grappled_girl(mut grappler: CombatCharacter, others: &mut HashMap<GUID, Entity>, delta_time_ms: i64) {
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
						for ally_of_girl in iter_mut_allies_of!(girl_released, others) {
							let order_mut = ally_of_girl.position_mut().order_mut();
							*order_mut += girl_size;
						}

						girl_released.position = girl_position;
						others.insert(girl_released.guid, Entity::Character(girl_released));
						others.insert(grappler     .guid, Entity::Character(grappler));
						return;
					}
					
					g_state.accumulated_ms += delta_time_ms;

					// early return
					if g_state.accumulated_ms < STANDARD_INTERVAL_MS {
						g_state.victim = GrappledGirl::Alive(girl_alive);
						grappler.state = CharacterState::Grappling(g_state);
						others.insert(grappler.guid, Entity::Character(grappler));
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
						others.insert(grappler.guid, Entity::Character(grappler));
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

					others.insert(grappler.guid, Entity::Character(grappler));
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
					others.insert(grappler.guid, Entity::Character(grappler));
				}
			}
		}
	}
	
	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		let mut all_events: Vec<TimelineEvent> = Vec::new();
		for entity in self.entities.values() {
			if let Entity::Character(character) = entity {
				TimelineEvent::register_character(character, &mut all_events)
			}
		}
		
		all_events.sort_by(|a, b| a.time_frame_ms.cmp(&b.time_frame_ms));
		return all_events;
	}
}