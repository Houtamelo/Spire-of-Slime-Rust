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
		modifier: isize,
	},
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

/*+impl PartialEq for PersistentEffect {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(PersistentEffect::Poison { duration_ms: a_dur, accumulated_ms: a_acu, dmg_per_sec: a_dmg, caster_guid: a_guid }, 
			 PersistentEffect::Poison { duration_ms: b_dur, accumulated_ms: b_acu, dmg_per_sec: b_dmg, caster_guid: b_guid })
			=> a_dur == b_dur && a_acu == b_acu && a_dmg == b_dmg && a_guid == b_guid,
			(PersistentEffect::Heal { duration_ms: a_dur, accumulated_ms: a_acu, heal_per_sec: a_heal }, 
			 PersistentEffect::Heal { duration_ms: b_dur, accumulated_ms: b_acu, heal_per_sec: b_heal })
			=> a_dur == b_dur && a_acu == b_acu && a_heal == b_heal,
			(PersistentEffect::Arousal { duration_ms: a_dur, accumulated_ms: a_acu, lust_per_sec: a_lust }, 
			 PersistentEffect::Arousal { duration_ms: b_dur, accumulated_ms: b_acu, lust_per_sec: b_lust })
			=> a_dur == b_dur && a_acu == b_acu && a_lust == b_lust,
			(PersistentEffect::Buff { duration_ms: a_dur, stat: a_stat, modifier: a_mod }, 
			 PersistentEffect::Buff { duration_ms: b_dur, stat: b_stat, modifier: b_mod })
			=> a_dur == b_dur && a_stat == b_stat && a_mod == b_mod,
			(PersistentEffect::Guarded { duration_ms: a_dur, guarder_guid: a_guarder }, 
			 PersistentEffect::Guarded { duration_ms: b_dur, guarder_guid: b_guarder })
			=> a_dur == b_dur && a_guarder == b_guarder,
			(PersistentEffect::Marked { duration_ms: a_dur }, 
			 PersistentEffect::Marked { duration_ms: b_dur })
			=> a_dur == b_dur,
			(PersistentEffect::Riposte { duration_ms: a_dur, dmg_multiplier: a_dmg, acc: a_acc, crit: a_crit }, 
			 PersistentEffect::Riposte { duration_ms: b_dur, dmg_multiplier: b_dmg, acc: b_acc, crit: b_crit})
			=> a_dur == b_dur && a_dmg == b_dmg && a_acc == b_acc && a_crit == b_crit,
			(PersistentEffect::TemporaryPerk { duration_ms: a_dur, perk: a_perk },
			 PersistentEffect::TemporaryPerk { duration_ms: b_dur, perk: b_perk })
			=> a_dur == b_dur && a_perk == b_perk,
			_ => false,
		}
	}
}

impl Eq for PersistentEffect {}*/

impl PersistentEffect {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, insert_owner_here: &mut HashMap<GUID, Entity>, ms: i64) {
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
					
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
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
					
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
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
					
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
				PersistentEffect::Buff{ duration_ms, .. } => {
					*duration_ms -= ms;
					
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
				PersistentEffect::Guarded{ duration_ms, .. } => {
					*duration_ms -= ms;
				
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
				PersistentEffect::Marked{ duration_ms } => {
					*duration_ms -= ms;
					
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
				PersistentEffect::Riposte{ duration_ms, .. } => {
					*duration_ms -= ms;
				
					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
				PersistentEffect::TemporaryPerk{ duration_ms, .. } => {
					*duration_ms -= ms;

					if *duration_ms > 0 {
						owner.persistent_effects.push(effect);
					}
				},
			}
		}
		
		insert_owner_here.insert(owner.guid, Entity::Character(owner));
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
	
	pub fn new_poison(duration_ms: i64, dmg_per_sec: usize, caster: &CombatCharacter) -> Self { 
		PersistentEffect::Poison {
			duration_ms,
			accumulated_ms: 0,
			dmg_per_sec,
			caster_guid: caster.guid,
		}
	}
	
	pub fn new_heal(duration_ms: i64, heal_per_sec: usize) -> Self { 
		PersistentEffect::Heal {
			duration_ms,
			accumulated_ms: 0,
			heal_per_sec,
		}
	}
	
	pub fn new_arousal(duration_ms: i64, lust_per_sec: usize) -> Self { 
		PersistentEffect::Arousal {
			duration_ms,
			accumulated_ms: 0,
			lust_per_sec,
		}
	}
	
	pub fn new_buff(duration_ms: i64, stat: ModifiableStat, modifier: isize) -> Self { 
		PersistentEffect::Buff {
			duration_ms,
			stat,
			modifier,
		}
	}
	
	pub fn new_guarded(duration_ms: i64, guarder: &CombatCharacter) -> Self { 
		PersistentEffect::Guarded {
			duration_ms,
			guarder_guid: guarder.guid,
		}
	}
	
	pub fn new_marked(duration_ms: i64) -> Self { 
		PersistentEffect::Marked {
			duration_ms,
		}
	}
	
	pub fn new_riposte(duration_ms: i64, dmg_multiplier: isize, acc: isize, crit: CRITMode) -> Self { 
		PersistentEffect::Riposte {
			duration_ms,
			dmg_multiplier,
			acc,
			crit,
		}
	}
	
	pub fn duration_remaining(&self) -> i64 {
		return match self {
			PersistentEffect::Poison       { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Heal         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Arousal      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Buff         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Guarded      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Marked       { duration_ms    } => { *duration_ms },
			PersistentEffect::Riposte      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::TemporaryPerk{ duration_ms, ..} => { *duration_ms },
		};
	}
}