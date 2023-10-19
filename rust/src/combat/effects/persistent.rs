use std::collections::{HashMap, HashSet};
use gdnative::log::godot_warn;
use rand::rngs::StdRng;
use combat::ModifiableStat;
use crate::{combat};
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::ethel::perks::EthelPerk;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::Entity;
use crate::combat::perk::Perk;
use crate::combat::skill_types::CRITMode;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub enum PersistentEffect {
	Poison {
		duration_ms: i64,
		accumulated_ms: i64,
		interval_ms: i64,
		dmg_per_interval: usize,
		additives: HashSet<PoisonAdditive>,
		caster_guid: GUID,
	},
	Heal {
		duration_ms: i64,
		accumulated_ms: i64,
		heal_per_sec: usize,
	},
	Arousal {
		duration_ms: i64,
		accumulated_ms: i64,
		lust_per_sec: usize,
	},
	Buff {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_increase: usize,
	},
	Debuff(PersistentDebuff),
	Guarded {
		duration_ms: i64,
		guarder_guid: GUID,
	},
	Marked {
		duration_ms: i64,
	},
	Riposte {
		duration_ms: i64,
		dmg_multiplier: isize,
		acc: isize,
		crit: CRITMode,
	},
	TemporaryPerk {
		duration_ms: i64,
		perk: Perk,
	}
}

#[derive(Debug, Clone)]
pub enum PersistentDebuff {
	Standard {
		duration_ms: i64,
		stat: ModifiableStat,
		stat_decrease: usize,
	},
	StaggeringForce {
		duration_ms: i64,
	}
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PoisonAdditive {
	Ethel_LingeringToxins,
	Ethel_ParalyzingToxins,
	Ethel_ConcentratedToxins,
	Nema_Madness,
}

impl PersistentDebuff {
	pub fn duration(&self) -> i64 {
		match self {
			PersistentDebuff::Standard { duration_ms, .. } => { *duration_ms },
			PersistentDebuff::StaggeringForce { duration_ms, .. } => { *duration_ms },
		}
	}

	pub fn duration_mut(&mut self) -> &mut i64 {
		match self {
			PersistentDebuff::Standard        { duration_ms, .. } => { duration_ms },
			PersistentDebuff::StaggeringForce { duration_ms, .. } => { duration_ms },
		}
	}
}

impl PersistentEffect {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<GUID, Entity>, seed: &mut StdRng, delta_ms: i64) {
		let iter : Vec<PersistentEffect> = owner.persistent_effects.drain(0..owner.persistent_effects.len()).collect();
		for mut effect in iter {
			match &mut effect {
				PersistentEffect::Poison { duration_ms, accumulated_ms, interval_ms, dmg_per_interval, caster_guid, .. } => {
					let caster_guid = caster_guid.clone();

					let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);

					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;

					let intervals_count = (*accumulated_ms / *interval_ms) as usize;
					let dmg: usize = intervals_count * *dmg_per_interval;


					if *duration_ms > 0 {
						if intervals_count > 0 {
							*accumulated_ms -= intervals_count as i64 * *interval_ms;
							owner.stamina_cur -= dmg as isize;
						}

						owner.persistent_effects.push(effect);
					} else if *accumulated_ms > 0 || intervals_count > 0 {
						let partial_interval_damage = (dmg as i64 + (*accumulated_ms * *dmg_per_interval as i64) / *interval_ms) as isize;
						owner.stamina_cur -= partial_interval_damage;
					} else {
						return;
					}

					if owner.stamina_dead() { // poison killed this character, we can ignore the rest of the status effects
						let mut killer_option : Option<CombatCharacter> = None;
						if let Some(killer_entity) = others.remove(&caster_guid) {
							if let Entity::Character(killer) = killer_entity {
								killer_option = Some(killer);
							} else {
								others.insert(killer_entity.guid(), killer_entity);
							}
						}

						owner.do_zero_stamina(killer_option.as_mut(), others, seed);

						if let Some(killer) = killer_option {
							others.insert(killer.guid, Entity::Character(killer));
						}

						return;
					}
				},
				PersistentEffect::Heal{ duration_ms, accumulated_ms, heal_per_sec } => {
					let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);
					
					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;
					
					let intervals_count = (*accumulated_ms / 1000) as usize;
					let heal: usize = intervals_count * *heal_per_sec;
					
					if heal > 0 {
						*accumulated_ms -= intervals_count as i64 * 1000;
						owner.stamina_cur += heal as isize;
					}

					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Arousal{ duration_ms, accumulated_ms, lust_per_sec } => {
					let actual_ms = clamp_tick_ms(delta_ms, *duration_ms);
					
					*accumulated_ms += actual_ms;
					*duration_ms -= actual_ms;
					
					let intervals_count = (*accumulated_ms / 1000) as usize;
					let lust: usize = intervals_count * *lust_per_sec;
					
					if lust > 0 {
						let Some(girl) = &mut owner.girl_stats else { 
							godot_warn!("character has arousal status but isn't a girl: {owner:?}");
							continue;
						};
						
						*accumulated_ms -= intervals_count as i64 * 1000;
						girl.lust += lust as isize;
					}

					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Buff{ duration_ms, .. } => {
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Debuff(debuff) => {
					let duration_ms = debuff.duration_mut();
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Guarded{ duration_ms, .. } => {
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Marked{ duration_ms } => {
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::Riposte{ duration_ms, .. } => {
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
				PersistentEffect::TemporaryPerk{ duration_ms, .. } => {
					*duration_ms -= delta_ms;
					if *duration_ms > 0 { owner.persistent_effects.push(effect); }
				},
			}
		}
		
		others.insert(owner.guid, Entity::Character(owner));
		return;
		
		fn clamp_tick_ms(input: i64, duration: i64) -> i64 {
			if input <= duration {
				return input;
			} else {
				godot_warn!("Tick ms is greater than duration_ms. This should not happen. Tick ms: {}, duration_ms: {}", input, duration);
				return duration;
			};
		}
	}
	
	pub fn duration(&self) -> i64 {
		return match self {
			PersistentEffect::Poison       { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Heal         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Arousal      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Buff         { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Guarded      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::Marked       { duration_ms    } => { *duration_ms },
			PersistentEffect::Riposte      { duration_ms, ..} => { *duration_ms },
			PersistentEffect::TemporaryPerk{ duration_ms, ..} => { *duration_ms },
			PersistentEffect::Debuff(debuff) => { debuff.duration() },
		};
	}

	pub fn get_poison_additives(perks: &Vec<Perk>) -> HashSet<PoisonAdditive> {
		let mut additives = HashSet::new();
		for perk in perks {
			match perk {
				Perk::Ethel(EthelPerk::Poison_LingeringToxins   ) => { additives.insert(PoisonAdditive::Ethel_LingeringToxins   ); },
				Perk::Ethel(EthelPerk::Poison_ParalyzingToxins  ) => { additives.insert(PoisonAdditive::Ethel_ParalyzingToxins  ); },
				Perk::Ethel(EthelPerk::Poison_ConcentratedToxins) => { additives.insert(PoisonAdditive::Ethel_ConcentratedToxins); },
				Perk::Nema (NemaPerk ::Poison_Madness           ) => { additives.insert(PoisonAdditive::Nema_Madness            ); },
				_ => {}
			}
		}

		return additives;
	}
}