use std::ops::{RangeInclusive};
use proc_macros::get_perk;
use houta_utils::prelude::BoundUSize;
use crate::combat::ModifiableStat;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::perk::Perk;
use crate::combat::skill_types::*;

#[derive(Debug, Clone)]
pub struct OffensiveSkill {
	pub skill_name: SkillName,
	pub recovery_ms     : i64,
	pub charge_ms       : i64,
	pub can_be_riposted : bool,
	pub acc_mode        : ACCMode,
	pub dmg             : DMGMode,
	pub crit            : CRITMode,
	pub custom_modifiers: Vec<CustomOffensiveModifier>,
	pub effects_self    : Vec<SelfApplier>,
	pub effects_target  : Vec<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target    : bool,
	pub use_counter     : UseCounter,
}

impl OffensiveSkill {
	pub fn calc_dmg(&self, caster: &CombatCharacter, target: &CombatCharacter, crit: bool) -> Option<RangeInclusive<isize>> {
		let (power, toughness_reduction) = match self.dmg {
			DMGMode::Power { power, toughness_reduction } => { (power, toughness_reduction) } 
			DMGMode::NoDamage => { return None; }
		};
		
		let (mut dmg_min, mut dmg_max) = (*caster.dmg.start() as isize, *caster.dmg.end() as isize);
		
		let base_toughness = target.get_stat(ModifiableStat::TOUGHNESS);
		let min_toughness = isize::min(base_toughness, 0);
		let final_toughness = isize::max(min_toughness, base_toughness - toughness_reduction);
		
		let total_power = power * caster.get_stat(ModifiableStat::POWER) * (100 - final_toughness);

		dmg_max = (dmg_max * total_power) / 1000000;
		dmg_min = isize::min((dmg_min * total_power) / 1000000, dmg_max);
		
		if crit {
			dmg_max = (dmg_max * 150) / 100;
			dmg_min = (dmg_min * 150) / 100;
		}
		
		return Some(dmg_min..=dmg_max);
	}
	
	pub fn calc_dmg_independent(power: isize, toughness_reduction: isize, caster: &CombatCharacter, target: &CombatCharacter, crit: bool) -> RangeInclusive<isize> {
		let (mut dmg_min, mut dmg_max) = (*caster.dmg.start() as isize, *caster.dmg.end() as isize);
		
		let base_toughness = target.get_stat(ModifiableStat::TOUGHNESS);
		let min_toughness = isize::min(base_toughness, 0);
		let final_toughness = isize::max(min_toughness, base_toughness - toughness_reduction);
		
		let total_power = power * caster.get_stat(ModifiableStat::POWER) * (100 - final_toughness);

		dmg_max = (dmg_max * total_power) / 1000000;
		dmg_min = isize::min((dmg_min * total_power) / 1000000, dmg_max);
		
		if crit {
			dmg_max = (dmg_max * 150) / 100;
			dmg_min = (dmg_min * 150) / 100;
		}
		
		return dmg_min..=dmg_max;
	}
	
	pub fn final_hit_chance(&self, caster: &CombatCharacter, target: &CombatCharacter) -> Option<BoundUSize<0, 100>> {
		let acc = match self.acc_mode {
			ACCMode::CanMiss { acc } => { acc }
			ACCMode::NeverMiss => { return None; }
		};
		
		return Some(OffensiveSkill::final_hit_chance_independent(acc, caster, target));
	}

	pub fn final_hit_chance_independent(mut base_acc: isize, caster: &CombatCharacter, target: &CombatCharacter) -> BoundUSize<0, 100> {
		if let Some(Perk::Nema(NemaPerk::Poison_Disbelief)) = get_perk!(target, Perk::Nema(NemaPerk::Poison_Disbelief)) {
			if caster.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Poison {..})) {
				base_acc -= 20;
			}
		}

		return (base_acc + caster.get_stat(ModifiableStat::ACC) - target.get_stat(ModifiableStat::DODGE)).into();
	}
	
	pub fn final_crit_chance(&self, caster: &CombatCharacter) -> Option<BoundUSize<0, 100>> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit_chance: crit } => { crit }
			CRITMode::NeverCrit => { return None; }
		};
		
		return Some(OffensiveSkill::final_crit_chance_independent(crit, caster));
	}
	
	pub fn final_crit_chance_independent(base_crit: isize, caster: &CombatCharacter) -> BoundUSize<0, 100> {
		return (base_crit + caster.get_stat(ModifiableStat::CRIT)).into();
	}
}

impl SkillTrait for OffensiveSkill {
	fn name            (&self) -> SkillName           { return self.skill_name       ; }
	fn recovery_ms     (&self) -> &i64                { return &self.recovery_ms     ; }
	fn charge_ms       (&self) -> &i64                { return &self.charge_ms       ; }
	fn crit            (&self) -> &CRITMode           { return &self.crit            ; }
	fn effects_self    (&self) -> &Vec<SelfApplier>   { return &self.effects_self    ; }
	fn effects_target  (&self) -> &Vec<TargetApplier> { return &self.effects_target  ; }
	fn caster_positions(&self) -> &PositionMatrix     { return &self.caster_positions; }
	fn target_positions(&self) -> &PositionMatrix     { return &self.target_positions; }
	fn multi_target    (&self) -> &bool               { return &self.multi_target    ; }
	fn use_counter     (&self) -> &UseCounter         { return &self.use_counter     ; }
}

#[derive(Debug, Clone)]
pub enum CustomOffensiveModifier {
	BonusVsMarked {
		power: isize,
		acc: isize,
		crit: isize,
	}
}
