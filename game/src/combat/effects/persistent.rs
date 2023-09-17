use std::cell::RefCell;
use std::rc::Weak;
use crate::combat::Character;

#[derive(Debug)]
pub enum PersistentEffect {
	Poison {
		duration_ms: isize,
		accumulated_ms: isize,
		dmg_per_sec: isize,
		caster: Weak<RefCell<Character>>,
	},
	Heal {
		duration_ms: isize,
		accumulated_ms: isize,
		heal_per_sec: isize,
	},
	Arousal {
		duration_ms: isize,
		accumulated_ms: isize,
		lust_per_sec: isize,
	},
	Buff {
		duration_ms: isize,
		stat: crate::combat::ModifiableStat,
		modifier: isize,
	},
	Guarded {
		duration_ms: isize,
		guarder: Weak<RefCell<Character>>,
	},
	Marked {
		duration_ms: isize,
	},
	Riposte {
		duration_ms: isize,
		dmg_multiplier: isize,
		acc: isize,
	},
}

impl PersistentEffect {
	pub fn tick(&mut self, ms: isize) {
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
	
	pub fn new_poison(duration_ms: isize, dmg_per_sec: isize, caster: Weak<RefCell<Character>>) -> Self { 
		PersistentEffect::Poison {
			duration_ms,
			accumulated_ms: 0,
			dmg_per_sec,
			caster,
		}
	}
	
	pub fn new_heal(duration_ms: isize, heal_per_sec: isize) -> Self { 
		PersistentEffect::Heal {
			duration_ms,
			accumulated_ms: 0,
			heal_per_sec,
		}
	}
	
	pub fn new_arousal(duration_ms: isize, lust_per_sec: isize) -> Self { 
		PersistentEffect::Arousal {
			duration_ms,
			accumulated_ms: 0,
			lust_per_sec,
		}
	}
	
	pub fn new_buff(duration_ms: isize, stat: crate::combat::ModifiableStat, modifier: isize) -> Self { 
		PersistentEffect::Buff {
			duration_ms,
			stat,
			modifier,
		}
	}
	
	pub fn new_guarded(duration_ms: isize, guarder: Weak<RefCell<Character>>) -> Self { 
		PersistentEffect::Guarded {
			duration_ms,
			guarder,
		}
	}
	
	pub fn new_marked(duration_ms: isize) -> Self { 
		PersistentEffect::Marked {
			duration_ms,
		}
	}
	
	pub fn new_riposte(duration_ms: isize, dmg_multiplier: isize, acc: isize) -> Self { 
		PersistentEffect::Riposte {
			duration_ms,
			dmg_multiplier,
			acc,
		}
	}
}