use std::collections::{HashMap, HashSet};
use std::num::{NonZeroI8, NonZeroU16, NonZeroU8};

use comfy_bounded_ints::prelude::{SqueezeTo, SqueezeTo_i8, SqueezeTo_u8};
use comfy_bounded_ints::types::Bound_u8;
use gdnative::log::godot_warn;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use util::any_matches;
use uuid::Uuid;

use combat::effects::MoveDirection;
use combat::effects::persistent::{PersistentDebuff, PersistentEffect, PoisonAdditive};
use combat::entity::{iter_allies_of, iter_mut_allies_of};
use combat::entity::character::*;
use combat::entity::data::character::CharacterData;
use combat::entity::data::girls::ethel::perks::*;
use combat::entity::data::girls::nema::perks::*;
use combat::entity::Entity;
use combat::entity::position::Position;
use combat::entity::stat::*;
use combat::perk::{get_perk, get_perk_mut};
use combat::perk::Perk;
use combat::skill_types::{ACCMode, CRITMode};

use crate::combat;
use crate::combat::effects::IntervalMS;
use crate::misc::{Base100ChanceGenerator, SaturatedU64, ToSaturatedI64, ToSaturatedU64, ToU8Percentage};

pub(super) const CRIT_DURATION_MULTIPLIER: u64 = 150;
pub(super) const CRIT_EFFECT_MULTIPLIER: u64 = 150;
pub(super) const CRIT_CHANCE_MODIFIER  : u16 = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetApplier {
	Arouse {
		duration_ms: SaturatedU64,
		lust_per_interval: NonZeroU8,
	},
	Buff {
		duration_ms: SaturatedU64,
		stat: DynamicStat,
		stat_increase: NonZeroU16,
	},
	Debuff {
		duration_ms: SaturatedU64,
		apply_chance: Option<NonZeroU16>,
		applier_kind: DebuffApplierKind
	},
	ChangeExhaustion {
		delta: NonZeroI8,
	},
	MakeSelfGuardTarget { 
		duration_ms: SaturatedU64, 
	},
	MakeTargetGuardSelf { 
		duration_ms: SaturatedU64,
	},
	Heal { 
		multiplier: NonZeroU16,
	},
	Lust { 
		delta: CheckedRange
	},
	Mark { 
		duration_ms: SaturatedU64,
	},
	Move {
		direction: MoveDirection,
		apply_chance: Option<NonZeroU16>,
	},
	PersistentHeal {
		duration_ms: SaturatedU64,
		heal_per_interval: NonZeroU8,
	},
	Poison {
		duration_ms: SaturatedU64,
		poison_per_interval: NonZeroU8,
		apply_chance: Option<NonZeroU16>,
		additives: HashSet<PoisonAdditive>,
	},
	MakeTargetRiposte {
		duration_ms: SaturatedU64,
		skill_power: Power,
		acc_mode: ACCMode,
		crit_mode: CRITMode,
	},
	Stun {
		force:NonZeroU16,
	},
	TemporaryPerk {
		duration_ms: SaturatedU64,
		perk: Perk,
	},
	Tempt {
		intensity: NonZeroU16,
	},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DebuffApplierKind {
	Standard {
		stat: DynamicStat,
		stat_decrease: NonZeroU16,
	},
	StaggeringForce,
}

impl TargetApplier {
	//noinspection RsLift
	/// returns target if it's still standing
	#[must_use]
	pub fn apply_target(&self, caster: &mut CombatCharacter, mut target: CombatCharacter,
	                    others: &mut HashMap<Uuid, Entity>, rng: &mut Xoshiro256PlusPlus, is_crit: bool) -> Option<CombatCharacter> {
		match self {
			TargetApplier::Arouse { duration_ms, lust_per_interval: lust_per_sec } => {
				if target.girl_stats.is_none() {
					godot_warn!("{}(): Trying to apply Arouse on non-girl character: {target:?}",
						util::full_fn_name(&Self::apply_target));
					return Some(target);
				};
				
				let final_lust_per_interval_option = {
					let mut temp = lust_per_sec.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU8::new(temp.squeeze_to())
				};
				final_lust_per_interval_option.map(|final_lust_per_interval| {
					let effect = PersistentEffect::Arousal {
						duration_ms: *duration_ms,
						accumulated_ms: 0.to_sat_u64(),
						lust_per_interval: final_lust_per_interval,
					};

					target.persistent_effects.push(effect);
				});
				
				return Some(target);
			},
			TargetApplier::Buff { duration_ms, stat, stat_increase } => {
				let final_stat_increase_option = {
					let mut temp = stat_increase.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU16::new(temp.squeeze_to())
				};
				final_stat_increase_option.map(|final_stat_increase| {
					let effect = PersistentEffect::Buff {
						duration_ms: *duration_ms,
						stat: *stat,
						stat_increase: final_stat_increase
					};

					target.persistent_effects.push(effect);
				});
				
				return Some(target);
			},
			TargetApplier::Debuff { duration_ms, apply_chance, 
				applier_kind: DebuffApplierKind::Standard { mut stat, stat_decrease } } => {
				if let Some(chance) = apply_chance
					&& Position::is_opposite_side(&caster.position, &target.position) //apply chance is only used when the caster and target are enemies 
				{ 
					let final_chance = {
						let mut temp = chance.get().to_sat_i64();
						temp += caster.dyn_stat::<DebuffRate>().get();
						temp -= target.dyn_stat::<DebuffRes>().get();
						if is_crit {
							temp += CRIT_CHANCE_MODIFIER;
						}
						Bound_u8::new(temp.squeeze_to())
					};
					
					if !rng.base100_chance(final_chance) { // roll failed
						return Some(target);
					}
				}

				let final_stat_decrease_option = {
					let mut temp = stat_decrease.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU16::new(temp.squeeze_to())
				};
				let Some(final_stat_decrease) = final_stat_decrease_option
					else { return Some(target); };
				
				if let Some(Perk::Ethel(EthelPerk::Bruiser_DisruptiveManeuvers)) = get_perk!(target, Perk::Ethel(EthelPerk::Bruiser_DisruptiveManeuvers)) {
					stat = DynamicStat::get_random_except(rng, stat);
					let stat_to_caster = DynamicStat::get_random_except(rng, stat);
					let random_debuff = PersistentEffect::Debuff {
						duration_ms: *duration_ms,
						debuff_kind: PersistentDebuff::Standard { 
							stat: stat_to_caster, 
							stat_decrease: final_stat_decrease
						},
					};

					caster.persistent_effects.push(random_debuff);
				}

				if let Some(Perk::Ethel(EthelPerk::Debuffer_WhatDoesntKillYou)) = get_perk!(target, Perk::Ethel(EthelPerk::Debuffer_WhatDoesntKillYou)) {
					let random_buff = PersistentEffect::Buff {
						duration_ms: *duration_ms,
						stat: DynamicStat::get_random_except(rng, stat),
						stat_increase: final_stat_decrease,
					};

					target.persistent_effects.push(random_buff);
				}

				let effect = PersistentEffect::Debuff {
					duration_ms: *duration_ms,
					debuff_kind: PersistentDebuff::Standard { 
						stat, 
						stat_decrease: final_stat_decrease
					},
				};

				target.persistent_effects.push(effect);
				
				return Some(target);
			},
			TargetApplier::Debuff { duration_ms, apply_chance, 
				applier_kind: DebuffApplierKind::StaggeringForce } => {
				if let Some(chance) = apply_chance
					&& Position::is_opposite_side(&caster.position, &target.position) //apply chance is only used when the caster and target are enemies 
				{ 
					let final_chance = {
						let mut temp = chance.get().to_sat_i64();
						temp += caster.dyn_stat::<DebuffRate>().get();
						temp -= target.dyn_stat::<DebuffRes>().get();
						if is_crit {
							temp += CRIT_CHANCE_MODIFIER;
						}
						Bound_u8::new(temp.squeeze_to())
					};
					
					if !rng.base100_chance(final_chance) { // roll failed
						return Some(target);
					}
				}

				let effect = PersistentEffect::Debuff {
					duration_ms: *duration_ms,
					debuff_kind: PersistentDebuff::StaggeringForce
				};
				
				target.persistent_effects.push(effect);
				return Some(target);
			},
			TargetApplier::ChangeExhaustion { delta } => { // ignores crit
				let Some(girl) = &mut target.girl_stats
					else {
						godot_warn!("{}(): Trying to change exhaustion of non-girl character: {target:?}",
							util::full_fn_name(&Self::apply_target));
						return Some(target);
					};
				
				*girl.exhaustion += delta.get();
				return Some(target);
			},
			TargetApplier::Heal{ multiplier } => {
				let final_multiplier = {
					let mut temp = multiplier.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					temp
				};
				
				let caster_dmg = caster.dmg;
				let (min, max) = (caster_dmg.bound_lower(), caster_dmg.bound_upper());

				if max == 0 {
					return Some(target);
				}

				let heal_amount_option = {
					let range_result =
						if max == min {
							max
						} else {
							rng.gen_range(min..=max)
						};
					
					let mut temp = range_result.to_sat_i64();
					temp *= final_multiplier;
					temp /= 100;

					if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection))
						&& any_matches!(target.persistent_effects, PersistentEffect::Debuff{..} | PersistentEffect::Poison{..}) {
						temp *= 130;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
						let stacks: u64 = u64::clamp(accumulated_ms.get() / 1000, 0, 8);
						temp *= 100 + stacks * 5;
						temp /= 100;
						accumulated_ms.set(0);
					}
					
					NonZeroU8::new(temp.squeeze_to())
				};
				
				let Some(heal_amount) = heal_amount_option
					else { return Some(target); };

				if let Some(Perk::Nema(NemaPerk::Healer_Adoration)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Adoration)) {
					let toughness_buff = TargetApplier::Buff {
						duration_ms: 4000.to_sat_u64(),
						stat: DynamicStat::Toughness,
						stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
					};

					let target_survived = toughness_buff.apply_target(caster, target, others, rng, false);
					if let Some(target_survived) = target_survived {
						target = target_survived;
					} else {
						godot_warn!("{}(): Healer_Adoration: target died from toughness buff, caster: {caster:?}",
							util::full_fn_name(&Self::apply_target));
						return None;
					}

					if let Some(girl) = &mut caster.girl_stats {
						*girl.lust -= 4;

						let composure_buff = TargetApplier::Buff {
							duration_ms: 4000.to_sat_u64(),
							stat: DynamicStat::Composure,
							stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
						};

						let target_survived = composure_buff.apply_target(caster, target, others, rng, false);
						if let Some(target_survived) = target_survived {
							target = target_survived;
						} else {
							godot_warn!("{}(): Healer_Adoration: target died from composure buff, caster: {caster:?}", 
								util::full_fn_name(&Self::apply_target));
							return None;
						}
					}
				}

				*target.stamina_cur += heal_amount.get();
				
				let max_stamina: u16 = target.max_stamina().get();
				if target.stamina_cur.get() > max_stamina {
					target.stamina_cur.set(max_stamina);
				}
				
				return Some(target);
			},
			TargetApplier::Lust { delta } => {
				let Some(girl) = &mut target.girl_stats
					else { 
						godot_warn!("{}(): Trying to apply lust on-non girl target: {target:?}",
							util::full_fn_name(&Self::apply_target));
						return Some(target);
					};
				
				let (min, max) = {
					let mut min_temp = delta.bound_lower().to_sat_i64();
					let mut max_temp = delta.bound_upper().to_sat_i64();
					
					if is_crit {
						min_temp *= CRIT_EFFECT_MULTIPLIER;
						min_temp /= 100;
						max_temp *= CRIT_EFFECT_MULTIPLIER;
						max_temp /= 100;
					}

					(min_temp.squeeze_to_u8(), max_temp.squeeze_to_u8())
				};
				
				let lust_amount = 
					if min == max { 
						max 
					} else {
						rng.gen_range(min..=max)
					};
				
				*girl.lust += lust_amount;
				return Some(target);
			},
			TargetApplier::Mark{ duration_ms } => {
				let final_duration_ms = {
					let mut temp = duration_ms.to_sat_i64();
					if is_crit {
						temp *= CRIT_DURATION_MULTIPLIER;
						temp /= 100;
					}
					temp.to_sat_u64()
				};
				let effect = PersistentEffect::Marked { 
					duration_ms: final_duration_ms 
				};
				target.persistent_effects.push(effect);
				return Some(target);
			},
			TargetApplier::Move{ direction, apply_chance } => {
				if let Some(chance) = apply_chance
					&& Position::is_opposite_side(&caster.position, &target.position) //apply chance is only used when the caster and target are enemies 
				{
					if let Some(Perk::Ethel(EthelPerk::Tank_Vanguard { cooldown_ms })) = get_perk_mut!(target, Perk::Ethel(EthelPerk::Tank_Vanguard { .. }))
						&& cooldown_ms.get() == 0 {
						cooldown_ms.set(10000);
						return Some(target);
					}
					
					let final_chance = {
						let mut temp = chance.get().to_sat_i64();
						temp += caster.dyn_stat::<MoveRate>().get();
						temp -= target.dyn_stat::<MoveRes>().get();
						if is_crit {
							temp += CRIT_CHANCE_MODIFIER;
						}
						Bound_u8::new(temp.squeeze_to())
					};
					
					if !rng.base100_chance(final_chance) { // roll failed
						return Some(target);
					}
				}

				let direction = match direction {
					MoveDirection::ToCenter(amount) => { -amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let allies_space_occupied = iter_allies_of!(target, others)
					.fold(0, |sum, ally| sum + ally.position().size().get());

				let (order_old, order_current) = {
					let temp: &mut Bound_u8<0, {u8::MAX}> = target.position.order_mut();
					let old_temp = *temp;
					*temp += direction.get();
					if temp.get() > allies_space_occupied {
						temp.set(allies_space_occupied);
					}
					(old_temp.squeeze_to_i8(), temp.squeeze_to_i8())
				};
				
				let order_delta = order_current - order_old;
				let inverse_delta = -1 * order_delta;

				iter_mut_allies_of!(target, others).for_each(|ally|
					*ally.position_mut().order_mut() += inverse_delta);

				return Some(target);
			},
			TargetApplier::PersistentHeal{ duration_ms, heal_per_interval } => {
				let final_heal_per_interval_option = {
					let mut temp = heal_per_interval.get().to_sat_i64();

					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection))
						&& any_matches!(target.persistent_effects, PersistentEffect::Debuff {..} | PersistentEffect::Poison {..}) {
						temp *= 130;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
						let stacks = u64::clamp(accumulated_ms.get() / 1000, 0, 8);
						temp *= 100 + stacks * 5;
						temp /= 100;
						accumulated_ms.set(0);
					}
					
					NonZeroU8::new(temp.squeeze_to())
				};
				let Some(final_heal_per_interval) = final_heal_per_interval_option
					else { return Some(target); };

				if let Some(Perk::Nema(NemaPerk::Healer_Adoration)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Adoration)) {
					let toughness_buff = TargetApplier::Buff {
						duration_ms: 4000.to_sat_u64(),
						stat: DynamicStat::Toughness,
						stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
					};

					let target_option = toughness_buff.apply_target(caster, target, others, rng, false);
					if let Some(target_survived) = target_option {
						target = target_survived;
					} else {
						return None;
					}

					if let Some(girl) = &mut target.girl_stats {
						*girl.lust -= 4;

						let composure_buff = TargetApplier::Buff {
							duration_ms: 4000.to_sat_u64(),
							stat: DynamicStat::Composure,
							stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
						};

						let target_option = composure_buff.apply_target(caster, target, others, rng, false);
						if let Some(target_survived) = target_option {
							target = target_survived;
						} else {
							return None;
						}
					}
				}
				
				let effect = PersistentEffect::Heal {
					duration_ms: *duration_ms,
					accumulated_ms: 0.to_sat_u64(),
					heal_per_interval: final_heal_per_interval
				};

				target.persistent_effects.push(effect);
				return Some(target);
			}
			TargetApplier::Poison{ duration_ms, poison_per_interval, 
				apply_chance, additives } => {
				if let Some(chance) = apply_chance 
					&& Position::is_opposite_side(&caster.position, &target.position) // Apply chance is only used when the caster and target are enemies
				{
					let final_chance = {
						let mut temp = chance.get().to_sat_i64();
						temp += caster.dyn_stat::<PoisonRate>().get();
						temp -= target.dyn_stat::<PoisonRes>().get();
						if is_crit {
							temp += CRIT_CHANCE_MODIFIER;
						}
						temp.to_percent_u8()
					};

					if !rng.base100_chance(final_chance) {
						return Some(target);
					}
				}
				
				let final_poison_per_interval_option = {
					let mut temp = poison_per_interval.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					
					NonZeroU8::new(temp.squeeze_to())
				};
				let Some(final_poison_per_interval) = final_poison_per_interval_option
					else { return Some(target); };

				let (final_duration_ms, interval_ms) =
					if any_matches!(additives, PoisonAdditive::Nema_Madness) {
						let mut temp = duration_ms.to_sat_i64();
						temp /= 2;
						(temp.to_sat_u64(), IntervalMS::new(500))
					} else {
						(*duration_ms, IntervalMS::new(1000))
					};

				let effect = PersistentEffect::Poison {
					duration_ms: final_duration_ms,
					accumulated_ms: 0.to_sat_u64(),
					interval_ms,
					poison_per_interval: final_poison_per_interval,
					caster_guid: caster.guid(),
					additives: additives.clone()
				};
				
				target.persistent_effects.push(effect);
				return Some(target);
			}
			TargetApplier::MakeTargetRiposte{ duration_ms, skill_power, 
				acc_mode, crit_mode } => { // can't crit!
				let final_skill_power_option = {
					let mut temp = skill_power.to_sat_i64();
					if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(target, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
						temp += 30;
					}
					NonZeroU16::new(temp.squeeze_to())
						.map(|non_zero| Power::new(non_zero.get()))
				};
				let Some(final_skill_power) = final_skill_power_option
					else { return Some(target); };

				let effect = PersistentEffect::Riposte {
					duration_ms: *duration_ms,
					skill_power: final_skill_power,
					acc_mode: *acc_mode,
					crit_mode: *crit_mode
				};
				
				target.persistent_effects.push(effect);
				return Some(target);
			}
			TargetApplier::Stun{ force } => {
				let final_force = {
					let mut temp = force.get().to_sat_i64();
					if is_crit {
						temp += CRIT_CHANCE_MODIFIER;
					}
					temp.get() as f64
				};
				
				let def = target.dyn_stat::<StunDef>().get() as f64;

				let dividend = final_force + (final_force * final_force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (final_force * 0.25) + (def * 0.25) + (final_force * def * 0.0005);
				
				let bonus_redundancy_f64 = (dividend / divisor) * 4000.0;
				if bonus_redundancy_f64 > 0. {
					let bonus_redundancy_f64 = f64::clamp(bonus_redundancy_f64, 1., u64::MAX as f64);
					let bonus_redundancy_ms = SaturatedU64::new(bonus_redundancy_f64.round() as u64);
					
					match &mut target.stun_redundancy_ms {
						Some(remaining) => { 
							*remaining += bonus_redundancy_ms;
						},
						None => { 
							target.stun_redundancy_ms = Some(bonus_redundancy_ms);
						},
					};
				}

				return Some(target);
			},
			TargetApplier::MakeSelfGuardTarget { duration_ms } => { //can't crit!
				let effect = PersistentEffect::Guarded { 
					duration_ms: *duration_ms, 
					guarder_guid: caster.guid 
				};
				
				target.persistent_effects.push(effect);
				return Some(target);
			},
			TargetApplier::MakeTargetGuardSelf { duration_ms } => { //can't crit!
				let effect = PersistentEffect::Guarded { 
					duration_ms: *duration_ms, 
					guarder_guid: target.guid 
				};
				
				caster.persistent_effects.push(effect);
				return Some(target);
			},
			TargetApplier::TemporaryPerk { duration_ms, perk } => {
				let effect = PersistentEffect::TemporaryPerk { 
					duration_ms: *duration_ms,
					perk: perk.clone()
				};
				
				target.persistent_effects.push(effect);
				return Some(target);
			}
			TargetApplier::Tempt{ intensity } => {
				let Some(girl) = &mut target.girl_stats
					else {
						godot_warn!("{}():Trying to apply tempt to character {target:?}, but it's not a girl.",
							util::full_fn_name(&Self::apply_target));
						return Some(target);
					};

				let lust_f64 = girl.lust.get() as f64;
				let lust_squared = lust_f64 * lust_f64;
				let extra_intensity_from_lust = lust_squared / 500.0;
				let multiplier_from_lust = 1.0 + (lust_squared / 80000.0);

				let intensity_f64 = (intensity.get() as f64 + extra_intensity_from_lust) * multiplier_from_lust;
				let composure_f64 = girl.composure.get() as f64;

				let dividend = 10.0 * (intensity_f64 + (intensity_f64 * intensity_f64 / 500.0) - composure_f64 - (composure_f64 * composure_f64 / 500.0));
				let divisor = 125.0 + (intensity_f64 * 0.25) + (composure_f64 * 0.25) + (intensity_f64 * composure_f64 * 0.0005);
				
				let temptation_delta_f64 = dividend / divisor;
				
				let temptation_delta: u8 = 
					if temptation_delta_f64 > 0. {
						let temptation_delta_f64 = f64::clamp(temptation_delta_f64, 1., u8::MAX as f64);
						temptation_delta_f64.round() as u8
					} else {
						0
					};
				
				if temptation_delta == 0 {
					return Some(target);
				}

				*girl.temptation += temptation_delta;
				if girl.temptation.get() < 100 {
					return Some(target);
				}

				let CharacterData::NPC(_) = caster.data // making sure caster is a npc (required for grappling) 
					else {
						godot_warn!("{}(): Trying to apply tempt to character {target:?}, but caster {caster:?} isn't an NPC.",
							util::full_fn_name(&Self::apply_target));
						return Some(target);
					};

				if !target.can_be_grappled() {
					return Some(target);
				}

				caster.state = CharacterState::Grappling(GrapplingState {
					victim: target.into_grappled_unchecked(),
					lust_per_interval: unsafe { NonZeroU8::new_unchecked(45) },
					temptation_per_interval: unsafe { NonZeroI8::new_unchecked(-5) },
					duration_ms: 5000.to_sat_u64(),
					accumulated_ms: 0.to_sat_u64(),
				});

				return None;
			}
		}
	}
	
	pub fn apply_self(&self, caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>,
	                  rng: &mut Xoshiro256PlusPlus, is_crit: bool) {
		match self {
			TargetApplier::Arouse { duration_ms, lust_per_interval } => {
				if caster.girl_stats.is_none() {
					godot_warn!("{}(): Trying to apply Arouse on non-girl character: {caster:?}",
						util::full_fn_name(&Self::apply_target));
					return;
				};

				let final_lust_per_interval_option = {
					let mut temp = lust_per_interval.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU8::new(temp.squeeze_to())
				};
				final_lust_per_interval_option.map(|final_lust_per_interval| {
					let effect = PersistentEffect::Arousal {
						duration_ms: *duration_ms,
						accumulated_ms: 0.to_sat_u64(),
						lust_per_interval: final_lust_per_interval,
					};

					caster.persistent_effects.push(effect);
				});
			}
			TargetApplier::Buff { duration_ms, stat, stat_increase, .. } => {
				let final_stat_increase_option = {
					let mut temp = stat_increase.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU16::new(temp.squeeze_to())
				};
				final_stat_increase_option.map(|final_stat_increase|{
					let effect = PersistentEffect::Buff {
						duration_ms: *duration_ms,
						stat: *stat,
						stat_increase: final_stat_increase,
					};

					caster.persistent_effects.push(effect);
				});
			},
			TargetApplier::Debuff { duration_ms, apply_chance: _apply_chance, // apply chance is ignored on self
				applier_kind: DebuffApplierKind::Standard { stat, stat_decrease, .. }} => {
				let final_stat_decrease_option = {
					let mut temp = stat_decrease.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU16::new(temp.squeeze_to())
				};
				final_stat_decrease_option.map(|final_stat_decrease|{
					let effect = PersistentEffect::Debuff {
						duration_ms: *duration_ms,
						debuff_kind: PersistentDebuff::Standard { stat: *stat, stat_decrease: final_stat_decrease },
					};

					caster.persistent_effects.push(effect);
				});
			},
			TargetApplier::Debuff { duration_ms, apply_chance: _apply_chance, // apply chance is ignored on self
				applier_kind: DebuffApplierKind::StaggeringForce } => {
				let effect = PersistentEffect::Debuff {
					duration_ms: *duration_ms,
					debuff_kind: PersistentDebuff::StaggeringForce
				};

				caster.persistent_effects.push(effect);
			},
			TargetApplier::ChangeExhaustion { delta } => { // ignores crit
				let Some(girl) = &mut caster.girl_stats
					else {
						godot_warn!("{}(): Trying to change exhaustion of non-girl character: {caster:?}",
							util::full_fn_name(&Self::apply_self));
						return;
					};

				*girl.exhaustion += delta.get();
			},
			TargetApplier::Heal { multiplier } => {
				let final_multiplier_option = {
					let mut temp = multiplier.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}
					NonZeroU16::new(temp.squeeze_to())
				};
				
				let Some(final_multiplier) = final_multiplier_option
					else { return; };

				let caster_dmg = caster.dmg;
				let (min, max) = (caster_dmg.bound_lower(), caster_dmg.bound_upper());

				if max == 0 {
					return;
				}

				let heal_amount_option = {
					let range_result =
						if max == min {
							max
						} else {
							rng.gen_range(min..=max)
						};

					let mut temp = range_result.to_sat_i64();
					temp *= final_multiplier.get();
					temp /= 100;

					if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection))
						&& any_matches!(caster.persistent_effects, PersistentEffect::Debuff{..} | PersistentEffect::Poison{..}) {
						temp *= 130;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
						let stacks: u64 = u64::clamp(accumulated_ms.get() / 1000, 0, 8);
						temp *= 100 + stacks * 5;
						temp /= 100;
						accumulated_ms.set(0);
					}

					NonZeroU8::new(temp.squeeze_to())
				};
				let Some(heal_amount) = heal_amount_option
					else { return; };

				if let Some(Perk::Nema(NemaPerk::Healer_Adoration)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Adoration)) {
					let toughness_buff = TargetApplier::Buff {
						duration_ms: 4000.to_sat_u64(),
						stat: DynamicStat::Toughness,
						stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
					};

					toughness_buff.apply_self(caster, others, rng, false);

					if let Some(girl) = &mut caster.girl_stats {
						*girl.lust -= 4;

						let composure_buff = TargetApplier::Buff {
							duration_ms: 4000.to_sat_u64(),
							stat: DynamicStat::Composure,
							stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
						};

						composure_buff.apply_self(caster, others, rng, false);
					}
				}

				*caster.stamina_cur += heal_amount.get();
				let max_stamina: u16 = caster.max_stamina().get();
				if caster.stamina_cur.get() > max_stamina {
					caster.stamina_cur.set(max_stamina);
				}
			},
			TargetApplier::Lust { delta } => {
				let Some(girl) = &mut caster.girl_stats
					else {
						godot_warn!("{}(): Trying to apply lust on-non girl self: {caster:?}",
							util::full_fn_name(&Self::apply_self));
						return;
					};

				let (min, max) = {
					let mut min_temp = delta.bound_lower().to_sat_i64();
					let mut max_temp = delta.bound_upper().to_sat_i64();

					if is_crit {
						min_temp *= CRIT_EFFECT_MULTIPLIER;
						min_temp /= 100;
						max_temp *= CRIT_EFFECT_MULTIPLIER;
						max_temp /= 100;
					}

					(min_temp.squeeze_to_u8(), max_temp.squeeze_to_u8())
				};

				let lust_amount =
					if min == max {
						max
					} else {
						rng.gen_range(min..=max)
					};

				*girl.lust += lust_amount;
			},
			TargetApplier::Mark { duration_ms } => {
				let final_duration_ms = {
					let mut temp = duration_ms.to_sat_i64();
					if is_crit {
						temp *= CRIT_DURATION_MULTIPLIER;
						temp /= 100;
					}
					temp.to_sat_u64()
				};

				let effect = PersistentEffect::Marked { duration_ms: final_duration_ms };
				caster.persistent_effects.push(effect);
			},
			TargetApplier::Move { direction, .. } => { //apply chance ignored when applied to self
				let direction = match direction {
					MoveDirection::ToCenter(amount) => { -amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let allies_space_occupied = iter_allies_of!(caster, others)
					.fold(0, |sum, ally| sum + ally.position().size().get());

				let (order_old, order_current) = {
					let temp: &mut Bound_u8<0, {u8::MAX}> = caster.position.order_mut();
					let old_temp = *temp;
					*temp += direction.get();
					if temp.get() > allies_space_occupied {
						temp.set(allies_space_occupied);
					}
					(old_temp.squeeze_to_i8(), temp.squeeze_to_i8())
				};

				let order_delta = order_current - order_old;
				let inverse_delta = -1 * order_delta;

				iter_mut_allies_of!(caster, others).for_each(|ally|
					*ally.position_mut().order_mut() += inverse_delta);
			},
			TargetApplier::PersistentHeal { duration_ms, heal_per_interval } => {
				let final_heal_per_interval_option = {
					let mut temp = heal_per_interval.get().to_sat_i64();

					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection))
						&& any_matches!(caster.persistent_effects, PersistentEffect::Debuff {..} | PersistentEffect::Poison {..}) {
						temp *= 130;
						temp /= 100;
					}

					if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
						let stacks = u64::clamp(accumulated_ms.get() / 1000, 0, 8);
						temp *= 100 + stacks * 5;
						temp /= 100;
						accumulated_ms.set(0);
					}

					NonZeroU8::new(temp.squeeze_to())
				};
				let Some(final_heal_per_interval) = final_heal_per_interval_option
					else { return; };

				if let Some(Perk::Nema(NemaPerk::Healer_Adoration)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Adoration)) {
					let toughness_buff = TargetApplier::Buff {
						duration_ms: 4000.to_sat_u64(),
						stat: DynamicStat::Toughness,
						stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
					};

					toughness_buff.apply_self(caster, others, rng, false);

					if let Some(girl) = &mut caster.girl_stats {
						*girl.lust -= 4;

						let composure_buff = TargetApplier::Buff {
							duration_ms: 4000.to_sat_u64(),
							stat: DynamicStat::Composure,
							stat_increase: unsafe { NonZeroU16::new_unchecked(10) }, // SOUNDNESS: 10 is not 0
						};

						composure_buff.apply_self(caster, others, rng, false);
					}
				}

				let effect = PersistentEffect::Heal {
					duration_ms: *duration_ms,
					accumulated_ms: 0.to_sat_u64(),
					heal_per_interval: final_heal_per_interval
				};

				caster.persistent_effects.push(effect);
			}
			TargetApplier::Poison { duration_ms, poison_per_interval, 
				additives, .. } => {
				let final_poison_per_interval_option = {
					let mut temp = poison_per_interval.get().to_sat_i64();
					if is_crit {
						temp *= CRIT_EFFECT_MULTIPLIER;
						temp /= 100;
					}

					NonZeroU8::new(temp.squeeze_to())
				};
				let Some(final_poison_per_interval) = final_poison_per_interval_option
					else { return; };

				let (final_duration_ms, interval_ms) =
					if any_matches!(additives, PoisonAdditive::Nema_Madness) {
						let mut temp = duration_ms.to_sat_i64();
						temp /= 2;
						(temp.to_sat_u64(), IntervalMS::new(500))
					} else {
						(*duration_ms, IntervalMS::new(1000))
					};

				let effect = PersistentEffect::Poison {
					duration_ms: final_duration_ms,
					accumulated_ms: 0.to_sat_u64(),
					interval_ms,
					poison_per_interval: final_poison_per_interval,
					caster_guid: caster.guid(),
					additives: additives.clone()
				};

				caster.persistent_effects.push(effect);
			}
			TargetApplier::MakeTargetRiposte { duration_ms, skill_power,
				acc_mode, crit_mode } => {
				let final_skill_power_option = {
					let mut temp = skill_power.to_sat_i64();
					if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(caster, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
						temp += 30;
					}
					NonZeroU16::new(temp.squeeze_to())
						.map(|non_zero| Power::new(non_zero.get()))
				};
				let Some(final_skill_power) = final_skill_power_option
					else { return; };

				let effect = PersistentEffect::Riposte {
					duration_ms: *duration_ms,
					skill_power: final_skill_power,
					acc_mode: *acc_mode,
					crit_mode: *crit_mode
				};

				caster.persistent_effects.push(effect);
			}
			TargetApplier::Stun { force } => {
				let final_force = {
					let mut temp = force.get().to_sat_i64();
					if is_crit {
						temp += CRIT_CHANCE_MODIFIER;
					}
					temp.get() as f64
				};

				let def = caster.dyn_stat::<StunDef>().get() as f64;

				let dividend = final_force + (final_force * final_force / 500.0) - def - (def * def / 500.0);
				let divisor = 125.0 + (final_force * 0.25) + (def * 0.25) + (final_force * def * 0.0005);

				let bonus_redundancy_f64 = (dividend / divisor) * 4000.0;
				if bonus_redundancy_f64 > 0. {
					let bonus_redundancy_f64 = f64::clamp(bonus_redundancy_f64, 1., u64::MAX as f64);
					let bonus_redundancy_ms = SaturatedU64::new(bonus_redundancy_f64.round() as u64);

					match &mut caster.stun_redundancy_ms {
						Some(remaining) => {
							*remaining += bonus_redundancy_ms;
						},
						None => {
							caster.stun_redundancy_ms = Some(bonus_redundancy_ms);
						},
					};
				}
			},
			TargetApplier::TemporaryPerk { duration_ms, perk } => { // can't crit
				let effect = PersistentEffect::TemporaryPerk {
					duration_ms: *duration_ms,
					perk: perk.clone()
				};

				caster.persistent_effects.push(effect);
			}
			TargetApplier::MakeSelfGuardTarget { .. }
			| TargetApplier::MakeTargetGuardSelf { .. }
			| TargetApplier::Tempt { .. } => {
				godot_warn!("{}(): {self:?} is not applicable to self! Caster: {caster:?}",
					util::full_fn_name(&Self::apply_self))
			},
		}
	}
}