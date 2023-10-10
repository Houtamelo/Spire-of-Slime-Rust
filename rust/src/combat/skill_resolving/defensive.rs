use gdnative::godot_error;
use rand::prelude::StdRng;
use crate::combat::entity::*;
use crate::combat::entity::character::*;
use crate::combat::skills::{PositionMatrix};
use crate::combat::skills::defensive::DefensiveSkill;
use crate::util::{Base100ChanceGenerator, GUID, TrackedTicks};

pub fn start_targeting_self(caster: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, skill: DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, allies, enemies, &skill, seed, recover_ms);
	resolve_target_self(caster, allies, enemies, &skill, seed);
	
	if skill.multi_target {
		let mut targets_guid = get_target_guids(&skill.allowed_ally_positions, allies);
		while let Some(target_guid) = targets_guid.pop() {
			let Some(position) = allies.iter().position(|ally| ally.guid() == target_guid) else {
				godot_error!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!"); continue;
			};

			let mut ally = allies.remove(position);
			if let Entity::Character(ally) = &mut ally { // for now, we only support skills on characters
				resolve_target_ally(caster, ally, allies, enemies, &skill, seed);
			}
			allies.push(ally);
		}
	}
	
	return;

	fn get_target_guids(positions: &PositionMatrix, possible_targets: &Vec<Entity>) -> Vec<GUID> {
		let mut target_guids: Vec<GUID> = Vec::new();
		for possible_target in possible_targets {
			let target_position = possible_target.position();
			if target_position.contains_any(&positions) {
				target_guids.push(possible_target.guid());
			}
		}
		return target_guids;
	}
}

pub fn start_targeting_ally(caster: &mut CombatCharacter, target: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, skill: DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(caster, allies, enemies, &skill, seed, recover_ms);
	resolve_target_ally(caster, target, allies, enemies, &skill, seed);
	
	if skill.multi_target {
		let mut targets_guid = get_target_guids(&skill.allowed_ally_positions, allies, caster);
		while let Some(target_guid) = targets_guid.pop() {
			if let Some(position) = allies.iter().position(|ally| ally.guid() == target_guid)
			{
				let mut ally = allies.remove(position);
				if let Entity::Character(ally) = &mut ally { // for now, we only support skills on characters
					resolve_target_ally(caster, ally, allies, enemies, &skill, seed);
				}
				allies.push(ally);
			} else if target_guid == caster.guid {
				resolve_target_self(caster, allies, enemies, &skill, seed);
			}
			else {
				godot_error!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!");
			}
		}
	}

	return;

	fn get_target_guids(positions: &PositionMatrix, possible_targets: &Vec<Entity>, caster: &CombatCharacter) -> Vec<GUID> {
		let mut target_guids: Vec<GUID> = Vec::new();
		for possible_target in possible_targets {
			let target_position = possible_target.position();
			if target_position.contains_any(&positions) {
				target_guids.push(possible_target.guid());
			}
		}
		
		if caster.position.contains_any(&positions) { // caster may collaterally target himself
			target_guids.push(caster.guid);
		}
		
		return target_guids;
	}
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, skill: &DefensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	if let Some(recover_ms) = recover_ms {
		caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(recover_ms) };
	}

	let crit_chance: Option<isize> = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false
	};

	for self_applier in skill.effects_self.iter() {
		self_applier.apply(caster, allies, enemies, seed, is_crit);
	}
}

fn resolve_target_ally(caster: &mut CombatCharacter, target: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, skill: &DefensiveSkill, seed: &mut StdRng) {
	let crit_chance: Option<isize> = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false
	};

	for target_applier in skill.effects_target.iter() {
		target_applier.apply_target(caster, target, allies, enemies, seed, is_crit);
	}
}

fn resolve_target_self(caster: &mut CombatCharacter, allies: &mut Vec<Entity>, enemies: &mut Vec<Entity>, skill: &DefensiveSkill, seed: &mut StdRng) {
	let crit_chance: Option<isize> = skill.calc_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false
	};

	for target_applier in skill.effects_target.iter() {
		target_applier.apply_self(caster, allies, enemies, seed, is_crit);
	}
}