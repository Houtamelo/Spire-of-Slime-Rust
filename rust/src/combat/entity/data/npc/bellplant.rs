use lazy_static::lazy_static;
use proc_macros::positions;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::skill_types::*;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::defensive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BellPlantSkillName {
	Engorge,
	InvigoratingFluids,
}

lazy_static! { pub static ref skill_bellplant_engorge: Skill = Skill::Lewd(LewdSkill {
	skill_name: SkillName::FromBellPlant(BellPlantSkillName::Engorge),
	recovery_ms: 0,
	charge_ms: 2000,
	acc_mode: ACCMode::NeverMiss,
	dmg: DMGMode::NoDamage,
	crit: CRITMode::NeverCrit,
	effects_self: vec![],
	effects_target: vec![TargetApplier::Lust { min: 6, max: 10 }, TargetApplier::Tempt { intensity: 80 }],
	caster_positions: positions!("✔️|❌|❌|❌"),
	target_positions: positions!("✔️|❌|❌|❌"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}

lazy_static! { pub static ref skill_bellplant_invigoratingfluids: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillName::FromBellPlant(BellPlantSkillName::InvigoratingFluids),
	recovery_ms: 0,
	charge_ms: 2000,
	crit: CRITMode::CanCrit { crit_chance: 5 },
	effects_self: vec![],
	effects_target: vec![TargetApplier::PersistentHeal { duration_ms: 4000, heal_per_sec: 1 }],
	caster_positions: positions!("❌|✔️|✔️|✔️"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	ally_requirement: AllyRequirement::CanSelf,
	multi_target: true,
	use_counter: UseCounter::Limited { max_uses: 2 },
});}

#[derive(Debug, Clone)]
pub struct LurePerk { }
