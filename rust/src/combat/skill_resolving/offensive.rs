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
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::{DebuffApplier, TargetApplier};
use crate::combat::effects::persistent::{PersistentEffect};
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::ethel::skills::EthelSkillName;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::ModifiableStat;
use crate::combat::perk::Perk;
use crate::util::{Base100ChanceGenerator, GUID, TrackedTicks};

pub fn start(mut caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<GUID, Entity>, mut skill: OffensiveSkill, seed: &mut StdRng, recover_ms: Option<i64>) {
	let name = skill.skill_name;
	if matches!(name, SkillName::FromEthel(EthelSkillName::Clash) | SkillName::FromEthel(EthelSkillName::Pierce) | SkillName::FromEthel(EthelSkillName::Sever)) {
		if let Some(Perk::Ethel(EthelPerk::Poison_PoisonCoating)) = get_perk!(caster, Perk::Ethel(EthelPerk::Poison_PoisonCoating)) {
			let poison = TargetApplier::Poison {
				duration_ms: 3000,
				dmg_per_interval: 1,
				apply_chance: Some(100),
				additives: PersistentEffect::get_poison_additives(&caster.perks),
			};

			skill.effects_target.push(poison);
		}
	}
	else if name == SkillName::FromEthel(EthelSkillName::Challenge) {
		if let Some(Perk::Ethel(EthelPerk::Duelist_AlluringChallenger)) = get_perk!(caster, Perk::Ethel(EthelPerk::Duelist_AlluringChallenger)) {
			skill.effects_self.push(SelfApplier::Mark { duration_ms: 4000 });
		}
	}

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
		Some(chance) if seed.base100_chance(chance) => {
			if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
				*stacks -= 2;
			}

			true
		},
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
			// On-Miss Perks
			{
				if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Relentless { .. })) {
					*stacks = 0;
				}
				if let Some(Perk::Ethel(EthelPerk::Bruiser_Grudge { active })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Grudge { .. })) {
					*active = true;
				}
			}

			return on_both_survive(caster, target, others);
		}
	}

	// On-Hit Perks
	{
		if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks += 1;
		}

		if let Some(Perk::Ethel(EthelPerk::Bruiser_EnragingPain { stacks })) = get_perk_mut!(&mut target, Perk::Ethel(EthelPerk::Bruiser_EnragingPain { .. })) {
			*stacks += 1;
		}

		if let Some(Perk::Ethel(EthelPerk::Duelist_Release)) = get_perk!(target, Perk::Ethel(EthelPerk::Duelist_Release)) {
			if let Some(girl_stats) = &mut target.girl_stats {
				girl_stats.lust += 2;
			}
		}

		if let Some(Perk::BellPlantLure(_)) = get_perk!(target, Perk::BellPlantLure(_)) {
			if let Some(girl_stats) = &mut caster.girl_stats {
				girl_stats.lust += 12;
			}
		}

		if let Some(Perk::Nema(NemaPerk::Grumpiness)) = get_perk!(target, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000,
				stat: ModifiableStat::SPD,
				stat_increase: 15,
			};

			let toughness_debuff = TargetApplier::Buff {
				duration_ms: 4000,
				stat: ModifiableStat::TOUGHNESS,
				stat_increase: 15,
			};

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
	
	let crit_chance = skill.final_crit_chance(&caster);
	let mut is_crit = match crit_chance {
		Some(chance) if seed.base100_chance(chance) => {
			if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
				*stacks -= 2;
			}
			if let Some(Perk::Ethel(EthelPerk::Crit_StaggeringForce)) = get_perk!(caster, Perk::Ethel(EthelPerk::Crit_StaggeringForce)) {
				let staggering_force = TargetApplier::Debuff(DebuffApplier::StaggeringForce { duration_ms: 4000, apply_chance: Some(100) });
				let Some(target_survived) = staggering_force.apply_target(&mut caster, target, others, seed, false) else { return Some(caster); }; // this should never happen but who knows
				target = target_survived;
			}

			true
		},
		_ => false
	};

	if let Some(Perk::Ethel(EthelPerk::Crit_Bold { used })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Crit_Bold { .. })) {
		if *used == false {
			*used = true;
			is_crit = true;
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
	
	let mut damage = seed.gen_range(damage_range);
	if damage <= 0 {
		return on_both_survive(caster, target, others);
	}

	// Damage-affecting perks
	{
		if let Some(Perk::Ethel(EthelPerk::Bruiser_FocusedSwings)) = get_perk!(caster, Perk::Ethel(EthelPerk::Bruiser_FocusedSwings)) {
			if skill.name() == SkillName::FromEthel(EthelSkillName::Clash) || skill.name() == SkillName::FromEthel(EthelSkillName::Sever) {
				let toughness_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
					duration_ms: 4000,
					stat: ModifiableStat::TOUGHNESS,
					stat_decrease: 25,
					apply_chance: Some(100),
				});

				let Some(target_survived) = toughness_debuff.apply_target(&mut caster, target, others, seed, is_crit) else { return Some(caster); }; // this should never happen but who knows
				target = target_survived;
			}
		}

		if let Some(Perk::Ethel(EthelPerk::Debuffer_GoForTheEyes)) = get_perk!(caster, Perk::Ethel(EthelPerk::Debuffer_GoForTheEyes)) {
			damage = (damage * 90) / 100;
			let random_debuff = TargetApplier::Debuff(DebuffApplier::Standard {
				duration_ms: 4000,
				stat: ModifiableStat::get_non_girl_random(seed),
				stat_decrease: 10,
				apply_chance: Some(100),
			});

			let target_survived = random_debuff.apply_target(&mut caster, target, others, seed, false);
			if let Some(target_survived) = target_survived {
				target = target_survived;
			} else {
				return Some(caster); // target died and was dropped
			}
		}

		// does target have unnerving aura and caster is debuffed?
		if let Some(Perk::Ethel(EthelPerk::Debuffer_UnnervingAura)) = get_perk!(target, Perk::Ethel(EthelPerk::Debuffer_UnnervingAura)) {
			if caster.persistent_effects.iter().any(|effect| return if let PersistentEffect::Debuff(_) = effect { true } else { false }) {
				damage = (damage * 75) / 100;
			}
		}

		if let Some(Perk::Ethel(EthelPerk::Bruiser_Grudge { active })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Grudge { .. })) {
			if *active == true {
				*active = false;
				damage = (damage * 130) / 100;
			}
		}

		if let Some(Perk::Ethel(EthelPerk::Debuffer_NoQuarters)) = get_perk!(caster, Perk::Ethel(EthelPerk::Debuffer_NoQuarters)) {
			let mut debuff_count = 0;
			for effect in target.persistent_effects.iter() {
				if let PersistentEffect::Debuff(_) = effect {
					debuff_count += 1;
				}
			}

			let mut dmg_modifier = isize::clamp(debuff_count * 50, 0, 250);
			if let CharacterState::Stunned { .. } = target.state {
				dmg_modifier += 100;
			}

			dmg_modifier /= 10;
			damage = (damage * (100 + dmg_modifier)) / 100;
		}

		if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Relentless { .. })) {
			caster.stamina_cur += (damage * 130) / 100;
			*stacks += 1;
		}

		if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk_mut!(target, Perk::Nema(NemaPerk::BattleMage_Trust { .. })) {
			*accumulated_ms = 0;
		}
		
		if let Some(Perk::Nema(NemaPerk::AOE_Hatred { stacks })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::AOE_Hatred { .. })) {
			damage = (damage * (100 + 15 * stacks.get() as isize)) / 100;
			stacks.set(0);
		}

		if let Some(Perk::Nema(NemaPerk::AOE_Hatred { stacks })) = get_perk_mut!(target, Perk::Nema(NemaPerk::AOE_Hatred { .. })) {
			*stacks += 1;
		}
	}

	match &target.state {
		CharacterState::Idle
		| CharacterState::Charging   {..}
		| CharacterState::Recovering {..}
		| CharacterState::Stunned    {..} => {
			target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);

			if target.stamina_dead() {
				target.do_on_zero_stamina(Some(&mut caster), others, seed);
				return None;
			} else {
				return on_both_survive(caster, target, others);
			}
		},
		CharacterState::Grappling(..) => {
			grappler_attacked(&mut caster, target, others, damage, seed);
			return Some(caster);
		},
		CharacterState::Downed { .. } => return on_both_survive(caster, target, others), // damage is ignored on downed characters.
	}

	fn on_both_survive(caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<GUID, Entity>) -> Option<CombatCharacter> {
		others.insert(target.guid, Entity::Character(target));
		return Some(caster);
	}
	
	fn grappler_attacked(caster: &mut CombatCharacter, mut target: CombatCharacter, others: &mut HashMap<GUID, Entity>, damage: isize, seed: &mut StdRng) {
		let target_old_stamina_percent = target.stamina_cur as f64 / target.get_max_stamina() as f64;
		target.stamina_cur -= damage;
		target.last_damager_guid = Some(caster.guid);
		let target_new_stamina_percent = target.stamina_cur as f64 / target.get_max_stamina() as f64;

		if target.stamina_dead() {
			target.do_on_zero_stamina(Some(caster), others, seed);
			return;
		}

		let CharacterState::Grappling(grappling_state) = target.state else { panic!(); };
		if target_old_stamina_percent - target_new_stamina_percent >= 0.25 { // even if it doesn't kill, any attack that deals more than 25% of total health disables grappling
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
			others.insert(target.guid(), Entity::Character(target));
		}
	}

	// returns caster if alive, target is passed by reference because it can't die here
	#[must_use]
	fn check_riposte(mut caster: CombatCharacter, target_as_riposter: &mut CombatCharacter, others: &mut HashMap<GUID, Entity>, skill: &OffensiveSkill, seed: &mut StdRng)
		-> Option<CombatCharacter> {
		
		if skill.can_be_riposted == false {
			return Some(caster);
		}
		
		let found = target_as_riposter.persistent_effects.iter().find_map(|effect|
				if let Riposte { .. } = effect {
					return Some(effect.clone());
				} else {
					return None;
				});
		
		let Some(Riposte { duration_ms: _, dmg_multiplier, acc, crit } ) = found else {
			return Some(caster);
		};

		let final_hit_chance = OffensiveSkill::final_hit_chance_independent(acc, target_as_riposter, &caster);

		if seed.base100_chance(final_hit_chance) == false {
			return Some(caster);
		}

		if let Some(Perk::Ethel(EthelPerk::Duelist_Release)) = get_perk!(target_as_riposter, Perk::Ethel(EthelPerk::Duelist_Release)) {
			if let Some(girl_stats) = &mut target_as_riposter.girl_stats {
				if girl_stats.lust > 100 {
					girl_stats.lust -= 4;
				}
			}
		}

		if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(target_as_riposter, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks += 1;
		}

		let is_crit: bool = match crit {
			CRITMode::CanCrit { crit_chance } => {
				if seed.base100_chance(OffensiveSkill::final_crit_chance_independent(crit_chance, target_as_riposter)) {
					if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(target_as_riposter, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
						*stacks -= 2;
					}

					true
				} else {
					false
				}
			},
			CRITMode::NeverCrit => false,
		};

		let damage_range = OffensiveSkill::calc_dmg_independent(dmg_multiplier, 0, target_as_riposter, &caster, is_crit);

		let damage = seed.gen_range(damage_range);
		if damage <= 0 {
			return Some(caster);
		}
		
		caster.stamina_cur -= damage;
		caster.last_damager_guid = Some(target_as_riposter.guid);

		if caster.stamina_dead() {
			caster.do_on_zero_stamina(Some(target_as_riposter), others, seed);
			return None;
		} else {
			return Some(caster);
		}
	}
}