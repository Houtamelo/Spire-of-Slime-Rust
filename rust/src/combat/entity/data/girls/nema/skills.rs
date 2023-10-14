use lazy_static::lazy_static;
use proc_macros::positions;
use crate::combat::effects::MoveDirection::ToEdge;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::{ACCMode, AllyRequirement, CRITMode, DMGMode, Skill, UseCounter};
use crate::combat::skill_types::offensive::OffensiveSkill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NemaSkillName {
	Calm,
	Gawky
}

lazy_static! { pub static ref skill_nema_calm: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillName::FromNema(NemaSkillName::Calm),
	recovery_ms: 1500,
	charge_ms: 0,
	crit: CRITMode::CanCrit { crit_chance: 5 },
	effects_self  : vec![SelfApplier::ChangeExhaustion { delta: 1 }],
	effects_target: vec![TargetApplier::Heal { base_multiplier: 100 }],
	caster_positions: positions!["✔️|✔️|✔️|✔️"],
	target_positions: positions!["✔️|✔️|✔️|✔️"],
	ally_requirement: AllyRequirement::CanSelf,
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}

lazy_static! { pub static ref skill_nema_gawky: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromNema(NemaSkillName::Gawky),
	recovery_ms: 1000,
	charge_ms: 0,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 90 },
	dmg     : DMGMode ::Power   { power: 25, toughness_reduction: 0 },
	crit    : CRITMode::CanCrit { crit_chance: 15 },
	effects_self  : vec![SelfApplier::Move { direction: ToEdge(1) }],
	effects_target: vec![TargetApplier::Stun { force: 40 }],
	caster_positions: positions!("✔️|✔️|❌|❌"),
	target_positions: positions!("✔️|✔️|❌|❌"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}