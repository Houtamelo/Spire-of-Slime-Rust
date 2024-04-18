#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;
use crate::combat::graphics::action_animation::character_position::DefensivePadding;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::skill_types::AllyRequirement;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefensiveSkill {
	pub skill_name: SkillName,
	pub recovery_ms: SaturatedU64,
	pub charge_ms  : SaturatedU64,
	pub crit_mode: CRITMode,
	pub effects_self  : DynamicArray<SelfApplier>,
	pub effects_target: DynamicArray<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub ally_requirement: AllyRequirement,
	pub multi_target: bool,
	pub use_counter : UseCounter,
}

impl DefensiveSkill {
	pub fn final_crit_chance(&self, caster: &CombatCharacter) -> Option<PercentageU8> {
		return match self.crit_mode {
			CRITMode::CanCrit { chance } => {
				let final_chance = {
					let mut temp = chance.to_sat_i64();
					temp += caster.dyn_stat::<CritRate>().get();
					temp.to_percent_u8()
				};
				
				Some(final_chance)
			}
			CRITMode::NeverCrit => None,
		};
	}
	pub fn padding(&self) -> DefensivePadding {
		todo!()
	}
}

impl SkillData for DefensiveSkill {
	fn name(&self) -> SkillName { return self.skill_name  ; }
	fn recovery_ms(&self) -> &SaturatedU64 { return &self.recovery_ms; }
	fn charge_ms  (&self) -> &SaturatedU64 { return &self.charge_ms  ; }
	fn crit(&self) -> &CRITMode { return &self.crit_mode; }
	fn effects_self    (&self) -> &[SelfApplier]   { return &self.effects_self    ; }
	fn effects_target  (&self) -> &[TargetApplier] { return &self.effects_target  ; }
	fn caster_positions(&self) -> &PositionMatrix  { return &self.caster_positions; }
	fn target_positions(&self) -> &PositionMatrix  { return &self.target_positions; }
	fn multi_target(&self) -> &bool       { return &self.multi_target; }
	fn use_counter (&self) -> &UseCounter { return &self.use_counter ; }
}