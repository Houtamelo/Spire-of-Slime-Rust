use crate::combat::skills::{DefensiveSkill, LewdSkill, OffensiveSkill};

#[derive(Debug, Clone)]
pub struct SkillIntention {
	pub skill: SkillAndTarget,
	pub charge_ticks: TrackedTicks,
	pub recovery_after_complete: Option<TrackedTicks>,
}

impl PartialEq for SkillIntention {
	fn eq(&self, other: &Self) -> bool {
		return self.skill == other.skill
				&& self.charge_ticks == other.charge_ticks
				&& self.recovery_after_complete == other.recovery_after_complete;
	}
}

impl Eq for SkillIntention {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillAndTarget {
	OnSelf(DefensiveSkill),
	OnAlly  { skill: DefensiveSkill, ally_guid: usize },
	OnEnemy { skill: OffensiveSkill, position : usize },
	Lewd    { skill: LewdSkill     , position : usize },
}