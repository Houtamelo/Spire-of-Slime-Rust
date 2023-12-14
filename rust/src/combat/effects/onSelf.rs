use std::collections::HashMap;
use rand::prelude::StdRng;
use rand::Rng;
use combat::ModifiableStat;
use proc_macros::{get_perk, get_perk_mut};
use crate::{iter_allies_of, iter_mut_allies_of, combat};
use crate::combat::entity::character::*;
use crate::combat::effects::MoveDirection;
use crate::combat::effects::onTarget::{CRIT_DURATION_MULTIPLIER, CRIT_EFFECT_MULTIPLIER, TargetApplier};
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::*;
use crate::combat::entity::Entity;
use crate::combat::perk::Perk;
use crate::combat::skill_types::CRITMode;
use crate::util::GUID;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfApplier {
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_increase: usize,
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
			SelfApplier::Buff{ duration_ms, stat, mut stat_increase } => {
				if is_crit { stat_increase = (stat_increase * CRIT_EFFECT_MULTIPLIER) / 100; }

				caster.persistent_effects.push(PersistentEffect::Buff{ duration_ms: *duration_ms, stat: *stat, stat_increase });
			}
			SelfApplier::ChangeExhaustion { delta } => { // ignores crit
				if let Some(girl) = &mut caster.girl_stats {
					girl.exhaustion += *delta;
				}
			},
			SelfApplier::Heal{ base_multiplier } => {
				let mut base_multiplier = isize::max(*base_multiplier, 0) as usize;
				if is_crit { base_multiplier = (base_multiplier * CRIT_EFFECT_MULTIPLIER) / 100; }

				let max: usize = usize::max(*caster.dmg.end(), 0);
				let min: usize = usize::clamp(*caster.dmg.start(), 0, max);

				let mut healAmount: usize;

				if max <= 0 {
					return;
				} else if max == min {
					healAmount = (max * base_multiplier) / 100;
				} else {
					healAmount = (seed.gen_range(min..=max) * (base_multiplier)) / 100;
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if caster.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						healAmount = (healAmount * 130) / 100;
					}
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
					let stacks = i64::clamp(*accumulated_ms / 1000, 0, 8) as usize;
					healAmount = (healAmount * (100 + stacks * 5)) / 100;
					*accumulated_ms = 0;
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Adoration)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Adoration)) {
					let toughness_buff = TargetApplier::Buff {
						duration_ms: 4000,
						stat: ModifiableStat::TOUGHNESS,
						stat_increase: 10,
					};

					toughness_buff.apply_self(caster, others, seed, false);

					if let Some(girl) = &mut caster.girl_stats {
						girl.lust -= 4;

						let composure_buff = TargetApplier::Buff {
							duration_ms: 4000,
							stat: ModifiableStat::COMPOSURE,
							stat_increase: 10,
						};

						composure_buff.apply_self(caster, others, seed, false);
					}
				}

				caster.stamina_cur = isize::clamp(caster.stamina_cur + healAmount as isize,0, caster.get_max_stamina());
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

				caster.persistent_effects.push(PersistentEffect::Marked { duration_ms });
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

				if let Some(Perk::Nema(NemaPerk::Healer_Affection)) = get_perk!(caster, Perk::Nema(NemaPerk::Healer_Affection)) {
					if caster.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Debuff(_) | PersistentEffect::Poison { .. })) {
						heal_per_sec = (heal_per_sec * 130) / 100;
					}
				}

				if let Some(Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::Healer_Awe {..})) {
					let stacks = i64::clamp(*accumulated_ms / 1000, 0, 8) as usize;
					heal_per_sec = (heal_per_sec * (100 + stacks * 5)) / 100;
					*accumulated_ms = 0;
				}

				caster.persistent_effects.push(PersistentEffect::Heal { duration_ms: *duration_ms, accumulated_ms: 0, heal_per_sec });
			}
			SelfApplier::Riposte{ duration_ms, mut dmg_multiplier, acc, crit } => {
				if let Some(Perk::Ethel(EthelPerk::Duelist_EnGarde)) = get_perk!(caster, Perk::Ethel(EthelPerk::Duelist_EnGarde)) {
					dmg_multiplier += 30;
				}

				caster.persistent_effects.push( PersistentEffect::Riposte { duration_ms: *duration_ms, dmg_multiplier, acc: *acc, crit: *crit });
			}
			SelfApplier::Summon{ .. } => {} //todo!
		}
	}
}