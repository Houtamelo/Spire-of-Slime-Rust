use crate::combat::effects::persistent::{PersistentEffect, PoisonAdditive};
use crate::combat::entity::character::*;
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::ModifiableStat::SPD;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub struct TimelineEvent {
	pub time_frame_ms: i64,
	pub event_type: EventType,
	pub character_guid: GUID
}

impl TimelineEvent {
	pub fn register_character(character: &CombatCharacter, events: &mut Vec<TimelineEvent>) {
		let character_guid = character.guid;
		match &character.state {
			CharacterState::Idle => {
				events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::TurnBegin, character_guid });
			}
			CharacterState::Grappling(g) => {
				events.push(TimelineEvent { time_frame_ms: g.duration_ms, event_type: EventType::GrapplingEnd, character_guid });
				
				let total_time_ms: i64 = g.duration_ms + g.accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / 1000;

				if total_intervals_count < 1 {
					let lust: i64 = (total_time_ms * (g.lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: g.duration_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: g.victim.guid() });
					}
					
					let temptation: i64 = (total_time_ms * (g.temptation_per_sec as i64)) / 1000;
					if temptation > 0 {
						events.push(TimelineEvent { time_frame_ms: g.duration_ms, event_type: EventType::TemptationTick { amount: temptation as usize }, character_guid: g.victim.guid() });
					}

					return;
				}

				let mut current_ms: i64 = 0;

				if g.accumulated_ms >= 1000 {
					let interval_count: i64 = g.accumulated_ms / 1000;
					
					let lust: i64 = interval_count * (g.lust_per_sec as i64);
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: g.victim.guid() });
					}
					
					let temptation: i64 = interval_count * (g.temptation_per_sec as i64);
					if temptation > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::TemptationTick { amount: temptation as usize }, character_guid: g.victim.guid() });
					}

					current_ms = -1 * (g.accumulated_ms - interval_count * 1000);
				}
				else {
					current_ms = -1 * g.accumulated_ms;
				}

				for _ in 0..total_intervals_count {
					current_ms += 1000;
					let lust: i64 = g.lust_per_sec as i64;
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid });
				}

				let remaining_ms = total_time_ms - (total_intervals_count * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let lust: i64 = (remaining_ms * (g.lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid });
					}
					
					let temptation: i64 = (remaining_ms * (g.temptation_per_sec as i64)) / 1000;
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
				let estimated_charge_ms = CharacterState::spd_charge_ms(skill_intention.charge_ticks.remaining_ms, character.get_stat(SPD));
				if estimated_charge_ms > 0 {
					events.push(TimelineEvent { time_frame_ms: estimated_charge_ms, event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() }, character_guid });
				}
			}
			CharacterState::Recovering { ticks } => {
				let estimated_recovery_ms = CharacterState::spd_recovery_ms(ticks.remaining_ms, character.get_stat(SPD));
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
		let event_end_ms = status.duration();
		debug_assert!(event_end_ms > 0, "Trying to register an event from status with negative duration: {:?}, duration: {:?}", status, event_end_ms);
		events.push(TimelineEvent { time_frame_ms: event_end_ms, event_type: EventType::StatusEnd { effect_clone: status.clone() }, character_guid: owner.guid });
		
		match status {
			PersistentEffect::Poison { duration_ms, accumulated_ms, dmg_per_interval: dmg_per_sec, additives, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / 1000;
				
				if total_intervals_count < 1 {
					let dmg: i64 = (total_time_ms * (*dmg_per_sec as i64)) / 1000;
					let mut event_ms = *duration_ms;

					if additives.iter().any(|add| matches!(add, PoisonAdditive::Nema_Madness)) {
						event_ms = (event_ms * 100) / 150;
					}

					if dmg > 0 {
						events.push(TimelineEvent { time_frame_ms: event_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= 1000 {
					let interval_count: i64 = accumulated_ms / 1000;
					let dmg: i64 = interval_count * (*dmg_per_sec as i64);
					if dmg > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}

				for _ in 0..total_intervals_count {
					current_ms += 1000;
					let dmg: i64 = *dmg_per_sec as i64;
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * 1000);
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
				let total_intervals_count: i64 = total_time_ms / 1000;
				
				if total_intervals_count < 1 {
					let heal: i64 = (total_time_ms * (*heal_per_sec as i64)) / 1000;
					if heal > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= 1000 {
					let interval_count: i64 = accumulated_ms / 1000;
					let heal: i64 = interval_count * (*heal_per_sec as i64);
					if heal > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += 1000;
					let heal: i64 = *heal_per_sec as i64;
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::HealTick { amount: heal as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * 1000);
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
				let total_intervals_count: i64 = total_time_ms / 1000;
				
				if total_intervals_count < 1 {
					let lust: i64 = (total_time_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= 1000 {
					let interval_count: i64 = accumulated_ms / 1000;
					let lust: i64 = interval_count * (*lust_per_sec as i64);
					if lust > 0 {
						events.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
					}
					
					current_ms = -1 * (accumulated_ms - interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += 1000;
					let lust: i64 = *lust_per_sec as i64;
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character_guid: owner.guid });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * 1000);
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

#[derive(Debug, Clone)]
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