use rand::prelude::StdRng;
use rand::Rng;
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use combat::ModifiableStat;
use crate::combat;
use crate::combat::{CombatCharacter, Position};
use crate::combat::effects::{MoveDirection};
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::Entity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfApplier {
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		modifier: isize,
	},
	Heal { 
		base_multiplier: isize,
	},
	Lust {
		min: usize,
		max: usize,
	},
	Mark { 
		duration_ms: i64,
	},
	Move { 
		direction: MoveDirection,
	},
	PersistentHeal {
		duration_ms: i64,
		heal_per_sec: usize,
	},
	Riposte {
		duration_ms: i64,
		dmg_multiplier: isize,
		acc: isize,
	},
	Summon { 
		character_key: Box<str>,
	},
}

impl SelfApplier {
	pub fn apply(&self, caster: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, seed: &mut StdRng, crit: bool) {
		match self
		{
			SelfApplier::Buff{ duration_ms, stat, modifier } => {
				caster.persistent_effects.push(PersistentEffect::new_buff(*duration_ms, *stat, *modifier));
			}
			SelfApplier::Heal{ base_multiplier } => {
				let max: isize = caster.damage.max.max(0);
				let min: isize = caster.damage.min.clamp(0, max);

				let healAmount: isize;

				if max <= 0 {
					return;
				} else if max == min {
					healAmount = max;
				} else {
					healAmount = (seed.gen_range(min..=max) * (*base_multiplier)) / 100;
				}

				caster.stamina_cur = (caster.stamina_cur + healAmount).clamp(0, caster.stamina_max);
			}
			SelfApplier::Lust{ min, max } => {
				match &mut caster.girl_stats {
					None => {
						return;
					}
					Some(girl) => {
						let actual_min = *min.min(&(max - 1));
						let lustAmount = seed.gen_range(actual_min..=*max);
						girl.lust += lustAmount as isize;
					}
				}
			}
			SelfApplier::Mark{ duration_ms } => {
				caster.persistent_effects.push(PersistentEffect::new_marked(*duration_ms));
			}
			SelfApplier::Move{ direction } => {
				let direction: isize = match *direction {
					MoveDirection::ToCenter(amount) => { -1 * amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let index_current : &mut usize = match &mut caster.position {
					Position::Left  { order: pos, .. } => pos,
					Position::Right { order: pos, .. } => pos,
				};

				let mut allies_space_occupied = 0;
				for ally in allies {
					allies_space_occupied += ally.position().size();
				}

				let index_old = index_current.clone() as isize;
				*index_current = usize::clamp(((*index_current as isize) + direction) as usize, 0, allies_space_occupied);
				let index_delta = *index_current as isize - index_old;
				let inverse_delta = -1 * index_delta;

				for ally in allies {
					let order = ally.position().order_mut();
					*order = (*order as isize + inverse_delta) as usize;
				}
			}
			SelfApplier::PersistentHeal{ duration_ms, heal_per_sec } => {
				caster.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, *heal_per_sec));
			}
			SelfApplier::Riposte{ duration_ms, dmg_multiplier, acc } => {
				caster.persistent_effects.push( PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc));
			}
			SelfApplier::Summon{ .. } => {} //todo!
		}
	}
}