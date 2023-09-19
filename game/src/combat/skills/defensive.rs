#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefensiveSkill {
	recovery_ms: i64,
	charge_ms: i64,
	crit: CRITMode,
	effects_self: Vec<SelfApplier>,
	effects_target: Vec<TargetApplier>,
	target_positions: PositionSetup,
	multi_target: bool,
	use_counter: UseCounter,
}

impl DefensiveSkill {
	pub fn calc_crit_chance(&self, caster: &CombatCharacter, target: &CombatCharacter) -> Option<isize> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit } => { crit }
			CRITMode::NeverCrit => { return None; }
		};

		return Some(crit + caster.stat(ModifiableStat::CRIT));
	}
}