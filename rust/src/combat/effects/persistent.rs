use combat::ModifiableStat;
use crate::combat;
use crate::combat::entity::character::*;
use crate::combat::skills::CRITMode;

#[derive(Debug, Clone)]
pub enum PersistentEffect {
	Poison {
		duration_ms: i64,
		accumulated_ms: i64,
		dmg_per_sec: usize,
		caster_guid: usize,
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
		guarder_guid: usize,
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
}

impl PartialEq for PersistentEffect {
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
			_ => false,
		}
	}
}

impl Eq for PersistentEffect {}

impl PersistentEffect {
	pub fn tick(&mut self, ms: i64) {
		match self {
			PersistentEffect::Poison { duration_ms, accumulated_ms,.. } => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Heal{ duration_ms, accumulated_ms,.. } => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Arousal{ duration_ms, accumulated_ms, .. } => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Buff{ duration_ms,.. } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Guarded{ duration_ms, .. } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Marked{ duration_ms } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Riposte{ duration_ms, .. } => {
				*duration_ms -= ms;
			},
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
			PersistentEffect::Poison { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Heal   { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Arousal{ duration_ms, ..} => { *duration_ms },
			PersistentEffect::Buff   { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Guarded{ duration_ms, ..} => { *duration_ms },
			PersistentEffect::Marked { duration_ms    } => { *duration_ms },
			PersistentEffect::Riposte{ duration_ms, ..} => { *duration_ms },
		};
	}
}