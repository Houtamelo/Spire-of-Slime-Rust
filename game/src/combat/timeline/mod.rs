use std::cell::{BorrowError, Ref, RefCell};
use std::rc::{Rc, Weak};
use crate::combat::{CombatCharacter};
use crate::combat::effects::persistent::PersistentEffect;
use crate::{CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, STANDARD_INTERVAL_MS, STANDARD_INTERVAL_S};
use crate::combat::entity::{CharacterState, SkillIntention};

#[derive(Debug, Clone)]
pub struct TimelineEvent {
	pub time_frame_ms: i64,
	pub event_type: EventType,
	pub character: Weak<RefCell<CombatCharacter>>
}

impl PartialEq for TimelineEvent {
	fn eq(&self, other: &Self) -> bool {
		return self.time_frame_ms == other.time_frame_ms 
				&& self.event_type == other.event_type
				&& Weak::ptr_eq(&self.character, &other.character)
	}
}

impl Eq for TimelineEvent {}

impl TimelineEvent {
	pub fn register_character(character_rc: &Rc<RefCell<CombatCharacter>>, events: &mut Vec<TimelineEvent>) {
		let character = match character_rc.try_borrow() {
			Ok(ok) => { ok }
			Err(err) => {
				eprintln!("Trying to register character but it is already borrowed: {:?}", err);
				return;
			}
		};

		let owner_down: Weak<RefCell<CombatCharacter>> = Rc::downgrade(character_rc);
		let mut current_ms: i64 = 0;
		
		match character.state {
			CharacterState::Idle => {} 
			CharacterState::Grappling { .. } => {} 
			CharacterState::Downed { .. } => {}
			CharacterState::Stunned { .. } => {} 
			CharacterState::Charging { .. } => {} 
			CharacterState::Recovering { .. } => {}
		}

		/*if let Some(girl) = &character.girl {
			if let Some(downed) = &girl.downed{
				let downed_ms = downed.remaining_ms;
				if downed_ms > 0 {
					current_ms += downed_ms;
					events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::DownedEnd, character: owner_down.clone() });
				}
			}
		}

		if let Some(stun_ms) = &character.stun {
			if stun_ms.remaining_ms > 0 {
				current_ms += stun_ms.remaining_ms;
				events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::StunEnd, character: owner_down.clone() });
			}
		}

		if let Some(recovery_ms) = &character.recovery {
			if recovery_ms.remaining_ms > 0 {
				current_ms += recovery_ms.remaining_ms;
				events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::TurnBegin, character: owner_down.clone() });
			}
		}

		if let Some(skill_intention) = &character.skill_intention {
			if skill_intention.charge_progress.remaining_ms > 0 {
				current_ms += skill_intention.charge_progress.remaining_ms;
				events.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() }, character: owner_down.clone() });
			}
		}*/

		for status in &character.persistent_effects {
			Self::register_status(status, character_rc, events);
		}
	}
	
	fn register_status(status: &PersistentEffect, owner_rc: &Rc<RefCell<CombatCharacter>>, allEvents: &mut Vec<TimelineEvent>) {
		let owner_down: Weak<RefCell<CombatCharacter>> = Rc::downgrade(owner_rc);
		let event_end_ms = status.duration_remaining();
		debug_assert!(event_end_ms > 0, "Trying to register an event from status with negative duration: {:?}, duration: {:?}", status, event_end_ms);
		allEvents.push(TimelineEvent { time_frame_ms: event_end_ms, event_type: EventType::StatusEnd { effect_clone: status.clone() }, character: owner_down.clone() });
		
		match status {
			PersistentEffect::Poison  { duration_ms, accumulated_ms, dmg_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let dmg: i64 = (total_time_ms * (*dmg_per_sec as i64)) / 1000;
					if dmg > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character: owner_down.clone() });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let dmg: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*dmg_per_sec as i64);
					if dmg > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::PoisonTick { amount: dmg as usize }, character: owner_down.clone() });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}

				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let dmg: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*dmg_per_sec as i64);
					allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character: owner_down.clone() });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let dmg: i64 = (remaining_ms * (*dmg_per_sec as i64)) / 1000;
					if dmg > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::PoisonTick { amount: dmg as usize }, character: owner_down.clone() });
					}
				}
			}
			PersistentEffect::Heal { duration_ms, accumulated_ms, heal_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let heal: i64 = (total_time_ms * (*heal_per_sec as i64)) / 1000;
					if heal > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::HealTick { amount: heal as usize }, character: owner_down.clone() });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let heal: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*heal_per_sec as i64);
					if heal > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::HealTick { amount: heal as usize }, character: owner_down.clone() });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let heal: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*heal_per_sec as i64);
					allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::HealTick { amount: heal as usize }, character: owner_down.clone() });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let heal: i64 = (remaining_ms * (*heal_per_sec as i64)) / 1000;
					if heal > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::HealTick { amount: heal as usize }, character: owner_down.clone() });
					}
				}
			}
			PersistentEffect::Arousal { duration_ms, accumulated_ms, lust_per_sec, .. } => {
				let total_time_ms: i64 = duration_ms + accumulated_ms;
				let total_intervals_count: i64 = total_time_ms / STANDARD_INTERVAL_MS;
				
				if total_intervals_count < 1 {
					let lust: i64 = (total_time_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: *duration_ms, event_type: EventType::LustTick { amount: lust as usize }, character: owner_down.clone() });
					}
					
					return;
				}
				
				let mut current_ms: i64;
				
				if *accumulated_ms >= STANDARD_INTERVAL_MS {
					let standard_interval_count: i64 = accumulated_ms / STANDARD_INTERVAL_MS;
					let lust: i64 = (standard_interval_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) * (*lust_per_sec as i64);
					if lust > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: 0, event_type: EventType::LustTick { amount: lust as usize }, character: owner_down.clone() });
					}
					
					current_ms = -1 * (accumulated_ms - standard_interval_count * 1000);
				}
				else { 
					current_ms = -1 * accumulated_ms;
				}
				
				for _ in 0..total_intervals_count {
					current_ms += STANDARD_INTERVAL_MS;
					let lust: i64 = CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * (*lust_per_sec as i64);
					allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character: owner_down.clone() });
				}
				
				let remaining_ms = total_time_ms - (total_intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT * 1000);
				if remaining_ms > 0 {
					current_ms += remaining_ms;
					let lust: i64 = (remaining_ms * (*lust_per_sec as i64)) / 1000;
					if lust > 0 {
						allEvents.push(TimelineEvent { time_frame_ms: current_ms, event_type: EventType::LustTick { amount: lust as usize }, character: owner_down.clone() });
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
	PoisonTick { amount: usize },
	LustTick   { amount: usize },
	HealTick   { amount: usize },
	StunEnd,
	DownedEnd,
	StatusEnd { effect_clone: PersistentEffect },
	SkillIntention { intention_clone: SkillIntention },
}