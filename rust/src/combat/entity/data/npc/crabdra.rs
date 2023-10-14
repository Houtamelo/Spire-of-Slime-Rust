use lazy_static::lazy_static;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::skill_types::*;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrabdraSkillName {
	Crush,
	Glare
}

pub static skill_crabdra_crush: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromCrabdra(CrabdraSkillName::Crush),
	recovery_ms: 0,
	charge_ms: 1500,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 85 },
	dmg     : DMGMode ::Power   { power: 100, toughness_reduction: 0 },
	crit    : CRITMode::CanCrit { crit_chance: 7 },
	effects_self: vec![],
	effects_target: vec![],
	caster_positions: PositionMatrix { positions: [true,  true, false, false] },
	target_positions: PositionMatrix { positions: [true, false, false, false] },
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

lazy_static! { pub static ref skill_crabdra_glare: Skill = Skill::Lewd(LewdSkill {
	skill_name: SkillName::FromCrabdra(CrabdraSkillName::Glare),
	recovery_ms: 0,
	charge_ms: 1700,
	acc_mode: ACCMode::NeverMiss,
	dmg: DMGMode::NoDamage,
	crit: CRITMode::NeverCrit,
	effects_self: vec![],
	effects_target: vec![TargetApplier::Lust { min: 5, max: 9 }, TargetApplier::Tempt { intensity: 100 }],
	caster_positions: PositionMatrix { positions: [true, true,  true,  true] },
	target_positions: PositionMatrix { positions: [true, true, false, false] },
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}