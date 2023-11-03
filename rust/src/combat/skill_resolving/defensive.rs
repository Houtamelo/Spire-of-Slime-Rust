use std::collections::{HashMap, HashSet};
use gdnative::prelude::*;
use rand::prelude::StdRng;
use proc_macros::{get_perk, get_perk_mut};
use crate::combat::effects::onTarget::{DebuffApplier, TargetApplier};
use crate::iter_allies_of;
use crate::combat::entity::*;
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::ModifiableStat;
use crate::combat::perk::Perk;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::util::{Base100ChanceGenerator, GUID, TrackedTicks};

pub fn start_targeting_self(caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, others, &skill, seed, recover_ms);
	resolve_target_self(caster, others, &skill, seed);
	
	if skill.multi_target == false {
		return;
	}

	let mut targets_guid = HashSet::new();
	for possible_target in iter_allies_of!(caster, others) {
		if possible_target.position().contains_any(&skill.target_positions) {
			targets_guid.insert(possible_target.guid());
		}
	}

	for target_guid in targets_guid {
		if let Some(Entity::Character(ally)) = others.remove(&target_guid) {
			let target_ally_option = resolve_target_ally(caster, ally, others, &skill, seed);
			if let Some(ally_alive) = target_ally_option {
				others.insert(ally_alive.guid(), Entity::Character(ally_alive));
			}
		} else {
			godot_warn!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!");
		}
	}
}

pub fn start_targeting_ally(caster: &mut CombatCharacter, target: CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, others, &skill, seed, recover_ms);

	if skill.multi_target == false {
		let target_option = resolve_target_ally(caster, target, others, &skill, seed);
		if let Some(target) = target_option {
			others.insert(target.guid(), Entity::Character(target));
		}
		return;
	}

	let mut targets_guid: HashSet<GUID> = HashSet::new();
	for possible_target in iter_allies_of!(target, others) {
		if possible_target.position().contains_any(&skill.target_positions) {
			targets_guid.insert(possible_target.guid());
		}
	}

	if caster.position.contains_any(&skill.target_positions) { // caster may collaterally target himself
		targets_guid.insert(caster.guid);
	}

	let target_option = resolve_target_ally(caster, target, others, &skill, seed);
	if let Some(target) = target_option {
		others.insert(target.guid(), Entity::Character(target));
	}

	for target_guid in targets_guid {
		if let Some(Entity::Character(ally)) = others.remove(&target_guid) {
			let target_ally_option = resolve_target_ally(caster, ally, others, &skill, seed);
			if let Some(ally_alive) = target_ally_option {
				others.insert(ally_alive.guid(), Entity::Character(ally_alive));
			}
		} else if target_guid == caster.guid {
			resolve_target_self(caster, others, &skill, seed);
		} else {
			godot_warn!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!");
		}
	}
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	if let Some(recover_ms) = recover_ms {
		caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(recover_ms) };
	}

	let crit_chance = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => {
			if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
				*stacks -= 2;
			}

			true
		},
		_ => false
	};

	for self_applier in skill.effects_self.iter() {
		self_applier.apply(caster, others, seed, is_crit);
	}
}

/// Returns the target, if it wasn't killed/grappled.
#[must_use]
fn resolve_target_ally(caster: &mut CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &DefensiveSkill, seed: &mut StdRng) -> Option<CombatCharacter> {
	let crit_chance = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => {
			if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
				*stacks -= 2;
			}

			true
		},
		_ => false
	};

	for target_applier in skill.effects_target.iter() {
		if let Some(target_option) = target_applier.apply_target(caster, target, others, seed, is_crit) {
			target = target_option;
		} else {
			return None;
		}
	}

	// Perks
	{
		if let Some(Perk::Nema(NemaPerk::Grumpiness)) = get_perk!(target, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000,
				stat: ModifiableStat::SPD,
				stat_increase: 25,
			};

			let toughness_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
				duration_ms: 4000,
				stat: ModifiableStat::TOUGHNESS,
				stat_decrease: 15,
				apply_chance: None,
			});

			let composure_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
				duration_ms: 4000,
				stat: ModifiableStat::COMPOSURE,
				stat_decrease: 15,
				apply_chance: None,
			});

			spd_buff.apply_self(&mut target, others, seed, false);
			toughness_debuff.apply_self(&mut target, others, seed, false);
			composure_debuff.apply_self(&mut target, others, seed, false);
		}
	}

	return Some(target);
}

fn resolve_target_self(caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &DefensiveSkill, seed: &mut StdRng) {
	let crit_chance = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => {
			if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
				*stacks -= 2;
			}

			true
		},
		_ => false
	};

	for target_applier in skill.effects_target.iter() {
		target_applier.apply_self(caster, others, seed, is_crit);
	}

	// Perks
	{
		if let Some(Perk::Nema(NemaPerk::Grumpiness)) = get_perk!(caster, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000,
				stat: ModifiableStat::SPD,
				stat_increase: 25,
			};

			let toughness_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
				duration_ms: 4000,
				stat: ModifiableStat::TOUGHNESS,
				stat_decrease: 15,
				apply_chance: None,
			});

			let composure_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
				duration_ms: 4000,
				stat: ModifiableStat::COMPOSURE,
				stat_decrease: 15,
				apply_chance: None,
			});

			spd_buff.apply_self(caster, others, seed, false);
			toughness_debuff.apply_self(caster, others, seed, false);
			composure_debuff.apply_self(caster, others, seed, false);
		}
	}
}