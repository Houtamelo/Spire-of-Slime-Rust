use std::collections::HashMap;
use std::num::{NonZeroI8, NonZeroU16, NonZeroU8};

use comfy_bounded_ints::prelude::{SqueezeTo, SqueezeTo_i8, SqueezeTo_u8};
use gdnative::log::godot_warn;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use util::any_matches;
use uuid::Uuid;

use combat::effects::MoveDirection;
use combat::effects::onTarget::{CRIT_DURATION_MULTIPLIER, CRIT_EFFECT_MULTIPLIER, TargetApplier};
use combat::effects::persistent::PersistentEffect;
use combat::entity::{iter_allies_of, iter_mut_allies_of};
use combat::entity::character::*;
use combat::entity::data::girls::ethel::perks::*;
use combat::entity::data::girls::nema::perks::*;
use combat::entity::Entity;
use combat::perk::{get_perk, get_perk_mut};
use combat::perk::Perk;
use combat::skill_types::{ACCMode, CRITMode};
use combat::stat::{CheckedRange, DynamicStat};

#[allow(unused_imports)]
use crate::*;
use crate::combat;
use crate::combat::entity::stat::Power;
use crate::misc::{SaturatedU64, ToSaturatedI64, ToSaturatedU64};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfApplier {
	Buff {
		duration_ms: SaturatedU64,
		stat: DynamicStat,
		stat_increase: NonZeroU16,
	},
	ChangeExhaustion {
		delta: NonZeroI8,
	},
	Heal { 
		multiplier: NonZeroU16,
	},
	Lust {
		delta: CheckedRange,
	},
	Mark { 
		duration_ms: SaturatedU64,
	},
	Move { 
		direction: MoveDirection,
	},
	PersistentHeal {
		duration_ms: SaturatedU64,
		heal_per_interval: NonZeroU8,
	},
	Riposte {
		duration_ms: SaturatedU64,
		skill_power: NonZeroU16,
		acc_mode: ACCMode,
		crit_mode: CRITMode,
	},
	Summon { 
		character_key: String,
	},
}

impl SelfApplier {
	pub fn apply(&self, caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>, 
	             rng: &mut Xoshiro256PlusPlus, is_crit: bool) {
		match self {
			SelfApplier::Buff{ duration_ms, stat, stat_increase } => {
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

					caster.persistent_effects.push(effect);
				});
			}
			SelfApplier::ChangeExhaustion { delta } => { // ignores crit
				let Some(girl) = &mut caster.girl_stats
					else {
						godot_warn!("{}(): Trying to change exhaustion of non-girl character: {caster:?}",
							full_fn_name(&Self::apply));
						return;
					};

				*girl.exhaustion += delta.get();
			},
			SelfApplier::Heal { multiplier } => {
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
					temp *= final_multiplier;
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

					NonZeroU16::new(temp.squeeze_to())
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
			SelfApplier::Lust{ delta } => {
				let Some(girl) = &mut caster.girl_stats
					else {
						godot_warn!("{}(): Trying to apply lust on-non girl self: {caster:?}",
							full_fn_name(&Self::apply));
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
			SelfApplier::Mark{ duration_ms } => {
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
				caster.persistent_effects.push(effect);
			},
			SelfApplier::Move{ direction } => {
				let direction = match direction {
					MoveDirection::ToCenter(amount) => { -amount.abs() }
					MoveDirection::ToEdge  (amount) => { amount.abs() }
				};

				let allies_space_occupied = iter_allies_of!(caster, others)
					.fold(0, |sum, ally| sum + ally.position().size.get());

				let (order_old, order_current) = {
					let temp = &mut caster.position.order;
					let old_temp = *temp;
					*temp += direction.get();
					if temp.get() > allies_space_occupied {
						temp.set(allies_space_occupied);
					}
					(old_temp.squeeze_to_i8(), temp.squeeze_to_i8())
				};

				let order_delta = order_current - order_old;
				let inverse_delta = -1 * order_delta;

				for caster_ally in iter_mut_allies_of!(caster, others) {
					caster_ally.position_mut().order += inverse_delta;
				}
			},
			SelfApplier::PersistentHeal{ duration_ms, heal_per_interval } => {
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
			SelfApplier::Riposte{ duration_ms, skill_power, 
				acc_mode, crit_mode } => {
				let final_skill_power = {
					let mut temp = skill_power.get().to_sat_i64();
					if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(caster, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
						temp += 30;
					}
					Power::new(temp.squeeze_to())
				};
				
				let effect = PersistentEffect::Riposte {
					duration_ms: *duration_ms,
					skill_power: final_skill_power,
					acc_mode: *acc_mode,
					crit_mode: *crit_mode
				};

				caster.persistent_effects.push(effect);
			}
			SelfApplier::Summon{ .. } => {} //todo!
		}
	}
}