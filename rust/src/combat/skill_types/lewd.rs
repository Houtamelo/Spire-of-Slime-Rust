use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::ModifiableStat;
use crate::combat::skill_types::*;
use crate::util::I_Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LewdSkill {
	pub skill_name: SkillName,
	pub recovery_ms     : i64,
	pub charge_ms       : i64,
	pub acc_mode        : ACCMode,
	pub dmg             : DMGMode,
	pub crit            : CRITMode,
	pub effects_self    : Vec<SelfApplier>,
	pub effects_target  : Vec<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target    : bool,
	pub use_counter     : UseCounter,
}

impl LewdSkill {
	pub fn calc_dmg(&self, caster: &CombatCharacter, target: &CombatCharacter, crit: bool) -> Option<I_Range> {
		let (power, toughness_reduction) = match self.dmg {
			DMGMode::Power { power, toughness_reduction } => { (power, toughness_reduction) }
			DMGMode::NoDamage => { return None; }
		};

		let (mut dmg_min, mut dmg_max) = (caster.dmg.min, caster.dmg.max);

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

		return Some(I_Range::new(dmg_min, dmg_max));
	}

	pub fn calc_hit_chance(&self, caster: &CombatCharacter, target: &CombatCharacter) -> Option<isize> {
		let acc = match self.acc_mode {
			ACCMode::CanMiss { acc } => { acc }
			ACCMode::NeverMiss => { return None; }
		};

		return Some(acc + caster.get_stat(ModifiableStat::ACC) - target.get_stat(ModifiableStat::DODGE));
	}
	
	pub fn calc_crit_chance(&self, caster: &CombatCharacter) -> Option<isize> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit_chance: crit } => { crit }
			CRITMode::NeverCrit => { return None; }
		};

		return Some(crit + caster.get_stat(ModifiableStat::CRIT));
	}
}

impl SkillTrait for LewdSkill {
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