use serde::{Deserialize, Serialize};

use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;
use crate::misc::TrackedTicks;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillIntention {
	pub skill: SkillTarget,
	pub charge_ticks: TrackedTicks,
	pub recovery_after_complete: Option<TrackedTicks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillTarget {
	OnSelf(DefensiveSkill),
	OnAlly  { skill: DefensiveSkill, ally_guid: usize },
	OnEnemy { skill: OffensiveSkill, position : usize },
	Lewd    { skill: LewdSkill     , position : usize },
}