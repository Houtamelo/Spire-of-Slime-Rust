use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;
use crate::util::TrackedTicks;

#[derive(Debug, Clone)]
pub struct SkillIntention {
	pub skill: SkillAndTarget,
	pub charge_ticks: TrackedTicks,
	pub recovery_after_complete: Option<TrackedTicks>,
}

#[derive(Debug, Clone)]
pub enum SkillAndTarget {
	OnSelf(DefensiveSkill),
	OnAlly  { skill: DefensiveSkill, ally_guid: usize },
	OnEnemy { skill: OffensiveSkill, position : usize },
	Lewd    { skill: LewdSkill     , position : usize },
}