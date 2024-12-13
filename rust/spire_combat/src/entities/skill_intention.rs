use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct SkillIntention {
	pub skill: SkillTarget,
	pub charge_ticks: TrackedTicks,
	pub recovery_after_complete: Option<TrackedTicks>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SkillTarget {
	OnSelf(DefensiveSkill),
	OnAlly {
		skill: DefensiveSkill,
		ally:  Id,
	},
	OnEnemy {
		skill: OffensiveSkill,
		position: usize,
	},
	Lewd {
		skill: LewdSkill,
		position: usize,
	},
}
