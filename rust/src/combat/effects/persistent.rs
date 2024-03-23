use std::collections::{HashMap, HashSet};
use std::num::{NonZeroU16, NonZeroU8};
use std::vec::IntoIter;

use gdnative::log::godot_warn;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::combat::effects::IntervalMS;
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::ethel::perks::EthelPerk;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::Entity;
use crate::combat::entity::stat::{DynamicStat, Power};
use crate::combat::perk::Perk;
use crate::combat::skill_types::{ACCMode, CRITMode};
use crate::misc::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistentEffect {
	Poison {
		duration_ms: SaturatedU64,
		accumulated_ms: SaturatedU64,
		interval_ms: IntervalMS,
		poison_per_interval: NonZeroU8,
		additives: HashSet<PoisonAdditive>,
		caster_guid: Uuid,
	},
	Heal {
		duration_ms: SaturatedU64,
		accumulated_ms: SaturatedU64,
		heal_per_interval: NonZeroU8,
	},
	Arousal {
		duration_ms: SaturatedU64,
		accumulated_ms: SaturatedU64,
		lust_per_interval: NonZeroU8,
	},
	Buff {
		duration_ms: SaturatedU64,
		stat: DynamicStat,
		stat_increase: NonZeroU16,
	},
	Debuff {
		duration_ms: SaturatedU64,
		debuff_kind: PersistentDebuff
	},
	Guarded {
		duration_ms: SaturatedU64,
		guarder_guid: Uuid,
	},
	Marked {
		duration_ms: SaturatedU64,
	},
	Riposte {
		duration_ms: SaturatedU64,
		skill_power: Power,
		acc_mode: ACCMode,
		crit_mode: CRITMode,
	},
	TemporaryPerk {
		duration_ms: SaturatedU64,
		perk: Perk,
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistentDebuff {
	Standard {
		stat: DynamicStat,
		stat_decrease: NonZeroU16,
	},
	StaggeringForce,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PoisonAdditive {
	Ethel_LingeringToxins,
	Ethel_ParalyzingToxins,
	Ethel_ConcentratedToxins,
	Nema_Madness,
}

impl PersistentEffect {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<Uuid, Entity>,
	                                  rng: &mut Xoshiro256PlusPlus, delta_ms: SaturatedU64) {
		let effects_vec: Vec<PersistentEffect> = owner.persistent_effects
			.drain(0..owner.persistent_effects.len())
			.collect::<Vec<_>>();
		
		let effects_iter = effects_vec.into_iter();
		
		Self::tick_next(owner, effects_iter, others, rng, delta_ms);
	}
	
	fn tick_next(mut owner: CombatCharacter, mut iter: IntoIter<PersistentEffect>, 
	               others: &mut HashMap<Uuid, Entity>, rng: &mut Xoshiro256PlusPlus, delta_ms: SaturatedU64) {
		macro_rules! next { () => { Self::tick_next(owner, iter, others, rng, delta_ms) }; }
		
		let Some(mut effect) = iter.next()
			else { // all effects ticked, recursion ends here
				others.insert(owner.guid, Entity::Character(owner)); 
				return;
			};
		
		match &mut effect {
			PersistentEffect::Poison { duration_ms, accumulated_ms,
				interval_ms, poison_per_interval: dmg_per_interval, caster_guid, .. } => {
				let caster_guid = *caster_guid;
				
				let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);

				*accumulated_ms += actual_ms;
				*duration_ms -= actual_ms;

				let intervals_count: u64 = accumulated_ms.get() / interval_ms.get();
				let dmg = {
					let mut temp = intervals_count.to_sat_i64();
					temp *= dmg_per_interval.get();
					temp
				};

				if duration_ms.get() > 0 {
					if intervals_count > 0 {
						*accumulated_ms -= intervals_count * interval_ms.get();
						*owner.stamina_cur -= dmg.get();
					}

					owner.persistent_effects.push(effect);
				} else if accumulated_ms.get() > 0 || intervals_count > 0 { // duration is over but there might still be some damage left
					*owner.stamina_cur -= dmg.get();

					let partial_interval_dmg = {
						let mut temp = accumulated_ms.to_sat_i64();
						temp *= dmg_per_interval.get();
						temp /= interval_ms.get();
						temp
					};
					
					*owner.stamina_cur -= partial_interval_dmg.get();
				} else { // duration is over and no damage to be dealt, next!
					next!();
					return;
				}

				if owner.stamina_alive() {
					next!();
					return;
				} else { // Owner is dead, recursion ends here
					match others.remove(&caster_guid) {
						Some(Entity::Character(mut killer)) => {
							owner.do_on_zero_stamina(Some(&mut killer), others, rng);
							others.insert(killer.guid, Entity::Character(killer));
						},
						Some(killer_non_character) => {
							others.insert(killer_non_character.guid(), killer_non_character);
							owner.do_on_zero_stamina(None, others, rng);
						}
						None => {
							owner.do_on_zero_stamina(None, others, rng);
						}
					}
				};
			},
			PersistentEffect::Heal{ duration_ms, accumulated_ms, heal_per_interval: heal_per_sec } => {
				const INTERVAL_MS: u64 = 1000;
				
				let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);

				*accumulated_ms += actual_ms;
				*duration_ms -= actual_ms;

				let intervals_count: u64 = accumulated_ms.get() / INTERVAL_MS;
				let heal_amount = {
					let mut temp = intervals_count.to_sat_i64();
					temp *= heal_per_sec.get();
					temp
				};

				if intervals_count > 0 {
					*accumulated_ms -= intervals_count * INTERVAL_MS;
					*owner.stamina_cur += heal_amount.get();
				}

				if duration_ms.get() > 0 {
					owner.persistent_effects.push(effect);
				}
				
				next!();
			},
			PersistentEffect::Arousal{ duration_ms, accumulated_ms, lust_per_interval: lust_per_sec } => {
				const INTERVAL_MS: u64 = 1000;
				
				let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);

				*accumulated_ms += actual_ms;
				*duration_ms -= actual_ms;

				let intervals_count: u64 = accumulated_ms.get() / INTERVAL_MS;
				let lust_delta = {
					let mut temp = intervals_count.to_sat_i64();
					temp *= lust_per_sec.get();
					temp
				};

				if intervals_count > 0 {
					let Some(girl) = &mut owner.girl_stats 
						else { 
							godot_warn!("character has arousal status but isn't a girl: {owner:?}");
							next!();
							return; 
						};

					*accumulated_ms -= intervals_count * INTERVAL_MS;
					*girl.lust += lust_delta.get();
				}

				if duration_ms.get() > 0 {
					owner.persistent_effects.push(effect);
				}

				next!();
			},
			PersistentEffect::Buff{ duration_ms, .. }
			| PersistentEffect::Debuff{ duration_ms, .. }
			| PersistentEffect::Guarded{ duration_ms, .. }
			| PersistentEffect::Marked{ duration_ms }
			| PersistentEffect::Riposte{ duration_ms, .. }
			| PersistentEffect::TemporaryPerk{ duration_ms, .. } => {
				*duration_ms -= delta_ms;
				if duration_ms.get() > 0 {
					owner.persistent_effects.push(effect);
				}

				next!();
			},
		}

		fn clamp_tick_ms(delta_ms: SaturatedU64, duration_ms: SaturatedU64) -> SaturatedU64 {
			return if delta_ms.get() <= duration_ms.get() {
				delta_ms
			} else {
				godot_warn!("Tick ms is greater than duration_ms. This should not happen. Tick ms: {:?}, duration_ms: {:?}", delta_ms, duration_ms);
				duration_ms
			};
		}
	}
	
	pub fn duration(&self) -> SaturatedU64 {
		return match self {
			PersistentEffect::Poison { duration_ms, .. } | PersistentEffect::Heal { duration_ms, .. }
			| PersistentEffect::Arousal { duration_ms, .. } | PersistentEffect::Buff { duration_ms, .. }
			| PersistentEffect::Guarded { duration_ms, .. } | PersistentEffect::Marked { duration_ms } 
			| PersistentEffect::Riposte { duration_ms, .. } | PersistentEffect::Debuff { duration_ms, .. }
			| PersistentEffect::TemporaryPerk { duration_ms, .. } => {
				*duration_ms
			}
		};
	}

	pub fn get_poison_additives(perks: &Vec<Perk>) -> HashSet<PoisonAdditive> {
		return perks.iter()
			.filter_map(|perk| {
				return match perk {
					Perk::Ethel(EthelPerk::Poison_LingeringToxins) => 
						Some(PoisonAdditive::Ethel_LingeringToxins),
					Perk::Ethel(EthelPerk::Poison_ParalyzingToxins) => 
						Some(PoisonAdditive::Ethel_ParalyzingToxins),
					Perk::Ethel(EthelPerk::Poison_ConcentratedToxins) => 
						Some(PoisonAdditive::Ethel_ConcentratedToxins),
					Perk::Nema(NemaPerk::Poison_Madness) => 
						Some(PoisonAdditive::Nema_Madness),
					_ => None
				};
			}).collect();
	}
}