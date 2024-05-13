#[allow(unused_imports)]
use crate::prelude::*;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};
use proc_macros::positions;
use crate::skill_types::AllyRequirement;
use crate::effects::*;

#[repr(usize)]
#[derive(FromVariant, ToVariant)]
#[derive(Serialize, Deserialize)]
#[derive(VariantNames, FromRepr, EnumString, EnumCount)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub enum EthelSkill {
	Safeguard,
	Clash,
	Jolt,
	Sever,
	Pierce,
	Challenge,
}

const SAFEGUARD_EFFECTS_SELF: &[SelfApplier; 1] = &[
	SelfApplier::Buff {
		duration_ms: SaturatedU64::new(5000),
		stat: DynamicStat::Dodge,
		stat_increase: NonZeroU16::new(15).unwrap(),
	}
];
const SAFEGUARD_EFFECTS_TARGET: &[TargetApplier; 1] = &[
	TargetApplier::MakeSelfGuardTarget {
		duration_ms: SaturatedU64::new(5000)
	}
];
pub static SAFEGUARD: Skill = SAFEGUARD_CONST;
pub const SAFEGUARD_CONST: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Safeguard),
	recovery_ms: SaturatedU64::new(1000),
	charge_ms  : SaturatedU64::new(0),
	crit_mode: CRITMode::NeverCrit,
	effects_self  : DynamicArray::Static(SAFEGUARD_EFFECTS_SELF),
	effects_target: DynamicArray::Static(SAFEGUARD_EFFECTS_TARGET),
	caster_positions: positions!("✔️|✔️|✔️|✔️"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	ally_requirement: AllyRequirement::NotSelf,
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

pub static CLASH: Skill = CLASH_CONST;
pub const CLASH_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Clash),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode : ACCMode ::CanMiss { acc: Accuracy::new(95) },
	dmg_mode : DMGMode ::Power   { power: Power::new(100), toughness_reduction: Bound_u8::new(5) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(9) },
	custom_modifiers: Cow::Borrowed(&[]),
	effects_self  : DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(&[]),
	caster_positions: positions!("✔️|✔️|🛑|🛑"),
	target_positions: positions!("✔️|✔️|🛑|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const JOLT_EFFECTS_SELF: &[SelfApplier; 1] = &[
	SelfApplier::Move {
		direction: MoveDirection::ToCenter(NonZeroI8::new(1).unwrap())
	}
];
const JOLT_EFFECTS_TARGET: &[TargetApplier; 2] = &[
	TargetApplier::Move {
		apply_chance: Some(NonZeroU16::new(100).unwrap()),
		direction: MoveDirection::ToEdge(NonZeroI8::new(1).unwrap())
	},
	TargetApplier::Stun {
		force: NonZeroU16::new(100).unwrap()
	}
];
pub static JOLT: Skill = JOLT_CONST;
pub const JOLT_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Jolt),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode : ACCMode ::CanMiss { acc: Accuracy::new(95) },
	dmg_mode : DMGMode ::Power   { power: Power::new(50), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(5) },
	custom_modifiers: Cow::Borrowed(&[]),
	effects_self  : DynamicArray::Static(JOLT_EFFECTS_SELF),
	effects_target: DynamicArray::Static(JOLT_EFFECTS_TARGET),
	caster_positions: positions!("✔️|✔️|🛑|🛑"),
	target_positions: positions!("✔️|🛑|🛑|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

pub static SEVER: Skill = SEVER_CONST;
pub const SEVER_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Sever),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode : ACCMode ::CanMiss { acc: Accuracy::new(90) },
	dmg_mode : DMGMode ::Power   { power: Power::new(60), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(0) },
	custom_modifiers: Cow::Borrowed(&[]),
	effects_self  : DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(&[]),
	caster_positions: positions!("✔️|🛑|🛑|🛑"),
	target_positions: positions!("✔️|✔️|🛑|🛑"),
	multi_target: true,
	use_counter: UseCounter::Unlimited,
});

const CHALLENGE_EFFECTS_SELF: &[SelfApplier; 1] = &[
	SelfApplier::Riposte {
		duration_ms: SaturatedU64::new(4000),
		acc_mode: ACCMode::CanMiss { acc: Accuracy::new(75) },
		crit_mode: CRITMode::CanCrit { chance: CritRate::new(-5) },
		skill_power: NonZeroU16::new(65).unwrap()
	}
];
const CHALLENGE_EFFECTS_TARGET: &[TargetApplier; 1] = &[
	TargetApplier::Mark {
		duration_ms: SaturatedU64::new(5000)
	}
];
pub static CHALLENGE: Skill = CHALLENGE_CONST;
pub const CHALLENGE_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Challenge),
	recovery_ms: SaturatedU64::new(1750),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: false,
	acc_mode : ACCMode ::NeverMiss,
	dmg_mode : DMGMode ::NoDamage,
	crit_mode: CRITMode::NeverCrit,
	custom_modifiers: Cow::Borrowed(&[]),
	effects_self  : DynamicArray::Static(CHALLENGE_EFFECTS_SELF),
	effects_target: DynamicArray::Static(CHALLENGE_EFFECTS_TARGET),
	caster_positions: positions!("✔️|🛑|🛑|🛑"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const PIERCE_CUSTOM_MODIFIERS: &[CustomOffensiveModifier; 1] = &[
	CustomOffensiveModifier::BonusVsMarked {
		power: 50,
		acc: 10,
		crit: 0
	}
];

pub static PIERCE: Skill = PIERCE_CONST;
pub const PIERCE_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Ethel(EthelSkill::Pierce),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode: ACCMode::CanMiss { acc: Accuracy::new(100) },
	dmg_mode: DMGMode::Power { power: Power::new(80), toughness_reduction: Bound_u8::new(15) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(13) },
	custom_modifiers: Cow::Borrowed(PIERCE_CUSTOM_MODIFIERS),
	effects_self: DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(&[]),
	caster_positions: positions!("✔️|🛑|🛑|🛑"),
	target_positions: positions!("✔️|✔️|✔️|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});