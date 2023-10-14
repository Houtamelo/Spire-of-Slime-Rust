use std::collections::HashMap;
use rand::prelude::StdRng;
use rand::Rng;
use combat::ModifiableStat;
use crate::{iter_allies_of, iter_mut_allies_of, combat};
use crate::combat::entity::character::*;
use crate::combat::effects::MoveDirection;
use crate::combat::effects::onTarget::{CRIT_DURATION_MULTIPLIER, CRIT_EFFECT_MULTIPLIER, CRIT_EFFECT_MULTIPLIER_I};
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::Entity;
use crate::combat::skill_types::CRITMode;
use crate::util::GUID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfApplier {
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		modifier: isize,
	},
	ChangeExhaustion {
		delta: isize,
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
		crit: CRITMode,
	},
	Summon { 
		character_key: Box<str>,
	},
}

impl SelfApplier {
	pub fn apply(&self, caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, is_crit: bool) {
		match self {
			SelfApplier::Buff{ duration_ms, stat, mut modifier } => {
				if is_crit { modifier = (modifier * CRIT_EFFECT_MULTIPLIER_I) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_buff(*duration_ms, *stat, modifier));
			}
			SelfApplier::ChangeExhaustion { delta } => { // ignores crit
				if let Some(girl) = &mut caster.girl_stats {
					girl.exhaustion += *delta;
				}
			},
			SelfApplier::Heal{ mut base_multiplier } => {
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER_I) / 100; }

				let max: isize = caster.dmg.max.max(0);
				let min: isize = caster.dmg.min.clamp(0, max);

				let healAmount: isize;

				if max <= 0 {
					return;
				} else if max == min {
					healAmount = max;
				} else {
					healAmount = (seed.gen_range(min..=max) * (base_multiplier)) / 100;
				}

				caster.stamina_cur = (caster.stamina_cur + healAmount).clamp(0, caster.stamina_max);
			}
			SelfApplier::Lust{ mut min, mut max } => {
				if is_crit {
					min = (min * CRIT_EFFECT_MULTIPLIER) / 100;
					max = (max * CRIT_EFFECT_MULTIPLIER) / 100;
				}

				if let Some(girl) = &mut caster.girl_stats {
					let actual_min = min.min(max - 1);
					let lustAmount = seed.gen_range(actual_min..=max);
					girl.lust += lustAmount as isize;
				}
			}
			SelfApplier::Mark{ mut duration_ms } => {
				if is_crit { duration_ms = (duration_ms * CRIT_DURATION_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_marked(duration_ms));
			}
			SelfApplier::Move{ direction } => {
				let direction: isize = match *direction {
					MoveDirection::ToCenter(amount) => { -1 * amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let mut allies_space_occupied = 0;
				for ally in iter_allies_of!(caster, others) {
					allies_space_occupied += ally.position().size();
				}

				let index_current : &mut usize = caster.position.order_mut();
				let index_old = *index_current as isize;
				*index_current = usize::clamp(((*index_current as isize) + direction) as usize, 0, allies_space_occupied);
				let index_delta = *index_current as isize - index_old;
				let inverse_delta = -1 * index_delta;

				for ally in iter_mut_allies_of!(caster, others) {
					let order = ally.position_mut().order_mut();
					*order = (*order as isize + inverse_delta) as usize;
				}
			}
			SelfApplier::PersistentHeal{ duration_ms, mut heal_per_sec } => {
				if is_crit { heal_per_sec = (heal_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, heal_per_sec));
			}
			SelfApplier::Riposte{ duration_ms, dmg_multiplier, acc, crit } => {
				caster.persistent_effects.push( PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc, crit.clone()));
			}
			SelfApplier::Summon{ .. } => {} //todo!
		}
	}
}