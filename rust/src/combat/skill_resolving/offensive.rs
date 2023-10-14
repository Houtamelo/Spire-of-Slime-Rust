use std::collections::{HashMap, HashSet};
use gdnative::godot_error;
use rand::prelude::StdRng;
use rand::Rng;
use proc_macros::{get_perk, get_perk_mut};
use crate::combat::effects::persistent::PersistentEffect::Riposte;
use crate::combat::entity::character::*;
use crate::combat::entity::girl::*;
use crate::combat::entity::*;
use crate::combat::skill_types::*;
use crate::combat::skill_types::offensive::OffensiveSkill;
use crate::{iter_enemies_of, iter_mut_allies_of};
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::girls::ethel::perks::{Category_Bruiser, EthelPerk};
use crate::combat::entity::data::girls::ethel::skills::EthelSkillName;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::ModifiableStat;
use crate::combat::perk::Perk;
use crate::util::{Base100ChanceGenerator, GUID, TrackedTicks};

pub fn start(mut caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: OffensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	process_self_effects_and_costs(&mut caster, others, &skill, seed, recover_ms);

	if skill.multi_target == false {
		let caster_option = resolve_target(caster, target, others, &skill, seed);  // caster may die due to riposte so we get him back as an option
		if let Some(caster) = caster_option {
			others.insert(caster.guid, Entity::Character(caster));
		}

		return;
	}

	let mut targets_guid = HashSet::new();

	for possible_target in iter_enemies_of!(caster, others) {
		if possible_target.position().contains_any(&skill.target_positions) {
			targets_guid.insert(possible_target.guid());
		}
	}

	let Some(mut caster) = resolve_target(caster, target, others, &skill, seed) else { return; }; // caster may die due to riposte so we get him back as an option

	for target_guid in targets_guid { // for now, we only support skills on characters
		if let Some(Entity::Character(enemy)) = others.remove(&target_guid) {
			let caster_option = resolve_target(caster, enemy, others, &skill, seed);  // caster may die due to riposte so we get him back as an option
			match caster_option {
				Some(caster_alive) => { caster = caster_alive; },
				None => return,
			}
		} else {
			godot_error!("Warning: Trying to apply skill to ally with guid {target_guid:?}, but it was not found in the allies!");
		}
	}

	others.insert(caster.guid, Entity::Character(caster));
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &OffensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	if let Some(recover_ms) = recover_ms {
		caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(recover_ms) };
	}
	
	match skill.use_counter { //we are not responsible for checking if use counter surpassed the limit, but we are responsible for incrementing it
		UseCounter::Limited { .. } => {
			caster.increment_skill_counter(skill.name());
		}
		UseCounter::Unlimited => {} 
	}

	let crit_chance = skill.final_crit_chance(caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false,
	};

	for self_applier in skill.effects_self.iter() {
		self_applier.apply(caster, others, seed, is_crit);
	}
}

/// returns caster if they are alive, otherwise we drop caster
#[must_use]
fn resolve_target(mut caster: CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &OffensiveSkill, seed: &mut StdRng)
	-> Option<CombatCharacter> {
	
	if let Some(chance) = skill.final_hit_chance(&caster, &target) {
		if seed.base100_chance(chance) == false {
			if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { stacks }))) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { .. }))) {
				*stacks = 0;
			}
			if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Grudge { active }))) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Grudge { .. }))) {
				*active = true;
			}

			return on_both_survive(caster, target, others);
		}
	}

	if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::EnragingPain { stacks }))) = get_perk_mut!(&mut target, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::EnragingPain { .. }))) {
		*stacks += 1;
	}
	
	let crit_chance = skill.final_crit_chance(&caster);
	let is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => true,
		_ => false
	};

	if let Some(Perk::BellPlantLure(_)) = get_perk!(&target, Perk::BellPlantLure(_)) {
		if let Some(girl_stats) = &mut caster.girl_stats {
			girl_stats.lust += 12;
		}
	}

	for target_applier in skill.effects_target.iter() {
		let target_option = target_applier.apply_target(&mut caster, target, others, seed, is_crit);
		if let Some(target_alive) = target_option {
			target = target_alive;
		} else {
			return Some(caster); // target was dropped
		}
	}

	let Some(damage_range) = skill.calc_dmg(&caster, &target, is_crit) else {
		return on_both_survive(caster, target, others);
	};
	
	let mut damage = seed.gen_range(damage_range.min..=damage_range.max);
	if damage <= 0 {
		return on_both_survive(caster, target, others);
	}

	if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::FocusedSwings))) = get_perk!(&caster, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::FocusedSwings))) {
		if skill.name() == SkillName::FromEthel(EthelSkillName::Clash) || skill.name() == SkillName::FromEthel(EthelSkillName::Sever) {
			let toughness_debuff = TargetApplier::Buff {
				duration_ms: 4000,
				stat: ModifiableStat::TOUGHNESS,
				modifier: -25,
				apply_chance: Some(100),
			};

			let Some(target_survived) = toughness_debuff.apply_target(&mut caster, target, others, seed, is_crit) else { return Some(caster); }; // this should never happen but who knows
			target = target_survived;
		}
	}

	if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { stacks }))) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Relentless { .. }))) {
		caster.stamina_cur += damage / 2;
		*stacks += 1;
	}

	if let Some(Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Grudge { active }))) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser(Category_Bruiser::Grudge { .. }))) {
		if *active == true {
			*active = false;
			damage = (damage * 130) / 100;
		}
	}

	match &target.state {
		CharacterState::Idle
		| CharacterState::Charging { .. }
		| CharacterState::Recovering { .. } => {
			target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);

			if target.is_dead() {
				match target.girl_stats {
					Some(_) => {
						target.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000) };
						return on_both_survive(caster, target, others);
					}
					None => {
						return Some(caster); //drop target
					}
				}
			} else {
				return check_riposte(caster, &mut target, others, skill, seed);
			}
		}
		CharacterState::Stunned { .. } => {
			target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);

			if target.is_dead() {
				match target.girl_stats {
					Some(_) => {
						target.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(8000) };
						return on_both_survive(caster, target, others);
					}
					None => {
						return Some(caster); //drop target
					}
				}
			} else {
				return on_both_survive(caster, target, others);
			}
		}
		,
		CharacterState::Grappling(..) => {
			grappler_attacked(&mut caster, target, others, damage);
			return Some(caster);
		},
		CharacterState::Downed { .. } => return on_both_survive(caster, target, others), // damage is ignored on downed characters.
	}

	fn on_both_survive(caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<GUID, Entity>) -> Option<CombatCharacter> {
		others.insert(target.guid, Entity::Character(target));
		return Some(caster);
	}
	
	fn grappler_attacked(caster: &mut CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<GUID, Entity>, damage: isize) {
		let target_old_stamina_percent = target.stamina_cur as f64 / target.stamina_max as f64;
		target.stamina_cur -= damage;
		target.last_damager_guid = Some(caster.guid);
		let target_new_stamina_percent = target.stamina_cur as f64 / target.stamina_max as f64;
		
		let CharacterState::Grappling(grappling_state) = target.state else { panic!(); };

		target.state = CharacterState::Idle; // we move state in to allow borrowing in the next line
		let is_target_dead = target.is_dead();
		let _ = target.state;                // we move state out to make the compiler force us to move the correct one back in

		if is_target_dead {
			target.state = CharacterState::Idle; // this is set just to validate the variable "target", since grappling_state was moved out of it
			if let Some(target_entity) = target.entity_on_defeat() {
				others.insert(target_entity.guid(), target_entity);
			}

			match grappling_state.victim {
				GrappledGirlEnum::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.to_non_grappled();
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500) }; // girl is downed for 2.5s after being released from a grapple

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let ally_order: &mut usize = girl_ally.position_mut().order_mut();
						*ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::Character(girl_standing));
				}
				GrappledGirlEnum::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.to_non_grappled();

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::DefeatedGirl(girl_standing));
				}
			}
		}
		else if target_old_stamina_percent - target_new_stamina_percent >= 0.25 { // even if it doesn't kill, any attack that deals more than 25% of total health disables grappling
			match grappling_state.victim {
				GrappledGirlEnum::Alive(girl_alive) => {
					let mut girl_standing = girl_alive.to_non_grappled();
					girl_standing.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500) }; // alive girls are downed for 2.5s after being released from a grapple

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::Character(girl_standing));
				}
				GrappledGirlEnum::Defeated(girl_defeated) => {
					let mut girl_standing = girl_defeated.to_non_grappled();

					*girl_standing.position.order_mut() = 0;

					for girl_ally in iter_mut_allies_of!(girl_standing, others) {
						let mutref_ally_order = girl_ally.position_mut().order_mut();
						*mutref_ally_order += girl_standing.position.size();
					}

					others.insert(girl_standing.guid, Entity::DefeatedGirl(girl_standing));
				}
			}

			target.state = CharacterState::Idle;
			others.insert(target.guid, Entity::Character(target));
		}
		else {
			target.state = CharacterState::Grappling(grappling_state);
			others.insert(target.guid, Entity::Character(target));
		}
	}

	// returns caster if alive, target is passed by reference because it can't die here
	#[must_use]
	fn check_riposte(mut caster: CombatCharacter, target: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &OffensiveSkill, seed: &mut StdRng)
		-> Option<CombatCharacter> {
		
		if skill.can_be_riposted == false {
			return Some(caster);
		}
		
		let found = target.persistent_effects.iter().find(|effect|
				if let Riposte { .. } = effect {
					return true;
				} else {
					return false;
				});
		
		let Some(Riposte { duration_ms: _, dmg_multiplier, acc, crit } ) = found else {
			return Some(caster);
		};

		let final_hit_chance = OffensiveSkill::final_hit_chance_independent(*acc, target, &caster);

		if seed.base100_chance(final_hit_chance) == false {
			return Some(caster);
		}

		let is_crit: bool = match crit {
			CRITMode::CanCrit { crit_chance } => seed.base100_chance(OffensiveSkill::final_crit_chance_independent(*crit_chance, target)),
			CRITMode::NeverCrit => false,
		};

		let damage_range = OffensiveSkill::calc_dmg_independent(*dmg_multiplier, 0, target, &caster, is_crit);

		let damage = seed.gen_range(damage_range.min..=damage_range.max);
		if damage <= 0 {
			return Some(caster);
		}
		
		caster.stamina_cur -= damage;
		caster.last_damager_guid = Some(target.guid);
		
		if caster.is_alive() {
			return Some(caster);
		} else {
			if let Some(entity) = caster.entity_on_defeat() {
				others.insert(entity.guid(), entity);
			}

			return None;
		}
	}
}