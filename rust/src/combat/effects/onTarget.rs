use std::collections::HashMap;
use gdnative::log::godot_warn;
use rand::prelude::StdRng;
use rand::Rng;
use crate::{iter_allies_of, iter_mut_allies_of};
use crate::combat::effects::MoveDirection;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::Entity;
use crate::combat::entity::position::Position;
use crate::combat::ModifiableStat;
use crate::combat::ModifiableStat::{DEBUFF_RATE, DEBUFF_RES, MOVE_RATE, MOVE_RES, STUN_DEF};
use crate::combat::skills::CRITMode;
use crate::util::{Base100ChanceGenerator, GUID};
use crate::util::bounded_integer_traits_U32::ToBounded;

pub(super) const CRIT_DURATION_MULTIPLIER: i64 = 150;
pub(super) const CRIT_EFFECT_MULTIPLIER: usize = 150;
pub(super) const CRIT_EFFECT_MULTIPLIER_I: isize = CRIT_EFFECT_MULTIPLIER as isize;
pub(super) const CRIT_CHANCE_MODIFIER  : isize = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetApplier {
	Arouse {
		duration_ms: i64,
		lust_per_sec: usize,
	},
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		modifier: isize,
		apply_chance: Option<isize>,
	},
	MakeSelfGuardTarget { 
		duration_ms: i64, 
	},
	MakeTargetGuardSelf { 
		duration_ms: i64,
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
		apply_chance: Option<isize>,
	},
	PersistentHeal {
		duration_ms: i64,
		heal_per_sec: usize,
	},
	Poison {
		duration_ms: i64,
		dmg_per_sec: usize,
	},
	MakeTargetRiposte {
		duration_ms: i64,
		dmg_multiplier: isize,
		acc: isize,
		crit: CRITMode,
	},
	Stun {
		force: isize,
	},
	Tempt { 
		intensity: isize,
	},
}

impl TargetApplier {
	pub fn apply_target(&self, caster: &mut CombatCharacter, target: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, is_crit: bool) {
		match self {
			TargetApplier::Arouse { duration_ms, mut lust_per_sec } => {
				if is_crit { lust_per_sec = (lust_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::new_arousal(*duration_ms, lust_per_sec));
			},
			TargetApplier::Buff{ duration_ms, stat, mut modifier, apply_chance } => {
				match apply_chance {
					Some(mut chance) if Position::opposite_side(&caster.position, &target.position) => { //apply chance is only used when the caster and target are enemies
						if is_crit { chance += CRIT_CHANCE_MODIFIER;  }

						let final_chance = chance + caster.stat(DEBUFF_RATE) - target.stat(DEBUFF_RES);
						if seed.base100_chance(final_chance.bind_0_p100()) == false {
							return;
						}
					},
					_ => { }
				}
				
				if is_crit { modifier = (modifier * CRIT_EFFECT_MULTIPLIER_I) / 100; }

				target.persistent_effects.push(PersistentEffect::new_buff(*duration_ms, *stat, modifier));
			},
			TargetApplier::Heal{ mut base_multiplier } => {
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER_I) / 100;  }
				
				let max: isize = caster.damage.max.max(0);
				let min: isize = caster.damage.min.clamp(0, max);

				let healAmount: isize;

				if max <= 0 {
					return;
				} else if max == min {
					healAmount = max;
				} else {
					healAmount = (seed.gen_range(min..=max) * base_multiplier) / 100;
				}

				target.stamina_cur = (target.stamina_cur + healAmount).clamp(0, target.stamina_max);
			},
			TargetApplier::Lust{ mut min, mut max } => {
				if is_crit {
					min = (min * CRIT_EFFECT_MULTIPLIER) / 100;
					max = (max * CRIT_EFFECT_MULTIPLIER) / 100;
				}
				
				match &mut target.girl_stats {
					None => {
						return;
					}
					Some(girl) => {
						let actual_min: usize = usize::min(min,max - 1);
						let lustAmount: usize = seed.gen_range(actual_min..=max);
						girl.lust += lustAmount as isize;
					}
				}
			},
			TargetApplier::Mark{ mut duration_ms } => {
				if is_crit { duration_ms = (duration_ms * CRIT_DURATION_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::new_marked(duration_ms));
			},
			TargetApplier::Move{ direction, apply_chance } => {
				match apply_chance {
					Some(mut chance) if Position::opposite_side(&caster.position, &target.position) => { //apply chance is only used when the caster and target are enemies
						if is_crit { chance += CRIT_CHANCE_MODIFIER;  }
						
						let final_chance = chance + caster.stat(MOVE_RATE) - target.stat(MOVE_RES);
						if seed.base100_chance(final_chance.bind_0_p100()) == false {
							return;
						}
					}
					_ => {}
				}

				let direction: isize = match direction {
					MoveDirection::ToCenter(amount) => { -1 * amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};


				let mut allies_space_occupied = 0;
				for target_ally in iter_allies_of!(target, others) {
					allies_space_occupied += target_ally.position().size();
				}

				let order_current : &mut usize = target.position.order_mut();
				let order_old = *order_current as isize;
				*order_current = usize::clamp(((*order_current as isize) + direction) as usize, 0, allies_space_occupied);
				let order_delta = *order_current as isize - order_old;
				let inverse_delta = -1 * order_delta;

				for target_ally in iter_mut_allies_of!(target, others) {
					let order = target_ally.position_mut().order_mut();
					*order = (*order as isize + inverse_delta) as usize;
				}
			},
			TargetApplier::PersistentHeal{ duration_ms, mut heal_per_sec } => {
				if is_crit { heal_per_sec = (heal_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, heal_per_sec));
			},
			TargetApplier::Poison{ duration_ms, mut dmg_per_sec } => {
				if is_crit { dmg_per_sec = (dmg_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::new_poison(*duration_ms, dmg_per_sec, caster));
			},
			TargetApplier::MakeTargetRiposte{ duration_ms, dmg_multiplier, acc, crit } => { // can't crit!
				target.persistent_effects.push(PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc, crit.clone()));
			},
			TargetApplier::Stun{ mut force } => {
				if is_crit { force += CRIT_CHANCE_MODIFIER; }
				
				let force = force as f64;
				let def = target.stat(STUN_DEF) as f64;

				let dividend = force + (force * force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (force * 0.25) + (def * 0.25) + (force * def * 0.0005);

				let bonus_redundancy_ms = ((dividend / divisor) * 4000.0) as i64;

				if bonus_redundancy_ms > 0 {
					match &mut target.stun_redundancy_ms {
						None => { target.stun_redundancy_ms = Some(bonus_redundancy_ms); },
						Some(remaining) => { *remaining += bonus_redundancy_ms; },
					};
				}
			},
			TargetApplier::MakeSelfGuardTarget { duration_ms } => { //can't crit!
				target.persistent_effects.push(PersistentEffect::new_guarded(*duration_ms, caster));
			},
			TargetApplier::MakeTargetGuardSelf { duration_ms } => { //can't crit!
				caster.persistent_effects.push(PersistentEffect::new_guarded(*duration_ms, caster));
			},
			TargetApplier::Tempt{ .. } => {} //todo!
		}
	}
	
	pub fn apply_self(&self, caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, is_crit: bool) {
		match self {
			TargetApplier::Arouse { duration_ms, mut lust_per_sec } => {
				if is_crit { lust_per_sec = (lust_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_arousal(*duration_ms, lust_per_sec));
			},
			TargetApplier::Buff { duration_ms, stat, mut modifier, .. } => { // apply_chance is ignored when applied to self
				if is_crit { modifier = (modifier * CRIT_EFFECT_MULTIPLIER_I) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_buff(*duration_ms, *stat, modifier));
			},
			TargetApplier::Heal { mut base_multiplier } => {
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER_I) / 100; }
				
				let max: isize = caster.damage.max.max(0);
				let min: isize = caster.damage.min.clamp(0, max);
				
				let healAmount: isize;
				
				if max <= 0 {
					return;
				} else if max == min {
					healAmount = max;
				} else {
					healAmount = (seed.gen_range(min..=max) * base_multiplier) / 100;
				}

				caster.stamina_cur = (caster.stamina_cur + healAmount).clamp(0, caster.stamina_max);
			},
			TargetApplier::Lust { mut min, mut max } => {
				if is_crit {
					min = (min * CRIT_EFFECT_MULTIPLIER) / 100;
					max = (max * CRIT_EFFECT_MULTIPLIER) / 100;
				}

				if let Some(girl) = &mut caster.girl_stats {
					let actual_min: usize = usize::min(min, max - 1);
					let lustAmount: usize = seed.gen_range(actual_min..=max);
					girl.lust += lustAmount as isize;
				}
			},
			TargetApplier::Mark { mut duration_ms } => {
				if is_crit { duration_ms = (duration_ms * CRIT_DURATION_MULTIPLIER) / 100; }
				
				caster.persistent_effects.push(PersistentEffect::new_marked(duration_ms));
			},
			TargetApplier::Move { direction, .. } => { //apply chance ignored when applied to self
				let direction: isize = match direction {
					MoveDirection::ToCenter(amount) => { -1 * amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let mut allies_space_occupied = 0;
				for ally in iter_allies_of!(caster, others) {
					allies_space_occupied += ally.position().size();
				}

				let order_current: &mut usize = caster.position.order_mut();
				let order_old = *order_current as isize;
				*order_current = usize::clamp(((*order_current as isize) + direction) as usize, 0, allies_space_occupied);
				let order_delta = *order_current as isize - order_old;
				let inverse_delta = -1 * order_delta;

				for ally in iter_mut_allies_of!(caster, others) {
					let order = ally.position_mut().order_mut();
					*order = (*order as isize + inverse_delta) as usize;
				}
			},
			TargetApplier::PersistentHeal { duration_ms, mut heal_per_sec } => {
				if is_crit { heal_per_sec = (heal_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_heal(*duration_ms, heal_per_sec));
			},
			TargetApplier::Poison { duration_ms, mut dmg_per_sec } => {
				if is_crit { dmg_per_sec = (dmg_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::new_poison(*duration_ms, dmg_per_sec, caster));
			},
			TargetApplier::MakeTargetRiposte { duration_ms, dmg_multiplier, acc, crit } => {
				caster.persistent_effects.push(PersistentEffect::new_riposte(*duration_ms, *dmg_multiplier, *acc, crit.clone()));
			},
			TargetApplier::Stun { mut force } => {
				if is_crit { force += CRIT_CHANCE_MODIFIER; }

				let force = force as f64;
				let def = caster.stat(STUN_DEF) as f64;

				let dividend = force + (force * force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (force * 0.25) + (def * 0.25) + (force * def * 0.0005);

				let bonus_redundancy_ms = ((dividend / divisor) * 4000.0) as i64;

				if bonus_redundancy_ms > 0 {
					match &mut caster.stun_redundancy_ms {
						None => { caster.stun_redundancy_ms = Some(bonus_redundancy_ms); }
						Some(remaining) => { *remaining += bonus_redundancy_ms; }
					};
				}
			},
			// we don't use the default case because we want to be warned about any new effects that are not yet implemented
			TargetApplier::MakeSelfGuardTarget { .. } => godot_warn!("Warning: MakeSelfGuardTarget effect is not applicable to self! Caster: {:?}", caster),
			TargetApplier::MakeTargetGuardSelf { .. } => godot_warn!("Warning: MakeTargetGuardSelf effect is not applicable to self! Caster: {:?}", caster),
			TargetApplier::Tempt { .. } => godot_warn!("Warning: Tempt effect is not applicable to self! Caster: {:?}", caster),
		}
	}
}