use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::combat::CombatCharacter;
use crate::combat::timeline::{EventType, TimelineEvent};

#[derive(Debug, Clone)]
pub enum PersistentEffect {
	Poison {
		duration_ms: i64,
		accumulated_ms: i64,
		dmg_per_sec: usize,
		caster: Weak<RefCell<CombatCharacter>>,
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
		stat: crate::combat::ModifiableStat,
		modifier: isize,
	},
	Guarded {
		duration_ms: i64,
		guarder: Weak<RefCell<CombatCharacter>>,
	},
	Marked {
		duration_ms: i64,
	},
	Riposte {
		duration_ms: i64,
		dmg_multiplier: isize,
		acc: isize,
	},
}

impl PartialEq for PersistentEffect {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(PersistentEffect::Poison { duration_ms: a_dur, accumulated_ms: a_acu, dmg_per_sec: a_dmg, caster: a_char }, 
			 PersistentEffect::Poison { duration_ms: b_dur, accumulated_ms: b_acu, dmg_per_sec: b_dmg, caster: b_char })
			=> a_dur == b_dur && a_acu == b_acu && a_dmg == b_dmg && Weak::ptr_eq(a_char, b_char),
			(PersistentEffect::Heal { duration_ms: a_dur, accumulated_ms: a_acu, heal_per_sec: a_heal }, 
			 PersistentEffect::Heal { duration_ms: b_dur, accumulated_ms: b_acu, heal_per_sec: b_heal })
			=> a_dur == b_dur && a_acu == b_acu && a_heal == b_heal,
			(PersistentEffect::Arousal { duration_ms: a_dur, accumulated_ms: a_acu, lust_per_sec: a_lust }, 
			 PersistentEffect::Arousal { duration_ms: b_dur, accumulated_ms: b_acu, lust_per_sec: b_lust })
			=> a_dur == b_dur && a_acu == b_acu && a_lust == b_lust,
			(PersistentEffect::Buff { duration_ms: a_dur, stat: a_stat, modifier: a_mod }, 
			 PersistentEffect::Buff { duration_ms: b_dur, stat: b_stat, modifier: b_mod })
			=> a_dur == b_dur && a_stat == b_stat && a_mod == b_mod,
			(PersistentEffect::Guarded { duration_ms: a_dur, guarder: a_guarder }, 
			 PersistentEffect::Guarded { duration_ms: b_dur, guarder: b_guarder })
			=> a_dur == b_dur && Weak::ptr_eq(a_guarder, b_guarder),
			(PersistentEffect::Marked { duration_ms: a_dur }, 
			 PersistentEffect::Marked { duration_ms: b_dur })
			=> a_dur == b_dur,
			(PersistentEffect::Riposte { duration_ms: a_dur, dmg_multiplier: a_dmg, acc: a_acc }, 
			 PersistentEffect::Riposte { duration_ms: b_dur, dmg_multiplier: b_dmg, acc: b_acc })
			=> a_dur == b_dur && a_dmg == b_dmg && a_acc == b_acc,
			_ => false,
		}
	}
}

impl Eq for PersistentEffect {}

impl PersistentEffect {
	pub fn tick(&mut self, ms: i64) {
		match self {
			PersistentEffect::Poison { duration_ms, accumulated_ms, dmg_per_sec, caster} => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Heal{ duration_ms, accumulated_ms, heal_per_sec } => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Arousal{ duration_ms, accumulated_ms, lust_per_sec } => {
				*accumulated_ms += ms;
				*duration_ms -= ms;
			},
			PersistentEffect::Buff{ duration_ms, stat, modifier } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Guarded{ duration_ms, guarder } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Marked{ duration_ms } => {
				*duration_ms -= ms;
			},
			PersistentEffect::Riposte{ duration_ms, dmg_multiplier, acc } => {
				*duration_ms -= ms;
			},
		}
	}
	
	pub fn new_poison(duration_ms: i64, dmg_per_sec: usize, caster: Weak<RefCell<CombatCharacter>>) -> Self { 
		PersistentEffect::Poison {
			duration_ms,
			accumulated_ms: 0,
			dmg_per_sec,
			caster,
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
	
	pub fn new_buff(duration_ms: i64, stat: crate::combat::ModifiableStat, modifier: isize) -> Self { 
		PersistentEffect::Buff {
			duration_ms,
			stat,
			modifier,
		}
	}
	
	pub fn new_guarded(duration_ms: i64, guarder: Weak<RefCell<CombatCharacter>>) -> Self { 
		PersistentEffect::Guarded {
			duration_ms,
			guarder,
		}
	}
	
	pub fn new_marked(duration_ms: i64) -> Self { 
		PersistentEffect::Marked {
			duration_ms,
		}
	}
	
	pub fn new_riposte(duration_ms: i64, dmg_multiplier: isize, acc: isize) -> Self { 
		PersistentEffect::Riposte {
			duration_ms,
			dmg_multiplier,
			acc,
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