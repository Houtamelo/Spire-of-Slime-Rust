#[derive(Debug, Clone)]
pub struct SkillIntention {
	skill: Skill,
	charge_progress: RemainingTicks,
	recovery_after_complete: Option<RemainingTicks>,
	chosen_target_position: PositionSetup,
}

impl PartialEq for SkillIntention {
	fn eq(&self, other: &Self) -> bool {
		return self.skill == other.skill
				&& self.charge_progress == other.charge_progress
				&& self.recovery_after_complete == other.recovery_after_complete
				&& self.chosen_target_position == other.chosen_target_position;
	}
}

impl Eq for SkillIntention {}