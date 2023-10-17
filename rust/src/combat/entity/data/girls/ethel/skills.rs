use lazy_static::lazy_static;
use proc_macros::positions;
use crate::combat::effects::MoveDirection;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::ModifiableStat;
use crate::combat::skill_types::*;
use crate::combat::skill_types::defensive::*;
use crate::combat::skill_types::offensive::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EthelSkillName {
	Safeguard,
	Clash,
	Jolt,
	Sever,
	Pierce,
	Challenge,
}

lazy_static! { pub static ref skill_ethel_safeguard: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Safeguard),
	recovery_ms: 1000,
	charge_ms  : 0,
	crit: CRITMode::NeverCrit,
	effects_self  : vec![SelfApplier::Buff { duration_ms: 5000, stat: ModifiableStat::DODGE, stat_increase: 15 }],
	effects_target: vec![TargetApplier::MakeSelfGuardTarget { duration_ms: 5000 }],
	caster_positions: positions!("✔️|✔️|✔️|✔️"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	ally_requirement: AllyRequirement::NotSelf,
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}

pub static skill_ethel_clash: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Clash),
	recovery_ms: 1500,
	charge_ms: 0,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 95 },
	dmg     : DMGMode ::Power   { power: 100, toughness_reduction: 5 },
	crit    : CRITMode::CanCrit { crit_chance: 9 },
	custom_modifiers: vec![],
	effects_self  : vec![],
	effects_target: vec![],
	caster_positions: positions!("✔️|✔️|❌|❌"),
	target_positions: positions!("✔️|✔️|❌|❌"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

lazy_static! { pub static ref skill_ethel_jolt: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Jolt),
	recovery_ms: 1500,
	charge_ms: 0,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 95 },
	dmg     : DMGMode ::Power   { power: 50, toughness_reduction: 0 },
	crit    : CRITMode::CanCrit { crit_chance: 5 },
	custom_modifiers: vec![],
	effects_self  : vec![SelfApplier  ::Move { direction: MoveDirection::ToCenter(1) }],
	effects_target: vec![TargetApplier::Move { apply_chance: Some(100), direction: MoveDirection::ToEdge(1) }, TargetApplier::Stun { force: 100 }],
	caster_positions: positions!("✔️|✔️|❌|❌"),
	target_positions: positions!("✔️|❌|❌|❌"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}

pub static skill_ethel_sever: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Sever),
	recovery_ms: 1500,
	charge_ms: 0,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 90 },
	dmg     : DMGMode ::Power   { power: 60, toughness_reduction: 0 },
	crit    : CRITMode::CanCrit { crit_chance: 0 },
	custom_modifiers: vec![],
	effects_self  : vec![],
	effects_target: vec![],
	caster_positions: positions!("✔️|❌|❌|❌"),
	target_positions: positions!("✔️|✔️|❌|❌"),
	multi_target: true,
	use_counter: UseCounter::Unlimited,
});

lazy_static! { pub static ref skill_ethel_pierce: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Pierce),
	recovery_ms: 1500,
	charge_ms: 0,
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: 100 },
	dmg     : DMGMode ::Power   { power: 80, toughness_reduction: 15 },
	crit    : CRITMode::CanCrit { crit_chance: 13 },
	custom_modifiers: vec![CustomOffensiveModifier::BonusVsMarked { power: 50, acc: 10, crit: 0 }],
	effects_self  : vec![],
	effects_target: vec![],
	caster_positions: positions!("✔️|❌|❌|❌"),
	target_positions: positions!("✔️|✔️|✔️|❌"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}

lazy_static! { pub static ref skill_ethel_challenge: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromEthel(EthelSkillName::Challenge),
	recovery_ms: 1750,
	charge_ms: 0,
	can_be_riposted: false,
	acc_mode: ACCMode ::NeverMiss,
	dmg     : DMGMode ::NoDamage,
	crit    : CRITMode::NeverCrit,
	custom_modifiers: vec![],
	effects_self  : vec![SelfApplier::Riposte { duration_ms: 4000, acc: 75, crit: CRITMode::CanCrit { crit_chance: -5 }, dmg_multiplier: 65 }],
	effects_target: vec![TargetApplier::Mark { duration_ms: 5000 }],
	caster_positions: positions!("✔️|❌|❌|❌"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});}