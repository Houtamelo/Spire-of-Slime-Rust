use crate::combat::{CombatCharacter};
use crate::combat::effects::persistent::PersistentEffect;
use crate::{CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, STANDARD_INTERVAL_MS};
use crate::combat::entity::{CharacterState, SkillIntention, StateBeforeStunned};
use crate::combat::ModifiableStat::SPD;

#[derive(Debug, Clone)]
pub struct TimelineEvent {
	pub time_frame_ms: i64,
	pub event_type: EventType,
	pub character_guid: usize
}

impl PartialEq for TimelineEvent {
	fn eq(&self, other: &Self) -> bool {
		return self.time_frame_ms == other.time_frame_ms 
				&& self.event_type == other.event_type
				&& self.character_guid == other.character_guid;
	}
}

impl Eq for TimelineEvent {}

impl TimelineEvent {
	pub fn register_character(character: &CombatCharacter, events: &mut Vec<TimelineEvent>) {
		let character_guid = character.guid;
		match &character.state {
			CharacterState::Idle => {
				events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::TurnBegin, character_guid });
			}
			CharacterState::Grappling { victim, lust_per_sec, temptation_per_sec, duration_ms, accumulated_ms } => {
				events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::GrapplingEnd, character_guid });
				
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;

				if total_intervals_count < 1 {
					let lust: i64 = (total_time_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: victim.guid() });
					}
					
					let temptation: i64 = (total_time_ms * (*temptation_per_sec as i64)) / 1000;
					if temptation > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::TemptationTick { amount: temptation as usize }, character_guid: victim.guid() });
					}

					return;
				}

				let mut current_ms: i64 = 0;

				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					
					let lust: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*lust_per_sec as i64);
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: victim.guid() });
					}
					
					let temptation: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*temptation_per_sec as i64);
					if temptation > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::TemptationTick { amount: temptation as usize }, character_guid: victim.guid() });
					}

					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else {
					current_ms = -1 * accumulated_ms;
				}

				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let lust: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*lust_per_sec as i64);
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid });
				}

				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let lust: i64 = (remaining_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid });
					}
					
					let temptation: i64 = (remaining_ms * (*temptation_per_sec as i64)) / 1000;
					if temptation > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::TemptationTick { amount: temptation as usize }, character_guid });
					}
				}
			} 
			CharacterState::Downed { ticks } => {
				if ticks.remaining_ms > 0 {
					events.push(TimelineEvent { time_frame_ms: ticks.remaining_ms, event_type: EventType::DownedEnd, character_guid });
				}
			}
			CharacterState::Stunned { ticks: stunned_ticks, state_before_stunned } => {
				if stunned_ticks.remaining_ms > 0 {
					events.push(TimelineEvent { time_frame_ms: stunned_ticks.remaining_ms, event_type: EventType::StunEnd, character_guid });
				}
				
				match state_before_stunned {
					StateBeforeStunned::Recovering { ticks: recovering_ticks } => {
						if recovering_ticks.remaining_ms > 0 {
							events.push(TimelineEvent { time_frame_ms: recovering_ticks.remaining_ms + stunned_ticks.remaining_ms, event_type: EventType::TurnBegin, character_guid });
						}
					}
					StateBeforeStunned::Charging { skill_intention } => {
						if skill_intention.charge_ticks.remaining_ms > 0 {
							events.push(TimelineEvent { time_frame_ms: skill_intention.charge_ticks.remaining_ms + stunned_ticks.remaining_ms,
								event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() }, character_guid });
						}
					}
					StateBeforeStunned::Idle => {
						events.push(TimelineEvent { time_frame_ms: stunned_ticks.remaining_ms, event_type: EventType::TurnBegin, character_guid });
					}
				}
			} 
			CharacterState::Charging   { skill_intention } => {
				let estimated_charge_ms = CharacterState::spd_charge_ms(skill_intention.charge_ticks.remaining_ms, character.stat(SPD));
				if estimated_charge_ms > 0 {
					events.push(TimelineEvent { time_frame_ms: estimated_charge_ms, event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() }, character_guid });
				}
			}
			CharacterState::Recovering { ticks } => {
				let estimated_recovery_ms = CharacterState::spd_recovery_ms(ticks.remaining_ms, character.stat(SPD));
				if estimated_recovery_ms > 0 {
					events.push(TimelineEvent { time_frame_ms: estimated_recovery_ms, event_type: EventType::TurnBegin, character_guid });
				}
			}
		}

		for status in &character.persistent_effects {
			Self::register_status(status, character, events);
		}
	}
	
	fn register_status(status: &PersistentEffect, owner: &CombatCharacter, events: &mut Vec<TimelineEvent>) {
		let event_end_ms = status.duration_remaining();
		debug_assert!(event_end_ms > 0, "Trying to register an event from status with negative duration: {:?}, duration: {:?}", status, event_end_ms);
		events.push(TimelineEvent { time_frame_ms: event_end_ms, event_type: EventType::StatusEnd { effect_clone: status.clone() }, character_guid: owner.guid });
		
		match status {
			PersistentEffect::Poison  { duration_ms, accumulated_ms, dmg_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let dmg: i64 = (total_time_ms * (*dmg_per_sec as i64)) / 1000;
					if dmg > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let dmg: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*dmg_per_sec as i64);
					if dmg > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}

				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let dmg: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*dmg_per_sec as i64);
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let dmg: i64 = (remaining_ms * (*dmg_per_sec as i64)) / 1000;
					if dmg > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
					}
				}
			}
			PersistentEffect::Heal { duration_ms, accumulated_ms, heal_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let heal: i64 = (total_time_ms * (*heal_per_sec as i64)) / 1000;
					if heal > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let heal: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*heal_per_sec as i64);
					if heal > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let heal: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*heal_per_sec as i64);
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let heal: i64 = (remaining_ms * (*heal_per_sec as i64)) / 1000;
					if heal > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
					}
				}
			}
			PersistentEffect::Arousal { duration_ms, accumulated_ms, lust_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let lust: i64 = (total_time_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let lust: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*lust_per_sec as i64);
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let lust: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*lust_per_sec as i64);
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let lust: i64 = (remaining_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
					}
				}
			}
			_ => {}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
	TurnBegin,
	PoisonTick       { amount: usize },
	LustTick         { amount: usize },
	TemptationTick   { amount: usize },
	HealTick         { amount: usize },
	StunEnd,
	DownedEnd,
	StatusEnd { effect_clone: PersistentEffect },
	GrapplingEnd,
	SkillIntention { intention_clone: SkillIntention },
}