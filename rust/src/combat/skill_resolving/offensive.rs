use std::collections::{HashMap, HashSet};
use std::num::{NonZeroU16, NonZeroU8};

use comfy_bounded_ints::prelude::{SqueezeTo_i64, SqueezeTo_u8};
use gdnative::godot_error;
use gdnative::log::godot_warn;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use util::any_matches;
use util::prelude::Touch;
use uuid::Uuid;

use combat::effects::onSelf::SelfApplier;
use combat::effects::onTarget::{DebuffApplierKind, TargetApplier};
use combat::effects::persistent::PersistentEffect;
use combat::effects::persistent::PersistentEffect::Riposte;
use combat::entity::*;
use combat::entity::character::*;
use combat::entity::data::girls::ethel::perks::*;
use combat::entity::data::girls::ethel::skills::EthelSkill;
use combat::entity::data::girls::nema::perks::NemaPerk;
use combat::entity::data::skill_name::SkillName;
use combat::entity::girl::*;
use combat::perk::Perk;
use combat::skill_types::*;
use combat::skill_types::offensive::OffensiveSkill;

use crate::combat;
use crate::combat::entity::stat::ToughnessReduction;
use crate::combat::perk::{get_perk_mut, has_perk};
use crate::combat::stat::DynamicStat;
use crate::misc::{Base100ChanceGenerator, SaturatedU64, ToSaturatedI64, ToSaturatedU64, TrackedTicks};

pub fn start(mut caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<Uuid, Entity>,
             mut skill: OffensiveSkill, rng: &mut Xoshiro256PlusPlus, recover_ms: Option<SaturatedU64>) {
	let name = skill.skill_name;
	if matches!(name, SkillName::FromEthel(EthelSkill::Clash | EthelSkill::Pierce | EthelSkill::Sever)) 
		&& has_perk!(caster, Perk::Ethel(EthelPerk::Poison_PoisonCoating)) {
		let poison = TargetApplier::Poison {
			duration_ms: 3000.to_sat_u64(),
			poison_per_interval: unsafe { NonZeroU8::new_unchecked(1) },
			apply_chance: Some(unsafe { NonZeroU16::new_unchecked(100) }),
			additives: PersistentEffect::get_poison_additives(&caster.perks),
		};
		
		skill.effects_target.convert_to_owned().push(poison);
	} 
	else if matches!(name, SkillName::FromEthel(EthelSkill::Challenge)) 
		&& has_perk!(caster, Perk::Ethel(EthelPerk::Duelist_AlluringChallenger)) {
		skill.effects_self.convert_to_owned()
			.push(SelfApplier::Mark { duration_ms: 4000.to_sat_u64() });
	}

	process_self_effects_and_costs(&mut caster, others, &skill, rng, recover_ms);

	if !skill.multi_target {
		// caster may die due to riposte so we get him back as an option
		resolve_target(caster, target, others, &skill, rng) 
			.map(|survived| { others.insert(survived.guid, Entity::Character(survived)) });
		return;
	}

	let targets_guid = iter_enemies_of!(caster, others)
		.filter_map(|possible_target| 
			possible_target.position().contains_any(&skill.target_positions).then(|| possible_target.guid()))
		.collect::<HashSet<Uuid>>();

	let Some(mut caster) = resolve_target(caster, target, others, &skill, rng) 
		else { return; }; // caster may die due to riposte so we get him back as an option
	
	for guid in targets_guid { // for now, we only support skills on characters
		match others.remove(&guid) {
			Some(Entity::Character(target)) => {
				if let Some(survived) = resolve_target(caster, target, others, &skill, rng) {
					caster = survived
				} else {
					return;
				}
			},
			Some(entity) => {
				godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but the entity was not a character.\n\
						Entity: {entity:?}", util::full_fn_name(&start));
				others.insert(entity.guid(), entity);
			},
			None => {
				godot_warn!("{}(): Trying to apply skill to character with guid {guid:?}, but it was not found.",
						util::full_fn_name(&start));
				return;
			}
		}
	}

	others.insert(caster.guid, Entity::Character(caster));
}

fn process_self_effects_and_costs(caster: &mut CombatCharacter, others: &mut HashMap<Uuid, Entity>,
                                  skill: &OffensiveSkill, rng: &mut Xoshiro256PlusPlus, recover_ms: Option<SaturatedU64>) {
	recover_ms.map(|ms| caster.state = CharacterState::Recovering { ticks: TrackedTicks::from_milliseconds(ms.to_sat_u64()) });
	caster.increment_skill_counter(skill.name());

	let is_crit = skill
		.final_crit_chance(caster)
		.is_some_and(|chance| rng.base100_chance(chance));

	if is_crit && let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
		*stacks -= 2;
	}

	skill.effects_self.iter().for_each(|applier|
		applier.apply(caster, others, rng, is_crit));
}

/// returns caster if they are alive, otherwise we drop caster
#[must_use]
fn resolve_target(mut caster: CombatCharacter, mut target: CombatCharacter,
                  others: &mut HashMap<Uuid, Entity>, skill: &OffensiveSkill, rng: &mut Xoshiro256PlusPlus)
	-> Option<CombatCharacter> {
	
	let is_miss = skill
		.final_hit_chance(&caster, &target)
		.is_some_and(|chance| !rng.base100_chance(chance));
	
	if is_miss {
		if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Relentless { .. })) {
			stacks.set(0);
		}
		if let Some(Perk::Ethel(EthelPerk::Bruiser_Grudge { active })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Grudge { .. })) {
			*active = true;
		}

		return on_both_survive(caster, target, others);
	}

	// On-Hit Perks
	{
		if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks += 1;
		}

		if let Some(Perk::Ethel(EthelPerk::Bruiser_EnragingPain { stacks })) = get_perk_mut!(&mut target, Perk::Ethel(EthelPerk::Bruiser_EnragingPain { .. })) {
			*stacks += 1;
		}

		if has_perk!(target, Perk::Ethel(EthelPerk::Duelist_Release)) {
			target.girl_stats.touch(|girl| *girl.lust += 2);
		}

		if has_perk!(target, Perk::BellPlantLure(_)) {
			caster.girl_stats.touch(|girl| *girl.lust += 12);
		}

		if has_perk!(target, Perk::Nema(NemaPerk::Grumpiness)) {
			let spd_buff = TargetApplier::Buff {
				duration_ms: 3000.to_sat_u64(),
				stat: DynamicStat::Speed,
				stat_increase: unsafe { NonZeroU16::new_unchecked(15) },
			};

			let toughness_debuff = TargetApplier::Buff {
				duration_ms: 4000.to_sat_u64(),
				stat: DynamicStat::Toughness,
				stat_increase: unsafe { NonZeroU16::new_unchecked(15) },
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

	let is_crit = skill
		.final_crit_chance(&caster)
		.is_some_and(|chance| rng.base100_chance(chance))
		|| {
			if let Some(Perk::Ethel(EthelPerk::Crit_Bold { used })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Crit_Bold { .. }))
				&& *used == false {
				*used = true;
				true
			} else {
				false
			}
		};

	if is_crit {
		if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(caster, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks -= 2;
		}

		if has_perk!(caster, Perk::Ethel(EthelPerk::Crit_StaggeringForce)) {
			let staggering_force = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: Some(unsafe { NonZeroU16::new_unchecked(100) }),
				applier_kind: DebuffApplierKind::StaggeringForce,
			};

			if let Some(target_survived) = staggering_force.apply_target(&mut caster, target, others, rng, false) {
				target = target_survived;
			} else {
				// this should never happen but who knows
				godot_error!("{}(): StaggeringForce debuff was applied to target, but target died and was dropped.", 
					util::full_fn_name(&resolve_target));
				return Some(caster);
			}
		}
	}

	for applier in skill.effects_target.iter() {
		if let Some(target_alive) = applier.apply_target(&mut caster, target, others, rng, is_crit) {
			target = target_alive;
		} else {
			return Some(caster); // target died and was dropped
		}
	}

	let Some(damage_range) = skill.calc_dmg(&caster, &target, is_crit)
		else { return on_both_survive(caster, target, others); };
	
	let damage = {
		let mut temp = rng
			.gen_range(damage_range.bound_lower()..=damage_range.bound_upper())
			.to_sat_i64();

		if *temp <= 0 {
			return on_both_survive(caster, target, others);
		}

		if matches!(skill.name(), SkillName::FromEthel(EthelSkill::Clash | EthelSkill::Sever)) 
			&& has_perk!(caster, Perk::Ethel(EthelPerk::Bruiser_FocusedSwings)) {
			let toughness_debuff = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: Some(unsafe { NonZeroU16::new_unchecked(100) }),
				applier_kind: DebuffApplierKind::Standard {
					stat: DynamicStat::Toughness,
					stat_decrease: unsafe { NonZeroU16::new_unchecked(25) },
				}
			};

			if let Some(survived) = toughness_debuff.apply_target(&mut caster, target, others, rng, is_crit) {
				target = survived;
			} else {
				// this should never happen but who knows
				godot_error!("{}(): FocusedSwings debuff was applied to target, but target died and was dropped.", 
					util::full_fn_name(&resolve_target));
				return Some(caster);
			}
		}

		if has_perk!(caster, Perk::Ethel(EthelPerk::Debuffer_GoForTheEyes)) {
			temp *= 9;
			temp /= 10;

			let random_debuff = TargetApplier::Debuff {
				duration_ms: 4000.to_sat_u64(),
				apply_chance: Some(unsafe { NonZeroU16::new_unchecked(100) }),
				applier_kind: DebuffApplierKind::Standard {
					stat: DynamicStat::get_random(rng),
					stat_decrease: unsafe { NonZeroU16::new_unchecked(10) },
				}
			};

			if let Some(survived) = random_debuff.apply_target(&mut caster, target, others, rng, false) {
				target = survived;
			} else {
				// this should never happen but who knows
				godot_error!("{}(): GoForTheEyes debuff was applied to target, but target died and was dropped.", 
					util::full_fn_name(&resolve_target));
				return Some(caster);
			}
		}

		// does target have unnerving aura and caster is debuffed?
		if has_perk!(target, Perk::Ethel(EthelPerk::Debuffer_UnnervingAura))
			&& any_matches!(caster.persistent_effects, PersistentEffect::Debuff{..}) {
			temp *= 75;
			temp /= 100;
		}

		if let Some(Perk::Ethel(EthelPerk::Bruiser_Grudge { active })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Grudge { .. }))
			&& *active == true {
			*active = false;
			temp *= 13;
			temp /= 10;
		}

		if has_perk!(caster, Perk::Ethel(EthelPerk::Debuffer_NoQuarters)) {
			let debuff_count = target.persistent_effects.iter()
			                         .filter(|effect| matches!(effect, PersistentEffect::Debuff{..}))
			                         .count().squeeze_to_i64();

			let dmg_modifier = {
				let mut temp_dmg_mod = i64::clamp(debuff_count * 50, 0, 250).to_sat_i64();
				if let CharacterState::Stunned { .. } = target.state {
					temp_dmg_mod += 100;
				}

				temp_dmg_mod /= 10;
				temp_dmg_mod.get()
			};

			temp *= 100 + dmg_modifier;
			temp /= 100;
		}

		if let Some(Perk::Nema(NemaPerk::AOE_Hatred { stacks })) = get_perk_mut!(caster, Perk::Nema(NemaPerk::AOE_Hatred { .. })) {
			temp *= 100 + 15 * stacks.squeeze_to_i64();
			temp /= 100;
			stacks.set(0);
		}
		
		temp.get()
	};
	
	if damage <= 0 {
		return on_both_survive(caster, target, others);
	}

	if let Some(Perk::Ethel(EthelPerk::Bruiser_Relentless { stacks })) = get_perk_mut!(&mut caster, Perk::Ethel(EthelPerk::Bruiser_Relentless { .. })) {
		*stacks += 1;
		*caster.stamina_cur += (damage * 3) / 10;
	}

	if let Some(Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms })) = get_perk_mut!(target, Perk::Nema(NemaPerk::BattleMage_Trust { .. })) {
		accumulated_ms.set(0);
	}

	if let Some(Perk::Nema(NemaPerk::AOE_Hatred { stacks })) = get_perk_mut!(target, Perk::Nema(NemaPerk::AOE_Hatred { .. })) {
		*stacks += 1;
	}

	return match &target.state {
		CharacterState::Idle
		| CharacterState::Charging { .. }
		| CharacterState::Recovering { .. }
		| CharacterState::Stunned { .. } => {
			*target.stamina_cur -= damage;
			target.last_damager_guid = Some(caster.guid);

			if target.stamina_dead() {
				target.do_on_zero_stamina(Some(&mut caster), others, rng);
				None
			} else {
				on_both_survive(caster, target, others)
			}
		},
		CharacterState::Grappling(..) => {
			let CharacterState::Grappling(grappling_state) = std::mem::replace(&mut target.state, CharacterState::Idle)
				else { unreachable!(); };
			
			grappler_attacked(&mut caster, target, grappling_state, others, damage, rng);
			Some(caster)
		},
		CharacterState::Downed { .. } => { // damage is ignored on downed characters.
			on_both_survive(caster, target, others)
		},
	};

	fn on_both_survive(caster: CombatCharacter, target: CombatCharacter, others: &mut HashMap<Uuid, Entity>) -> Option<CombatCharacter> {
		others.insert(target.guid, Entity::Character(target));
		return Some(caster);
	}
	
	fn grappler_attacked(caster: &mut CombatCharacter, mut target: CombatCharacter, detached_state: GrapplingState,
	                     others: &mut HashMap<Uuid, Entity>, damage: i64, rng: &mut Xoshiro256PlusPlus) {
		target.last_damager_guid = Some(caster.guid);
		
		let target_old_stamina_percent = {
			let mut temp = target.stamina_cur.to_sat_i64();
			temp *= 100;
			temp /= target.max_stamina().get();
			temp.squeeze_to_u8()
		};

		*target.stamina_cur -= damage;
		let target_new_stamina_percent = {
			let mut temp = target.stamina_cur.to_sat_i64();
			temp *= 100;
			temp /= target.max_stamina().get();
			temp.squeeze_to_u8()
		};

		if target.stamina_dead() {
			target.do_on_zero_stamina(Some(caster), others, rng);
			return;
		}

		if u8::saturating_sub(target_old_stamina_percent, target_new_stamina_percent) < 25 {
			target.state = CharacterState::Grappling(detached_state);
			others.insert(target.guid(), Entity::Character(target));
			return;
		}

		// even if it doesn't kill, any attack that deals more than 25% of total health disables grappling
		match detached_state.victim {
			GrappledGirlEnum::Alive(girl_alive) => {
				let girl_entity = {
					let mut temp = girl_alive.into_non_grappled();
					
					// alive girls are downed for 2.5s after being released from a grapple
					temp.state = CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2500.to_sat_u64()) };
					temp.position.order = 0.into();
					temp
				};

				let girl_size = girl_entity.position.size;
				iter_mut_allies_of!(girl_entity, others).for_each(|ally| 
					ally.position_mut().order += girl_size);

				others.insert(girl_entity.guid, Entity::Character(girl_entity));
			}
			GrappledGirlEnum::Defeated(girl_defeated) => {
				let girl_entity = {
					let mut temp = girl_defeated.into_non_grappled();
					temp.position.order = 0.into();
					temp
				};
				
				let girl_size = girl_entity.position.size;
				iter_mut_allies_of!(girl_entity, others).for_each(|ally| 
					ally.position_mut().order += girl_size);

				others.insert(girl_entity.guid, Entity::DefeatedGirl(girl_entity));
			}
		}

		target.state = CharacterState::Idle;
		others.insert(target.guid, Entity::Character(target));
	}

	// returns caster if alive, target is passed by reference because it can't die here
	#[must_use]
	fn check_riposte(mut caster: CombatCharacter, target_as_riposter: &mut CombatCharacter,
	                 others: &mut HashMap<Uuid, Entity>, skill: &OffensiveSkill, rng: &mut Xoshiro256PlusPlus)
		-> Option<CombatCharacter> {
		
		if !skill.can_be_riposted {
			return Some(caster);
		}
		
		let Some(Riposte { skill_power: dmg_mult, acc_mode, crit_mode, .. } ) = 
			target_as_riposter.persistent_effects.iter()
				.find(|effect| matches!(effect, Riposte {..}))
			else {
				return Some(caster);
			};
		
		let dmg_mult = *dmg_mult;
		let crit_mode = *crit_mode;

		let is_miss = 'miss: {
			let acc =
				match acc_mode {
					ACCMode::NeverMiss => break 'miss false,
					ACCMode::CanMiss { acc } => acc,
				};
			
			let hit_chance = OffensiveSkill::final_hit_chance_independent(*acc, target_as_riposter, &caster);
			rng.base100_chance(hit_chance) == false
		};

		if is_miss {
			return Some(caster);
		}

		if has_perk!(target_as_riposter, Perk::Ethel(EthelPerk::Duelist_Release)) {
			target_as_riposter.girl_stats.touch(|girl| {
				if girl.lust.get() > 100 {
					*girl.lust -= 4;
				}
			});
		}

		if let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(target_as_riposter, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks += 1;
		}

		let is_crit: bool = match crit_mode {
			CRITMode::NeverCrit => false,
			CRITMode::CanCrit { chance } => {
				let final_crit_chance = OffensiveSkill::final_crit_chance_independent(chance, target_as_riposter);
				rng.base100_chance(final_crit_chance)
			},
		};
		
		if is_crit && let Some(Perk::Ethel(EthelPerk::Crit_Vicious { stacks })) = get_perk_mut!(target_as_riposter, Perk::Ethel(EthelPerk::Crit_Vicious { .. })) {
			*stacks -= 2;
		}

		let dmg_range = OffensiveSkill::calc_dmg_independent(
			dmg_mult, ToughnessReduction::new(0), target_as_riposter, &caster, is_crit);

		let damage = rng.gen_range(dmg_range.bound_lower()..=dmg_range.bound_upper());
		if damage <= 0 {
			return Some(caster);
		}
		
		*caster.stamina_cur -= damage;
		caster.last_damager_guid = Some(target_as_riposter.guid);

		return if caster.stamina_alive() {
			Some(caster)
		} else {
			caster.do_on_zero_stamina(Some(target_as_riposter), others, rng);
			None
		};
	}
}