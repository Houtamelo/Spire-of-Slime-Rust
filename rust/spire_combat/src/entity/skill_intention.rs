#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillIntention {
	pub skill: SkillTarget,
	pub charge_ticks: TrackedTicks,
	pub recovery_after_complete: Option<TrackedTicks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTarget {
	OnSelf(DefensiveSkill),
	OnAlly  { skill: DefensiveSkill, ally_guid: Uuid },
	OnEnemy { skill: OffensiveSkill, position: usize },
	Lewd    { skill: LewdSkill     , position: usize },
}