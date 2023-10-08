use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::ModifiableStat;
use crate::combat::skills::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefensiveSkill {
	pub crit: CRITMode,
	pub effects_self: Vec<SelfApplier>,
	pub effects_target: Vec<TargetApplier>,
	pub allowed_ally_positions: PositionMatrix,
	pub ally_requirement: AllyRequirement,
	pub multi_target: bool,
	pub use_counter: UseCounter,
}

impl DefensiveSkill {
	pub fn calc_crit_chance(&self, caster: &CombatCharacter) -> Option<isize> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit_chance: crit } => { crit }
			CRITMode::NeverCrit => { return None; }
		};

		return Some(crit + caster.stat(ModifiableStat::CRIT));
	}
}