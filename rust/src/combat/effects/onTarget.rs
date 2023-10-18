use std::collections::HashMap;
use gdnative::log::godot_warn;
use rand::prelude::StdRng;
use rand::Rng;
use proc_macros::get_perk;
use crate::{iter_allies_of, iter_mut_allies_of};
use crate::combat::effects::MoveDirection;
use crate::combat::effects::persistent::{PersistentDebuff, PersistentEffect};
use crate::combat::entity::character::*;
use crate::combat::entity::data::character::CharacterData;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::*;
use crate::combat::entity::Entity;
use crate::combat::entity::position::Position;
use crate::combat::ModifiableStat;
use crate::combat::ModifiableStat::{DEBUFF_RATE, DEBUFF_RES, MOVE_RATE, MOVE_RES, STUN_DEF};
use crate::combat::perk::Perk;
use crate::combat::skill_types::CRITMode;
use crate::util::{Base100ChanceGenerator, GUID};

pub(super) const CRIT_DURATION_MULTIPLIER: i64 = 150;
pub(super) const CRIT_EFFECT_MULTIPLIER: usize = 150;
pub(super) const CRIT_EFFECT_MULTIPLIER_I: isize = CRIT_EFFECT_MULTIPLIER as isize;
pub(super) const CRIT_CHANCE_MODIFIER  : isize = 50;

#[derive(Debug, Clone)]
pub enum TargetApplier {
	Arouse {
		duration_ms: i64,
		lust_per_sec: usize,
	},
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_increase: usize,
	},
	Debuff(DebuffApplier),
	ChangeExhaustion {
		delta: isize,
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
	TemporaryPerk {
		duration_ms: i64,
		perk: Perk,
	},
	Tempt {
		intensity: isize,
	},
}

#[derive(Debug, Clone)]
pub enum DebuffApplier {
	Standard {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_decrease: usize,
		apply_chance: Option<isize>,
	},
	StaggeringForce {
		duration_ms: i64,
		apply_chance: Option<isize>,
	},
}

impl TargetApplier {

	/// returns target if it's still standing
	#[must_use]
	pub fn apply_target(&self, caster: &mut CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, is_crit: bool) -> Option<CombatCharacter> {
		match self {
			TargetApplier::Arouse { duration_ms, mut lust_per_sec } => {
				if is_crit { lust_per_sec = (lust_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::Arousal { duration_ms: *duration_ms, accumulated_ms: 0, lust_per_sec });
				return Some(target);
			},
			TargetApplier::Buff{ duration_ms, stat, mut stat_increase } => {
				if is_crit { stat_increase = (stat_increase * CRIT_EFFECT_MULTIPLIER) / 100; }

				target.persistent_effects.push(PersistentEffect::Buff { duration_ms:*duration_ms, stat: *stat, stat_increase });
				return Some(target);
			},
			TargetApplier::Debuff(DebuffApplier::Standard { duration_ms, mut stat, mut stat_decrease, apply_chance }) => {
				match apply_chance {
					Some(mut chance) if Position::is_opposite_side(&caster.position, &target.position) => { //apply chance is only used when the caster and target are enemies
						if is_crit { chance += CRIT_CHANCE_MODIFIER;  }

						let final_chance = chance + caster.get_stat(DEBUFF_RATE) - target.get_stat(DEBUFF_RES);
						if seed.base100_chance(final_chance.into()) == false {
							return Some(target);
						}
					},
					_ => { }
				}

				if is_crit { stat_decrease = (stat_decrease * CRIT_EFFECT_MULTIPLIER) / 100; }

				if let Some(Perk::Ethel(EthelPerk::Bruiser_DisruptiveManeuvers)) = get_perk!(target, Perk::Ethel(EthelPerk::Bruiser_DisruptiveManeuvers)) {
					stat = ModifiableStat::get_non_girl_random_except(seed, stat);

					let apply_to_caster = ModifiableStat::get_non_girl_random_except(seed, stat);
					caster.persistent_effects.push(PersistentEffect::Debuff(PersistentDebuff::Standard { duration_ms: *duration_ms, stat: apply_to_caster, stat_decrease }));
				}

				if let Some(Perk::Ethel(EthelPerk::Debuffer_WhatDoesntKillYou)) = get_perk!(target, Perk::Ethel(EthelPerk::Debuffer_WhatDoesntKillYou)) {
					let random_buff = PersistentEffect::Buff {
						duration_ms: *duration_ms,
						stat: ModifiableStat::get_non_girl_random_except(seed,stat),
						stat_increase: stat_decrease,
					};

					target.persistent_effects.push(random_buff);
				}

				target.persistent_effects.push(PersistentEffect::Debuff(PersistentDebuff::Standard { duration_ms: *duration_ms, stat, stat_decrease }));
				return Some(target);
			},
			TargetApplier::Debuff(DebuffApplier::StaggeringForce { duration_ms, apply_chance }) => {
				match apply_chance {
					Some(mut chance) if Position::is_opposite_side(&caster.position, &target.position) => { //apply chance is only used when the caster and target are enemies
						if is_crit { chance += CRIT_CHANCE_MODIFIER;  }

						let final_chance = chance + caster.get_stat(DEBUFF_RATE) - target.get_stat(DEBUFF_RES);
						if seed.base100_chance(final_chance.into()) == false {
							return Some(target);
						}
					},
					_ => { }
				}

				target.persistent_effects.push(PersistentEffect::Debuff(PersistentDebuff::StaggeringForce { duration_ms: *duration_ms }));
				return Some(target);
			},
			TargetApplier::ChangeExhaustion { delta } => { // ignores crit
				let Some(girl) = &mut target.girl_stats else { return Some(target); };
				girl.exhaustion += *delta;
				return Some(target);
			},
			TargetApplier::Heal{ mut base_multiplier } => {
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER_I) / 100;  }
				
				let max: isize = caster.dmg.max.max(0);
				let min: isize = caster.dmg.min.clamp(0, max);

				let mut healAmount: isize;

				if max <= 0 {
					return Some(target);
				} else if max == min {
					healAmount = (max * base_multiplier) / 100;
				} else {
					healAmount = (seed.gen_range(min..=max) * base_multiplier) / 100;
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if target.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						healAmount = (healAmount * 130) / 100;
					}
				}

				target.stamina_cur = CombatCharacter::clamp_stamina(target.stamina_cur + healAmount, target.get_max_stamina());
				return Some(target);
			},
			TargetApplier::Lust{ mut min, mut max } => {
				if is_crit {
					min = (min * CRIT_EFFECT_MULTIPLIER) / 100;
					max = (max * CRIT_EFFECT_MULTIPLIER) / 100;
				}

				if let Some(girl) = &mut target.girl_stats {
					let actual_min: usize = usize::min(min, max - 1);
					let lustAmount: usize = seed.gen_range(actual_min..=max);
					girl.lust += lustAmount as isize;
				}

				return Some(target);
			},
			TargetApplier::Mark{ mut duration_ms } => {
				if is_crit { duration_ms = (duration_ms * CRIT_DURATION_MULTIPLIER) / 100; }
				
				target.persistent_effects.push(PersistentEffect::Marked { duration_ms });
				return Some(target);
			},
			TargetApplier::Move{ direction, apply_chance } => {
				match apply_chance {
					Some(mut chance) if Position::is_opposite_side(&caster.position, &target.position) => { //apply chance is only used when the caster and target are enemies
						if is_crit { chance += CRIT_CHANCE_MODIFIER;  }
						
						let final_chance = chance + caster.get_stat(MOVE_RATE) - target.get_stat(MOVE_RES);
						if seed.base100_chance(final_chance.into()) == false {
							return Some(target);
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

				return Some(target);
			},
			TargetApplier::PersistentHeal{ duration_ms, mut heal_per_sec } => {
				if is_crit { heal_per_sec = (heal_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if target.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						heal_per_sec = (heal_per_sec * 130) / 100;
					}
				}
				
				target.persistent_effects.push(PersistentEffect::Heal { duration_ms: *duration_ms, accumulated_ms: 0, heal_per_sec });
				return Some(target);
			},
			TargetApplier::Poison{ mut duration_ms, mut dmg_per_sec } => {
				if is_crit { dmg_per_sec = (dmg_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				if let Some(Perk::Ethel(EthelPerk::Poison_LingeringToxins)) = get_perk!(caster, Perk::Ethel(EthelPerk::Poison_LingeringToxins)) {
					duration_ms += 1;
				}
				
				target.persistent_effects.push(PersistentEffect::Poison { duration_ms, accumulated_ms: 0, dmg_per_sec, caster_guid: caster.guid() });
				return Some(target);
			},
			TargetApplier::MakeTargetRiposte{ duration_ms, mut dmg_multiplier, acc, crit } => { // can't crit!
				if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(target, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
					dmg_multiplier += 30;
				}

				target.persistent_effects.push(PersistentEffect::Riposte { duration_ms: *duration_ms, dmg_multiplier, acc: *acc, crit: *crit });
				return Some(target);
			},
			TargetApplier::Stun{ mut force } => {
				if is_crit { force += CRIT_CHANCE_MODIFIER; }
				
				let force = force as f64;
				let def = target.get_stat(STUN_DEF) as f64;

				let dividend = force + (force * force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (force * 0.25) + (def * 0.25) + (force * def * 0.0005);

				let bonus_redundancy_ms = ((dividend / divisor) * 4000.0) as i64;

				if bonus_redundancy_ms > 0 {
					match &mut target.stun_redundancy_ms {
						Some(remaining) => { *remaining += bonus_redundancy_ms; },
						None => { target.stun_redundancy_ms = Some(bonus_redundancy_ms); },
					};
				}

				return Some(target);
			},
			TargetApplier::MakeSelfGuardTarget { duration_ms } => { //can't crit!
				target.persistent_effects.push(PersistentEffect::Guarded { duration_ms: *duration_ms, guarder_guid: caster.guid });
				return Some(target);
			},
			TargetApplier::MakeTargetGuardSelf { duration_ms } => { //can't crit!
				caster.persistent_effects.push(PersistentEffect::Guarded { duration_ms: *duration_ms, guarder_guid: target.guid });
				return Some(target);
			},
			TargetApplier::TemporaryPerk { duration_ms, perk } => {
				target.persistent_effects.push(PersistentEffect::TemporaryPerk { duration_ms: *duration_ms, perk: perk.clone() });
				return Some(target);
			}
			TargetApplier::Tempt{ intensity } => {
				let Some(girl) = &mut target.girl_stats else {
					godot_warn!("Warning: Trying to apply tempt to character {target:?}, but it's not a girl.");
					return Some(target);
				};

				let lust_squared = girl.lust.get() as f64 * girl.lust.get() as f64;
				let extra_intensity_from_lust = lust_squared / 500.0;
				let multiplier_from_lust = 1.0 + (lust_squared / 80000.0);

				let intensity_f64 = (*intensity as f64 + extra_intensity_from_lust) * multiplier_from_lust;
				let composure_f64 = girl.composure.get() as f64;

				let dividend = 10.0 * (intensity_f64 + (intensity_f64 * intensity_f64 / 500.0) - composure_f64 - (composure_f64 * composure_f64 / 500.0));
				let divisor = 125.0 + (intensity_f64 * 0.25) + (composure_f64 * 0.25) + (intensity_f64 * composure_f64 * 0.0005);

				let temptation_delta = (dividend / divisor) as isize;
				
				if temptation_delta <= 0 {
					return Some(target);
				}

				girl.temptation += temptation_delta;
				if girl.temptation < 100 {
					return Some(target);
				}

				let CharacterData::NPC(_) = caster.data else { return Some(target); };  // making sure caster is an npc (required for grappling)

				if target.can_be_grappled() == false {
					return Some(target);
				}

				let grappled_girl = target.into_grappled_unchecked();

				caster.state = CharacterState::Grappling(State_Grappling {
					victim: grappled_girl,
					lust_per_sec: 45,
					temptation_per_sec: -5,
					duration_ms: 5000,
					accumulated_ms: 0,
				});

				return None;
			}
		}
	}
	
	pub fn apply_self(&self, caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, is_crit: bool) {
		match self {
			TargetApplier::Arouse { duration_ms, mut lust_per_sec } => {
				if is_crit { lust_per_sec = (lust_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::Arousal { duration_ms: *duration_ms, accumulated_ms: 0, lust_per_sec });
			},
			TargetApplier::Buff { duration_ms, stat, mut stat_increase, .. } => {
				if is_crit { stat_increase = (stat_increase * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::Buff { duration_ms: *duration_ms, stat: *stat, stat_increase });
			},
			TargetApplier::Debuff(DebuffApplier::Standard { duration_ms, stat, mut stat_decrease, .. }) => { // apply chance is ignored on self
				if is_crit { stat_decrease = (stat_decrease * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::Debuff(PersistentDebuff::Standard {
					duration_ms: *duration_ms,
					stat: *stat,
					stat_decrease,
				}))
			},
			TargetApplier::Debuff(DebuffApplier::StaggeringForce { duration_ms, .. }) => {
				caster.persistent_effects.push(PersistentEffect::Debuff(PersistentDebuff::StaggeringForce {
					duration_ms: *duration_ms,
				}))
			},
			TargetApplier::ChangeExhaustion { delta } => { // ignores crit
				if let Some(girl) = &mut caster.girl_stats {
					girl.exhaustion += *delta;
				}
			},
			TargetApplier::Heal { mut base_multiplier } => {
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER_I) / 100; }
				
				let max: isize = caster.dmg.max.max(0);
				let min: isize = caster.dmg.min.clamp(0, max);
				
				let mut healAmount: isize;
				
				if max <= 0 {
					return;
				} else if max == min {
					healAmount = (max * base_multiplier) / 100;
				} else {
					healAmount = (seed.gen_range(min..=max) * base_multiplier) / 100;
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if caster.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						healAmount = (healAmount * 130) / 100;
					}
				}

				caster.stamina_cur = (caster.stamina_cur + healAmount).clamp(0, caster.get_max_stamina());
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
				
				caster.persistent_effects.push(PersistentEffect::Marked { duration_ms });
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

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if caster.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						heal_per_sec = (heal_per_sec * 130) / 100;
					}
				}

				caster.persistent_effects.push(PersistentEffect::Heal{ duration_ms: *duration_ms, accumulated_ms: 0, heal_per_sec });
			},
			TargetApplier::Poison { mut duration_ms, mut dmg_per_sec } => {
				if is_crit { dmg_per_sec = (dmg_per_sec * CRIT_EFFECT_MULTIPLIER) / 100; }

				if let Some(Perk::Ethel(EthelPerk::Poison_LingeringToxins)) = get_perk!(caster, Perk::Ethel(EthelPerk::Poison_LingeringToxins)) {
					duration_ms += 1;
				}

				if let Some(Perk::Ethel(EthelPerk::Poison_ConcentratedToxins)) = get_perk!(caster, Perk::Ethel(EthelPerk::Poison_ConcentratedToxins)) {
					dmg_per_sec = (dmg_per_sec * 125) / 100;
				}

				caster.persistent_effects.push(PersistentEffect::Poison{ duration_ms, accumulated_ms: 0, dmg_per_sec, caster_guid: caster.guid() });
			},
			TargetApplier::MakeTargetRiposte { duration_ms, mut dmg_multiplier, acc, crit } => {
				if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(caster, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
					dmg_multiplier += 30;
				}

				caster.persistent_effects.push(PersistentEffect::Riposte{ duration_ms: *duration_ms, dmg_multiplier, acc: *acc, crit: *crit });
			},
			TargetApplier::Stun { mut force } => {
				if is_crit { force += CRIT_CHANCE_MODIFIER; }

				let force = force as f64;
				let def = caster.get_stat(STUN_DEF) as f64;

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
			TargetApplier::TemporaryPerk { duration_ms, perk } => { // can't crit
				caster.persistent_effects.push(PersistentEffect::TemporaryPerk { duration_ms: *duration_ms, perk: perk.clone() });
			}
			// we don't use the default case because we want to be warned about any new effects that are not yet implemented
			TargetApplier::MakeSelfGuardTarget { .. } => godot_warn!("Warning: MakeSelfGuardTarget effect is not applicable to self! Caster: {:?}", caster),
			TargetApplier::MakeTargetGuardSelf { .. } => godot_warn!("Warning: MakeTargetGuardSelf effect is not applicable to self! Caster: {:?}", caster),
			TargetApplier::Tempt { .. } => godot_warn!("Warning: Tempt effect is not applicable to self! Caster: {:?}", caster),
		}
	}
}