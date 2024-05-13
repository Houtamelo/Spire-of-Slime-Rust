#[allow(unused_imports)]
use crate::prelude::*;
use proc_macros::positions;
use crate::effects;
use crate::skill_types;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};

#[repr(usize)]
#[derive(Serialize, Deserialize)]
#[derive(FromVariant, ToVariant)]
#[derive(FromRepr, EnumString, EnumCount, VariantNames)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub enum NemaSkill {
	Calm,
	Gawky
}

const CALM_EFFECTS_SELF: &[effects::SelfApplier; 1] = &[
	effects::SelfApplier::ChangeExhaustion { 
		delta: NonZeroI8::new(1).unwrap() 
	}
];
const CALM_EFFECTS_TARGET: &[effects::TargetApplier; 1] = &[
	effects::TargetApplier::Heal { 
		multiplier: NonZeroU16::new(100).unwrap() 
	}
];
pub static CALM: Skill = CALM_CONST;
pub const CALM_CONST: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillVariant::Nema(NemaSkill::Calm),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(5) },
	effects_self: DynamicArray::Static(CALM_EFFECTS_SELF),
	effects_target: DynamicArray::Static(CALM_EFFECTS_TARGET),
	caster_positions: positions!["✔️|✔️|✔️|✔️"],
	target_positions: positions!["✔️|✔️|✔️|✔️"],
	ally_requirement: skill_types::AllyRequirement::CanSelf,
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const GAWKY_EFFECTS_SELF: &[effects::SelfApplier; 1] = &[
	effects::SelfApplier::Move {
		direction: effects::MoveDirection::ToEdge(NonZeroI8::new(1).unwrap())
	}
];
const GAWKY_EFFECTS_TARGET: &[effects::TargetApplier; 1] = &[
	effects::TargetApplier::Stun {
		force: NonZeroU16::new(40).unwrap()
	}
];

pub static GAWKY: Skill = GAWKY_CONST;
pub const GAWKY_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Nema(NemaSkill::Gawky),
	recovery_ms: SaturatedU64::new(1000),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode : ACCMode ::CanMiss { acc: Accuracy::new(90) },
	dmg_mode : DMGMode ::Power   { power: Power::new(25), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(15) },
	custom_modifiers: Cow::Borrowed(&[]),
	effects_self  : DynamicArray::Static(GAWKY_EFFECTS_SELF),
	effects_target: DynamicArray::Static(GAWKY_EFFECTS_TARGET),
	caster_positions: positions!("✔️|✔️|🛑|🛑"),
	target_positions: positions!("✔️|✔️|🛑|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});