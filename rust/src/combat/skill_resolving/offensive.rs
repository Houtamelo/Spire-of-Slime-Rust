use gdnative::godot_error;
use rand::prelude::StdRng;
use rand::Rng;
use crate::combat::effects::persistent::PersistentEffect::Riposte;
use crate::combat::entity::character::*;
use crate::combat::entity::girl::*;
use crate::combat::entity::*;
use crate::combat::entity::position::Position;
use crate::combat::skills::*;
use crate::combat::skills::offensive::OffensiveSkill;
use crate::util::{Base100ChanceGenerator, GUID, TrackedTicks};

//todo! check riposte
#[must_use]
pub fn start(mut caster: CombatCharacter, mut target: CombatCharacter, caster_allies: &mut Vec<Entity>, caster_enemies: &mut Vec<Entity>, skill: OffensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(&mut caster, caster_allies, caster_enemies, &skill, seed, recover_ms);
	if skill.multi_target {
		let mut targets_guid: Vec<GUID> = Vec::new();
		for possible_target in caster_enemies.iter() {
			let target_position = possible_target.position();
			if target_position.contains_any(&skill.allowed_enemy_positions) {
				targets_guid.push(possible_target.guid());
			}
		}
		
		let main_result = resolve_target(&mut caster, &mut target, caster_allies, caster_enemies, &skill, seed);
		let is_caster_alive = check_attack_result(caster, target, main_result, caster_allies, caster_enemies);
		
		match is_caster_alive {
			Some(alive) => caster = alive,
			None => return,
		}

		while let Some(target_guid) = targets_guid.pop() {
			if let Some(position) = caster_enemies.iter().position(|ally| ally.guid() == target_guid) {
				let mut enemy = caster_enemies.remove(position);
				if let Entity::Character(mut enemy) = enemy { // for now, we only support skills on characters
					let result = resolve_target(&mut caster, &mut enemy, caster_allies, caster_enemies, &skill, seed);
					let is_caster_alive = check_attack_result(caster, enemy, result, caster_allies, caster_enemies);

					match is_caster_alive {
						Some(alive) => caster = alive,
						None => return,
					}
				} else { // just give it back
					caster_enemies.push(enemy);
				}
			} else {
				godot_error!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!");
			}
		}
		
		caster_allies.push(Entity::Character(caster));
	}
	else {
		let result = resolve_target(&mut caster, &mut target, caster_allies, caster_enemies, &skill, seed);
		let is_caster_alive = check_attack_result(caster, target, result, caster_allies, caster_enemies);
		if let Some(alive) = is_caster_alive {
			caster_allies.push(Entity::Character(alive))
		}
	}
	
	return;
	
	/// returns caster if they are alive, otherwise we drop caster
	#[must_use]
	fn check_attack_result(mut caster: CombatCharacter, mut target: CombatCharacter, result: AttackResult, caster_allies: &mut Vec<Entity>, caster_enemies: &mut Vec<Entity>) -> Option<CombatCharacter> {
		match result {
			AttackResult::TargetDefeated => {
				match target.on_defeat {
					OnDefeat::CorpseOrDefeatedGirl => {
						let corpse = Corpse {
							guid: target.guid,
							position: target.position,
						};

						match Position::same_side(&caster.position, &corpse.position) {
							true  => caster_allies .push(Entity::Corpse(corpse)),
							false => caster_enemies.push(Entity::Corpse(corpse)),
						}
					}
					OnDefeat::Vanish => {}
				}
				
				return Some(caster);
			},
			AttackResult::BothAlive => {
				match Position::same_side(&caster.position, &target.position) {
					true  => caster_allies .push(Entity::Character(target)),
					false => caster_enemies.push(Entity::Character(target)),
				} 
				
				return Some(caster);
			}
			AttackResult::CasterDefeated => {
				match Position::same_side(&caster.position, &target.position) {
					true  => caster_allies .push(Entity::Character(target)),
					false => caster_enemies.push(Entity::Character(target)),
				}
				
				return None;
			}
		} 
	}
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, caster_allies: &mut Vec<Entity>, caster_enemies: &mut Vec<Entity>, skill: &OffensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	if let Some(recover_ms) = recover_ms {
		caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(recover_ms) };
	}
	
	match skill.use_counter { //we are not responsible for checking if use counter surpassed the limit, but we are responsible for incrementing it
		UseCounter::Limited { .. } => {
			//caster.increment_skill_counter(&skill.data_key);// todo!
		}
		UseCounter::Unlimited => {} 
	}

	let crit_chance: Option<isize> = skill.final_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false,
	};

	for self_applier in skill.effects_self.iter() {
		self_applier.apply(caster, caster_allies, caster_enemies, seed, is_crit);
	}
}

#[must_use]
fn resolve_target(caster: &mut CombatCharacter, target: &mut CombatCharacter, caster_allies: &mut Vec<Entity>, caster_enemies: &mut Vec<Entity>, skill: &OffensiveSkill, seed: &mut StdRng) -> AttackResult {
	match skill.final_hit_chance(caster, target) {
		Some(chance) => {
			if seed.base100_chance(chance) == false {
				return AttackResult::BothAlive;
			}
		}
		None => {}
	}
	
	let crit_chance: Option<isize> = skill.final_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false
	};

	for target_applier in skill.effects_target.iter() {
		target_applier.apply_target(caster, target, caster_allies, caster_enemies, seed, is_crit);
	}

	let Some(damage_range) = skill.calc_dmg(caster, target, is_crit) else {
		return AttackResult::BothAlive;
	};
	
	let damage = seed.gen_range(damage_range.min..=damage_range.max);
	if damage <= 0 {
		if target.stamina_cur <= 0 {
			return AttackResult::TargetDefeated;
		} else {
			return AttackResult::BothAlive;
		}
	}

	match &target.state {
		CharacterState::Idle
		| CharacterState::Charging { .. }
		| CharacterState::Recovering { .. } => {
			target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);
			if target.stamina_cur <= 0 {
				match target.girl_stats {
					Some(_) => {
						target.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000) };
						return AttackResult::BothAlive;
					}
					None => {
						return AttackResult::TargetDefeated;
					}
				}
			} else {
				return check_riposte(caster, target, skill, seed);
			}
		}
		CharacterState::Stunned { .. } => {
			target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);
			if target.stamina_cur <= 0 {
				match target.girl_stats {
					Some(_) => {
						target.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000) };
						return AttackResult::BothAlive;
					}
					None => {
						return AttackResult::TargetDefeated;
					}
				}
			} else {
				return AttackResult::BothAlive;
			}
		}
		,
		CharacterState::Grappling(g) => {
			//todo! fix borrow checker
			return AttackResult::BothAlive;
			//return grappler_attacked(caster, target, caster_allies, caster_enemies, victim, damage);
		},
		CharacterState::Downed { .. } => return AttackResult::BothAlive, // damage is ignored on downed characters.
	}
	
	#[must_use]
	fn grappler_attacked(caster: &mut CombatCharacter, target: &mut CombatCharacter, caster_allies: &mut Vec<Entity>, caster_enemies: &mut Vec<Entity>, victim: GrappledGirl, damage: isize) -> AttackResult {
		let target_old_stamina_percent = target.stamina_cur as f64 / target.stamina_max as f64;
		target.stamina_cur -= damage;
		target.last_damager_guid = Some(caster.guid);
		let target_new_stamina_percent = target.stamina_cur as f64 / target.stamina_max as f64;

		if target.stamina_cur <= 0 {
			target.state = CharacterState::Idle;
			match victim {
				GrappledGirl::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.to_non_grappled();
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500) }; // girl is downed for 2.5s after being released from a grapple

					let girl_allies = match Position::same_side(&caster.position, &girl_standing.position) {
						true  => caster_allies,
						false => caster_enemies,
					};

					*girl_standing.position.order_mut() = 0;

					for girl_ally in girl_allies.iter_mut() {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					girl_allies.push(Entity::Character(girl_standing));
				}
				GrappledGirl::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.to_non_grappled();

					let girl_allies = match Position::same_side(&caster.position, &girl_standing.position) {
						true => caster_allies,
						false => caster_enemies,
					};

					*girl_standing.position.order_mut() = 0;

					for girl_ally in girl_allies.iter_mut() {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					girl_allies.push(Entity::DefeatedGirl(girl_standing));
				}
			}

			return AttackResult::TargetDefeated;
		}
		else if target_old_stamina_percent - target_new_stamina_percent >= 0.25 { // even if it doesn't kill, any attack that deals more than 25% of total health disables grappling
			target.state = CharacterState::Idle;
			match victim {
				GrappledGirl::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.to_non_grappled();
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500) }; // girl is downed for 2.5s after being released from a grapple

					let girl_allies = match Position::same_side(&caster.position, &girl_standing.position) {
						true => caster_allies,
						false => caster_enemies,
					};

					*girl_standing.position.order_mut() = 0;

					for girl_ally in girl_allies.iter_mut() {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					girl_allies.push(Entity::Character(girl_standing));
				}
				GrappledGirl::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.to_non_grappled();

					let girl_allies = match Position::same_side(&caster.position, &girl_standing.position) {
						true => caster_allies,
						false => caster_enemies,
					};

					*girl_standing.position.order_mut() = 0;

					for girl_ally in girl_allies.iter_mut() {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					girl_allies.push(Entity::DefeatedGirl(girl_standing));
				}
			}

			return AttackResult::BothAlive;
		}
		else {
			return AttackResult::BothAlive;
		}
	}

	#[must_use]
	fn check_riposte(caster: &mut CombatCharacter, target: &mut CombatCharacter, skill: &OffensiveSkill, seed: &mut StdRng) -> AttackResult {
		if skill.can_be_riposted == false {
			return AttackResult::BothAlive;
		}
		
		let found = target.persistent_effects.iter().find(|effect|
				if let Riposte { .. } = effect {
					return true;
				} else {
					return false;
				});
		
		let Some(Riposte { duration_ms: _, dmg_multiplier, acc, crit } ) = found else {
			return AttackResult::BothAlive;
		};

		let final_hit_chance = OffensiveSkill::final_hit_chance_independent(*acc, target, caster);

		if seed.base100_chance(final_hit_chance) == false {
			return AttackResult::BothAlive;
		}

		let is_crit: bool = match crit {
			CRITMode::CanCrit { crit_chance } => seed.base100_chance(OffensiveSkill::final_crit_chance_independent(*crit_chance, target)),
			CRITMode::NeverCrit => false,
		};

		let damage_range = OffensiveSkill::calc_dmg_independent(*dmg_multiplier, 0, target, caster, is_crit);

		let damage = seed.gen_range(damage_range.min..=damage_range.max);
		if damage <= 0 {
			return AttackResult::BothAlive;
		}
		
		caster.stamina_cur -= damage;
		caster.last_damager_guid = Some(target.guid);
		if caster.stamina_cur <= 0 {
			return AttackResult::CasterDefeated;
		} else {
			return AttackResult::BothAlive;
		}
	}
}

enum AttackResult {
	BothAlive,
	TargetDefeated,
	CasterDefeated,
}