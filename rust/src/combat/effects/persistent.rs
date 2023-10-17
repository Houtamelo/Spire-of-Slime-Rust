use std::collections::HashMap;
use gdnative::log::godot_warn;
use combat::ModifiableStat;
use crate::{combat, CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT, STANDARD_INTERVAL_MS};
use crate::combat::entity::character::*;
use crate::combat::entity::Entity;
use crate::combat::perk::Perk;
use crate::combat::skill_types::CRITMode;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub enum PersistentEffect {
	Poison {
		duration_ms: i64,
		accumulated_ms: i64,
		dmg_per_sec: usize,
		caster_guid: GUID,
	},
	Heal {
		duration_ms: i64,
		accumulated_ms: i64,
		heal_per_sec: usize,
	},
	Arousal {
		duration_ms: i64,
		accumulated_ms: i64,
		lust_per_sec: usize,
	},
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_increase: usize,
	},
	Debuff(PersistentDebuff),
	Guarded {
		duration_ms: i64,
		guarder_guid: GUID,
	},
	Marked {
		duration_ms: i64,
	},
	Riposte {
		duration_ms: i64,
		dmg_multiplier: isize,
		acc: isize,
		crit: CRITMode,
	},
	TemporaryPerk {
		duration_ms: i64,
		perk: Perk,
	}
}

#[derive(Debug, Clone)]
pub enum PersistentDebuff {
	Standard {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_decrease: usize,
	},
	StaggeringForce {
		duration_ms: i64,
	}
}

impl PersistentDebuff {
	pub fn duration(&self) -> i64 {
		match self {
			PersistentDebuff::Standard { duration_ms, .. } => { *duration_ms },
			PersistentDebuff::StaggeringForce { duration_ms, .. } => { *duration_ms },
		}
	}

	pub fn duration_mut(&mut self) -> &mut i64 {
		match self {
			PersistentDebuff::Standard        { duration_ms, .. } => { duration_ms },
			PersistentDebuff::StaggeringForce { duration_ms, .. } => { duration_ms },
		}
	}
}

impl PersistentEffect {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<GUID, Entity>, ms: i64) {
		let iter : Vec<PersistentEffect> = owner.persistent_effects.drain(0..owner.persistent_effects.len()).collect();
		for mut effect in iter {
			match &mut effect {
				PersistentEffect::Poison { duration_ms, accumulated_ms, dmg_per_sec, .. } => {
					let actual_ms = clamp_tick_ms(&ms, *duration_ms);
					
					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;
					
					let intervals_count = *accumulated_ms / STANDARD_INTERVAL_MS;
					let dmg: usize = (intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as usize * (*dmg_per_sec);

					if dmg > 0 {
						*accumulated_ms -= intervals_count * STANDARD_INTERVAL_MS;
						owner.stamina_cur -= dmg as isize;
						if owner.is_dead() { // poison killed this character, we can ignore the rest of the status effects
							return;
						}
					}

					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Heal{ duration_ms, accumulated_ms, heal_per_sec } => {
					let actual_ms = clamp_tick_ms(&ms, *duration_ms);
					
					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;
					
					let intervals_count = *accumulated_ms / STANDARD_INTERVAL_MS;
					let heal: usize = (intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as usize * (*heal_per_sec);
					
					if heal > 0 {
						*accumulated_ms -= intervals_count * STANDARD_INTERVAL_MS;
						owner.stamina_cur += heal as isize;
					}

					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Arousal{ duration_ms, accumulated_ms, lust_per_sec } => {
					let actual_ms = clamp_tick_ms(&ms, *duration_ms);
					
					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;
					
					let intervals_count = *accumulated_ms / STANDARD_INTERVAL_MS;
					let lust: usize = (intervals_count * CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT) as usize * (*lust_per_sec);
					
					if lust > 0 {
						let Some(girl) = &mut owner.girl_stats else { 
							godot_warn!("character has arousal status but isn't a girl: {owner:?}");
							continue;
						};
						
						*accumulated_ms -= intervals_count * STANDARD_INTERVAL_MS;
						girl.lust += lust as isize;
					}

					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Buff{ duration_ms, .. } => {
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Debuff(debuff) => {
					let duration_ms = debuff.duration_mut();
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Guarded{ duration_ms, .. } => {
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Marked{ duration_ms } => {
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Riposte{ duration_ms, .. } => {
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::TemporaryPerk{ duration_ms, .. } => {
					*duration_ms -= ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
			}
		}
		
		others.insert(owner.guid, Entity::Character(owner));
		return;
		
		fn clamp_tick_ms(input: &i64, duration: i64) -> i64 {
			if *input <= duration { 
				return *input;
			} else {
				godot_warn!("Tick ms is greater than duration_ms. This should not happen. Tick ms: {}, duration_ms: {}", input, duration);
				return duration;
			};
		}
	}
	
	pub fn duration(&self) -> i64 {
		return match self {
			PersistentEffect::Poison       { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Heal         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Arousal      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Buff         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Guarded      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Marked       { duration_ms    } => { *duration_ms },
			PersistentEffect::Riposte      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::TemporaryPerk{ duration_ms, ..} => { *duration_ms },
			PersistentEffect::Debuff(debuff) => { debuff.duration() },
		};
	}
}