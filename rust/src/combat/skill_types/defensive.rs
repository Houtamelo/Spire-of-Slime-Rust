use houta_utils::prelude::BoundUSize;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::ModifiableStat;
use crate::combat::skill_types::*;

#[derive(Debug, Clone)]
pub struct DefensiveSkill {
	pub skill_name: SkillName,
	pub recovery_ms     : i64,
	pub charge_ms       : i64,
	pub crit            : CRITMode,
	pub effects_self    : Vec<SelfApplier>,
	pub effects_target  : Vec<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub ally_requirement: AllyRequirement,
	pub multi_target    : bool,
	pub use_counter     : UseCounter,
}

impl DefensiveSkill {
	pub fn calc_crit_chance(&self, caster: &CombatCharacter) -> Option<BoundUSize<0, 100>> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit_chance: crit } => crit,
			CRITMode::NeverCrit => return None,
		};

		return Some((crit + caster.get_stat(ModifiableStat::CRIT)).into());
	}
}

impl SkillTrait for DefensiveSkill {
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