use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use crate::combat::{CombatCharacter, CombatState, Side};
use crate::combat::effects::{MoveDirection};
use crate::combat::effects::persistent;
use crate::combat::effects::persistent::PersistentEffect as PersistentEffect;
use crate::combat::entity::Entity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfApplier {
	Buff {
		duration_ms: i64,
		stat: crate::combat::ModifiableStat,
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
	pub fn apply(&self, caster_rc: &mut Rc<RefCell<CombatCharacter>>, side: &mut Side, manager: &mut CombatState) {
		let seed = &mut manager.seed;
		let mut caster = caster_rc.get_mut();

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
				match &mut caster.girl {
					None => {
						return;
					}
					Some(girl) => {
						let actual_min = *min.min(&(max - 1));
						let lustAmount = seed.gen_range(*min..=*max);
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

				let (index_current, allies): (&mut usize, &Vec<Entity>) = match side {
					Side::Left (pos) => (pos, &manager.left_characters),
					Side::Right(pos) => (pos, &manager.right_characters),
				};

				let index_new = (((*index_current as isize) + direction) as usize).clamp(0, allies.len());
				*index_current = index_new;
			}
			SelfApplier::PersistentHeal{ duration_ms, heal_per_sec } => {
				caster.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, *heal_per_sec));
			}
			SelfApplier::Riposte{ duration_ms, dmg_multiplier, acc } => {
				caster.persistent_effects.push( PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc));
			}
			SelfApplier::Summon{ character_key } => {} //todo!
		}
	}
}
