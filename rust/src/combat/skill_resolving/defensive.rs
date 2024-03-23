use std::collections::{HashMap, HashSet};
use std::num::NonZeroU16;

use gdnative::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use util::full_fn_name;
use uuid::Uuid;

use crate::combat::effects::onTarget::{DebuffApplierKind, TargetApplier};
use crate::combat::entity::*;
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::perk::{get_perk_mut, has_perk, Perk};
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::stat::DynamicStat;
use crate::misc::{Base100ChanceGenerator, ToSaturatedU64, TrackedTicks};

pub fn start_targeting_self(caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>,
                            skill: DefensiveSkill, rng: &mut Xoshiro256PlusPlus, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, others, &skill, rng, recover_ms);
	resolve_target_self(caster, others, &skill, rng);
	
	if !skill.multi_target {
		return;
	}

	let targets_guid: HashSet<Uuid> = iter_allies_of!(caster, others)
		.filter_map(|possible_target| 
			possible_target.position()
				.contains_any(&skill.target_positions)
				.then_some(possible_target.guid()))
		.collect();

	targets_guid.into_iter()
		.for_each(|guid|
			match others.remove(&guid) {
				Some(Entity::Character(ally)) => {
					resolve_target_ally(caster, ally, others, &skill, rng)
						.map(|survived| others.insert(survived.guid(), Entity::Character(survived)));
				},
				Some(entity) => {
					godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but the entity was not a character.\n\
						Entity: {entity:?}", full_fn_name(&start_targeting_ally));
					others.insert(entity.guid(), entity);
				},
				None => {
					godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but it was not found in the allies!",
						full_fn_name(&start_targeting_ally));
					return;
				}
			});
}

pub fn start_targeting_ally(caster: &mut CombatCharacter, target: CombatCharacter, others: &mut HashMap<Uuid, Entity>,
                            skill: DefensiveSkill, rng: &mut Xoshiro256PlusPlus, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, others, &skill, rng, recover_ms);

	if !skill.multi_target {
		resolve_target_ally(caster, target, others, &skill, rng)
			.map(|alive| others.insert(alive.guid(), Entity::Character(alive)));
		return;
	}

	let targets_guid: HashSet<Uuid> = {
		let mut temp: HashSet<Uuid> = iter_allies_of!(target, others)
			.filter_map(|possible_target|
				possible_target.position()
					.contains_any(&skill.target_positions)
					.then_some(possible_target.guid()))
			.collect();

		if caster.position.contains_any(&skill.target_positions) { // caster may collaterally target himself
			temp.insert(caster.guid);
		}
		
		temp
	};
	
	resolve_target_ally(caster, target, others, &skill, rng)
		.map(|alive| others.insert(alive.guid(), Entity::Character(alive)));

	targets_guid.into_iter()
		.for_each(|guid|
			match others.remove(&guid) {
				Some(Entity::Character(ally)) => {
					resolve_target_ally(caster, ally, others, &skill, rng)
						.map(|survived| others.insert(survived.guid(), Entity::Character(survived)));
				},
				Some(entity) => {
					godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but the entity was not a character.\n\
						Entity: {entity:?}", full_fn_name(&start_targeting_ally));
					others.insert(entity.guid(), entity);
				},
				None if guid == caster.guid => {
					resolve_target_self(caster, others, &skill, rng);
				},
				None => {
					godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but it was not found.",
						full_fn_name(&start_targeting_ally));
					return;
				}
			});
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>,
                                  skill: &DefensiveSkill, rng: &mut Xoshiro256PlusPlus, recover_ms: Option<i64>) {
	recover_ms.map(|ms| caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(ms.to_sat_u64()) });

	let is_crit = skill
		.final_crit_chance(caster)
		.is_some_and(|chance| rng.base100_chance(chance));

	if is_crit && let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
		*stacks -= 2;
	}

	skill.effects_self.iter().for_each(|applier|
		applier.apply(caster, others, rng, is_crit));
}

/// Returns the target, if it wasn't killed/grappled.
#[must_use]
fn resolve_target_ally(caster: &mut CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<Uuid, Entity>,
                       skill: &DefensiveSkill, rng: &mut Xoshiro256PlusPlus) -> Option<CombatCharacter> {
	let is_crit = skill
		.final_crit_chance(caster)
		.is_some_and(|chance| rng.base100_chance(chance));
	
	if is_crit && let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
		*stacks -= 2;
	}
	
	for applier in skill.effects_target.iter() {
		if let Some(target_option) = applier.apply_target(caster, target, others, rng, is_crit) {
			target = target_option;
		} else {
			return None;
		}
	}

	// Perks
	{
		if has_perk!(target, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000.to_sat_u64(),
				stat: DynamicStat::Speed,
				stat_increase: unsafe { NonZeroU16::new_unchecked(25) },
			};

			let toughness_debuff = TargetApplier::Debuff { 
				duration_ms: 4000.to_sat_u64(), 
				apply_chance: None,
				applier_kind: DebuffApplierKind::Standard { 
					stat: DynamicStat::Toughness, 
					stat_decrease: unsafe { NonZeroU16::new_unchecked(15) },
				}
			};

			let composure_debuff = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: None,
				applier_kind: DebuffApplierKind::Standard { 
					stat: DynamicStat::Composure, 
					stat_decrease: unsafe { NonZeroU16::new_unchecked(15) }, 
				}
			};

			spd_buff.apply_self(&mut target, others, rng, false);
			toughness_debuff.apply_self(&mut target, others, rng, false);
			composure_debuff.apply_self(&mut target, others, rng, false);
		}
	}

	return Some(target);
}

fn resolve_target_self(caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>, 
                       skill: &DefensiveSkill, rng: &mut Xoshiro256PlusPlus) {
	let is_crit = skill
		.final_crit_chance(caster)
		.is_some_and(|chance| rng.base100_chance(chance));

	if is_crit && let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
		*stacks -= 2;
	}

	skill.effects_target.iter()
		.for_each(|applier| applier.apply_self(caster, others, rng, is_crit));

	// Perks
	{
		if has_perk!(caster, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000.to_sat_u64(),
				stat: DynamicStat::Speed,
				stat_increase: unsafe { NonZeroU16::new_unchecked(25) },
			};

			let toughness_debuff = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: None,
				applier_kind: DebuffApplierKind::Standard { 
					stat: DynamicStat::Toughness, 
					stat_decrease: unsafe { NonZeroU16::new_unchecked(15) }, 
				} 
			};

			let composure_debuff = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: None,
				applier_kind: DebuffApplierKind::Standard { 
					stat: DynamicStat::Composure, 
					stat_decrease: unsafe { NonZeroU16::new_unchecked(15) }, 
				} 
			};

			spd_buff.apply_self(caster, others, rng, false);
			toughness_debuff.apply_self(caster, others, rng, false);
			composure_debuff.apply_self(caster, others, rng, false);
		}
	}
}