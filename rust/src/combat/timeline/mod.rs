use std::num::{NonZeroU64, NonZeroU8};

use comfy_bounded_ints::prelude::{SqueezeTo, SqueezeTo_u64, SqueezeTo_u8};
use gdnative::log::godot_warn;
use uuid::Uuid;

use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::entity::stat::Speed;
use crate::misc::{SaturatedU64, ToSaturatedI64, ToSaturatedU64, TrackedTicks};

#[derive(Debug, Clone)]
pub struct TimelineEvent {
	pub time_frame_ms: SaturatedU64,
	pub event_type: EventType,
	pub character_guid: Uuid
}

impl TimelineEvent {
	pub fn generate_events(character: &CombatCharacter) -> Vec<TimelineEvent> {
		let mut events = Vec::new();
		
		let character_guid = character.guid;
		match &character.state {
			CharacterState::Idle => {
				let event = TimelineEvent { 
					time_frame_ms: 0.to_sat_u64(), 
					event_type: EventType::TurnBegin, 
					character_guid
				};
				events.push(event);
			}
			CharacterState::Grappling(GrapplingState { victim, lust_per_interval, temptation_per_interval,
				                          duration_ms, accumulated_ms }) => {
				const INTERVAL_MS: u64 = 1000;
				
				let event = TimelineEvent { 
					time_frame_ms: *duration_ms, 
					event_type: EventType::GrapplingEnd, 
					character_guid
				};
				events.push(event);
				
				let total_time_ms = {
					let mut temp = duration_ms.to_sat_i64();
					temp += accumulated_ms.get();
					temp.squeeze_to_u64()
				};
				
				let total_intervals_count = total_time_ms / INTERVAL_MS;

				if total_intervals_count < 1 {
					let lust_option = {
						let mut temp = lust_per_interval.get().to_sat_i64();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::LustTick { lust },
							character_guid: victim.guid()
						};

						events.push(event);
					});
					
					let temptation_option = {
						let mut temp = temptation_per_interval.get().to_sat_i64();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					temptation_option.map(|temptation| {
						let event = TimelineEvent { 
							time_frame_ms: *duration_ms, 
							event_type: EventType::TemptationTick { temptation }, 
							character_guid: victim.guid() 
						};
						
						events.push(event);
					});

					return events;
				}

				let mut current_ms = 0.to_sat_i64();

				if accumulated_ms.get() < INTERVAL_MS {
					current_ms *= -1;
					current_ms *= accumulated_ms.get();
				} else {
					let interval_count = accumulated_ms.get() / INTERVAL_MS;
					
					let lust_option = {
						let mut temp = lust_per_interval.get().to_sat_i64();
						temp *= interval_count;
						NonZeroU8::new(temp.squeeze_to())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent { 
							time_frame_ms: current_ms.to_sat_u64(), 
							event_type: EventType::LustTick { lust }, 
							character_guid: victim.guid()
						};
						
						events.push(event);
					});
					
					let temptation_option = {
						let mut temp = temptation_per_interval.get().to_sat_i64();
						temp *= interval_count;
						NonZeroU8::new(temp.squeeze_to())
					};

					temptation_option.map(|temptation| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(), 
							event_type: EventType::TemptationTick { temptation }, 
							character_guid: victim.guid()
						};
						
						events.push(event);
					});
					
					let mult = {
						let mut temp = accumulated_ms.to_sat_i64();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp.get()
					};
					current_ms *= mult;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;
					let event = TimelineEvent {
						time_frame_ms: current_ms.to_sat_u64(),
						event_type: EventType::LustTick { lust: *lust_per_interval },
						character_guid
					};

					events.push(event);
				}

				let remaining_ms = {
					let mut temp = total_time_ms.to_sat_i64();
					temp -= total_intervals_count * INTERVAL_MS;
					temp.squeeze_to_u64()
				};
				
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					
					let lust_option = {
						let mut temp = lust_per_interval.get().to_sat_i64();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(),
							event_type: EventType::LustTick { lust },
							character_guid,
						};
						
						events.push(event);
					});
					
					let temptation_option = {
						let mut temp = temptation_per_interval.get().to_sat_i64();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					temptation_option.map(|temptation| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(),
							event_type: EventType::TemptationTick { temptation },
							character_guid
						};
						
						events.push(event);
					});
				}
			} 
			CharacterState::Downed { ticks: TrackedTicks { remaining_ms, .. } } => {
				if remaining_ms.get() > 0 {
					let event = TimelineEvent {
						time_frame_ms: *remaining_ms,
						event_type: EventType::DownedEnd,
						character_guid
					};
					
					events.push(event);
				}
			}
			CharacterState::Stunned { ticks: stunned_ticks, state_before_stunned } => {
				if stunned_ticks.remaining_ms.get() > 0 {
					let event = TimelineEvent {
						time_frame_ms: stunned_ticks.remaining_ms.to_sat_u64(),
						event_type: EventType::StunEnd,
						character_guid
					};
					
					events.push(event);
				}
				
				match state_before_stunned {
					StateBeforeStunned::Recovering { ticks: recovering_ticks } => {
						if recovering_ticks.remaining_ms.get() > 0 {
							let time_frame_ms = {
								let mut temp = recovering_ticks.remaining_ms.to_sat_i64();
								temp += stunned_ticks.remaining_ms.get();
								temp.to_sat_u64()
							};
							
							let event = TimelineEvent {
								time_frame_ms,
								event_type: EventType::TurnBegin,
								character_guid
							};
							
							events.push(event);
						}
					},
					StateBeforeStunned::Charging { skill_intention } => {
						if skill_intention.charge_ticks.remaining_ms.get() > 0 {
							let time_frame_ms = {
								let mut temp = skill_intention.charge_ticks.remaining_ms.to_sat_i64();
								temp += stunned_ticks.remaining_ms.get();
								temp.to_sat_u64()
							};
							
							let event = TimelineEvent {
								time_frame_ms,
								event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() },
								character_guid
							};
							
							events.push(event);
						}
					}
					StateBeforeStunned::Idle => {
						let event = TimelineEvent {
							time_frame_ms: stunned_ticks.remaining_ms.to_sat_u64(),
							event_type: EventType::TurnBegin,
							character_guid
						};
						
						events.push(event);
					}
				}
			},
			CharacterState::Charging { skill_intention } => {
				let estimated_charge_ms = CharacterState::spd_charge_ms(
					skill_intention.charge_ticks.remaining_ms, character.dyn_stat::<Speed>());
				if estimated_charge_ms.get() > 0 {
					let event = TimelineEvent {
						time_frame_ms: estimated_charge_ms,
						event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() },
						character_guid
					};
					
					events.push(event);
				}
			}
			CharacterState::Recovering { ticks } => {
				let estimated_recovery_ms = CharacterState::spd_recovery_ms(
					ticks.remaining_ms, character.dyn_stat::<Speed>());
				if estimated_recovery_ms.get() > 0 {
					let event = TimelineEvent {
						time_frame_ms: estimated_recovery_ms,
						event_type: EventType::TurnBegin,
						character_guid
					};
					
					events.push(event);
				}
			}
		}

		character.persistent_effects.iter().for_each(|status| 
			Self::register_status(status, character, &mut events));
		
		return events;
	}
	
	fn register_status(status: &PersistentEffect, owner: &CombatCharacter, events: &mut Vec<TimelineEvent>) {
		let event_end_ms = status.duration();
		if event_end_ms.get() <= 0 { 
			godot_warn!("{}(): Trying to register an event from status with negative duration: {:?}, duration: {:?}", 
				util::full_fn_name(&Self::register_status), status, event_end_ms);
			return;
		}

		let event = TimelineEvent {
			time_frame_ms: event_end_ms,
			event_type: EventType::StatusEnd { effect_clone: status.clone() },
			character_guid: owner.guid
		};
		events.push(event);
		
		match status {
			| PersistentEffect::Buff{..} | PersistentEffect::Debuff{..}
			| PersistentEffect::Guarded{..} | PersistentEffect::Marked{..}
			| PersistentEffect::Riposte{..} | PersistentEffect::TemporaryPerk{..} => {}
			PersistentEffect::Poison { duration_ms, accumulated_ms,
				interval_ms, poison_per_interval, .. } => {
				
				let total_time_ms = {
					let mut temp = duration_ms.to_sat_i64();
					temp += accumulated_ms.get();
					temp.squeeze_to_u64()
				};
				
				let total_intervals_count = total_time_ms / interval_ms.get();
				
				if total_intervals_count < 1 {
					let poison_option = {
						let mut temp = poison_per_interval.get().to_sat_i64();
						temp *= total_time_ms;
						temp /= interval_ms.get();
						NonZeroU8::new(temp.squeeze_to_u8())
					};

					poison_option.map(|poison| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::PoisonTick { poison },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
					
					return;
				}
				
				let mut current_ms =
					if accumulated_ms.get() < interval_ms.get() {
						let mut temp = accumulated_ms.to_sat_i64();
						temp *= -1;
						temp
					} else {
						let interval_count = accumulated_ms.get() / interval_ms.get();
						
						let poison_option = {
							let mut temp = poison_per_interval.get().to_sat_i64();
							temp *= interval_count;
							NonZeroU8::new(temp.squeeze_to_u8())
						};
						poison_option.map(|poison| {
							let event = TimelineEvent {
								time_frame_ms: 0.to_sat_u64(),
								event_type: EventType::PoisonTick { poison },
								character_guid: owner.guid
							};
							
							events.push(event);
						});
						
						let mut temp = accumulated_ms.to_sat_i64();
						temp -= interval_count * interval_ms.get();
						temp *= -1;
						temp
					};
				
				for _ in 0..total_intervals_count {
					current_ms += interval_ms.get();
					let event = TimelineEvent {
						time_frame_ms: current_ms.to_sat_u64(),
						event_type: EventType::PoisonTick { poison: *poison_per_interval },
						character_guid: owner.guid,
					};

					events.push(event);
				}
				
				let remaining_ms_option = { 
					let mut temp = total_time_ms.to_sat_i64();
					temp -= total_intervals_count * interval_ms.get();
					NonZeroU64::new(temp.squeeze_to())
				};
				
				remaining_ms_option.map(|remaining_ms| {
					current_ms += remaining_ms.get();
					let poison_option = {
						let mut temp = poison_per_interval.get().to_sat_i64();
						temp *= remaining_ms.get();
						temp /= interval_ms.get();
						NonZeroU8::new(temp.squeeze_to())
					};
					poison_option.map(|poison| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(),
							event_type: EventType::PoisonTick { poison },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
				});
			}
			PersistentEffect::Heal { duration_ms, accumulated_ms, heal_per_interval, .. } => {
				const INTERVAL_MS: u64 = 1000;
				
				let total_time_ms = duration_ms.get() + accumulated_ms.get();
				let total_intervals_count = total_time_ms / INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let heal_option = {
						let mut temp = heal_per_interval.get().to_sat_i64();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					
					heal_option.map(|heal| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::HealTick { heal },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
					
					return;
				}
				
				let mut current_ms =
					if accumulated_ms.get() < INTERVAL_MS {
						let mut temp = accumulated_ms.to_sat_i64();
						temp *= -1;
						temp
					} else {
						let interval_count = accumulated_ms.get() / 1000;
						let heal_option = {
							let mut temp = heal_per_interval.get().to_sat_i64();
							temp *= interval_count;
							NonZeroU8::new(temp.squeeze_to())
						};

						heal_option.map(|heal| {
							let event = TimelineEvent {
								time_frame_ms: 0.to_sat_u64(),
								event_type: EventType::HealTick { heal },
								character_guid: owner.guid
							};
							
							events.push(event);
						});

						let mut temp = accumulated_ms.to_sat_i64();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp
					};
				
				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;
					let event = TimelineEvent {
						time_frame_ms: current_ms.to_sat_u64(),
						event_type: EventType::HealTick { heal: *heal_per_interval },
						character_guid: owner.guid
					};
					
					events.push(event);
				}
				
				let remaining_ms_option = {
					let mut temp = total_time_ms.to_sat_i64();
					temp -= total_intervals_count * INTERVAL_MS;
					NonZeroU64::new(temp.squeeze_to())
				};

				remaining_ms_option.map(|remaining_ms| {
					current_ms += remaining_ms.get();
					let heal_option = {
						let mut temp = heal_per_interval.get().to_sat_i64();
						temp *= remaining_ms.get();
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					
					heal_option.map(|heal| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(),
							event_type: EventType::HealTick { heal },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
				});
			}
			PersistentEffect::Arousal { duration_ms, accumulated_ms, lust_per_interval, .. } => {
				const INTERVAL_MS: u64 = 1000;
				
				let total_time_ms = duration_ms.get() + accumulated_ms.get();
				let total_intervals_count = total_time_ms / INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let lust_option = {
						let mut temp = lust_per_interval.get().to_sat_i64();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::LustTick { lust },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
					
					return;
				}
				
				let mut current_ms =
					if accumulated_ms.get() < INTERVAL_MS {
						let mut temp = accumulated_ms.to_sat_i64();
						temp *= -1;
						temp
					} else {
						let interval_count = accumulated_ms.get() / INTERVAL_MS;
						let lust_option = {
							let mut temp = lust_per_interval.get().to_sat_i64();
							temp *= interval_count;
							NonZeroU8::new(temp.squeeze_to())
						};
						
						lust_option.map(|lust| {
							let event = TimelineEvent {
								time_frame_ms: 0.to_sat_u64(),
								event_type: EventType::LustTick { lust },
								character_guid: owner.guid
							};
							
							events.push(event);
						});
						
						let mut temp = accumulated_ms.to_sat_i64();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp
					};
				
				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;
					let event = TimelineEvent {
						time_frame_ms: current_ms.to_sat_u64(),
						event_type: EventType::LustTick { lust: *lust_per_interval },
						character_guid: owner.guid
					};
					
					events.push(event);
				}
				
				// todo! This was incomplete last time I checked, check if it's still the case
				let remaining_ms_option = {
					let mut temp = total_time_ms.to_sat_i64();
					temp -= total_intervals_count * INTERVAL_MS;
					NonZeroU64::new(temp.squeeze_to())
				};
				
				remaining_ms_option.map(|remaining_ms| {
					current_ms += remaining_ms.get();
					let lust_option = {
						let mut temp = lust_per_interval.get().to_sat_i64();
						temp *= remaining_ms.get();
						temp /= INTERVAL_MS;
						NonZeroU8::new(temp.squeeze_to())
					};
					
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.to_sat_u64(),
							event_type: EventType::LustTick { lust },
							character_guid: owner.guid
						};
						
						events.push(event);
					});
				});
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum EventType {
	TurnBegin,
	PoisonTick { poison: NonZeroU8 },
	LustTick { lust: NonZeroU8 },
	TemptationTick { temptation: NonZeroU8 },
	HealTick { heal: NonZeroU8 },
	StunEnd,
	DownedEnd,
	StatusEnd { effect_clone: PersistentEffect },
	GrapplingEnd,
	SkillIntention { intention_clone: SkillIntention },
}