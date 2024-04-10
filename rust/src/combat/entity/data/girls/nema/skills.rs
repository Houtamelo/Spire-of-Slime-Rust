#[allow(unused_imports)]
use crate::*;

use std::num::{NonZeroI8, NonZeroU16};

use proc_macros::positions;

use crate::combat::shared::*;
use crate::combat::effects::MoveDirection;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::character::SkillUser;
use crate::combat::entity::data::girls::nema::stats::NemaData;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::stat::{Accuracy, CritRate, Power};
use crate::combat::skill_types::{ACCMode, AllyRequirement, CRITMode, DMGMode, Skill, UseCounter};
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NemaSkill {
	Calm,
	Gawky
}

impl SkillUser for NemaData { 
	fn skills(&self) -> &[Skill] { 
		return &self.skills;
	}
}

const CALM_EFFECTS_SELF: &[SelfApplier; 1] = &[
	SelfApplier::ChangeExhaustion { 
		delta: NonZeroI8::new(1).unwrap() 
	}
];
const CALM_EFFECTS_TARGET: &[TargetApplier; 1] = &[
	TargetApplier::Heal { 
		multiplier: NonZeroU16::new(100).unwrap() 
	}
];
pub static CALM: Skill = CALM_CONST;
pub const CALM_CONST: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillName::FromNema(NemaSkill::Calm),
	recovery_ms: SaturatedU64::new(1500),
	charge_ms: SaturatedU64::new(0),
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(5) },
	effects_self: DynamicArray::Static(CALM_EFFECTS_SELF),
	effects_target: DynamicArray::Static(CALM_EFFECTS_TARGET),
	caster_positions: positions!["✔️|✔️|✔️|✔️"],
	target_positions: positions!["✔️|✔️|✔️|✔️"],
	ally_requirement: AllyRequirement::CanSelf,
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const GAWKY_EFFECTS_SELF: &[SelfApplier; 1] = &[
	SelfApplier::Move {
		direction: MoveDirection::ToEdge(NonZeroI8::new(1).unwrap())
	}
];
const GAWKY_EFFECTS_TARGET: &[TargetApplier; 1] = &[
	TargetApplier::Stun {
		force: NonZeroU16::new(40).unwrap()
	}
];
pub static GAWKY: Skill = GAWKY_CONST;
pub const GAWKY_CONST: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromNema(NemaSkill::Gawky),
	recovery_ms: SaturatedU64::new(1000),
	charge_ms: SaturatedU64::new(0),
	can_be_riposted: true,
	acc_mode : ACCMode ::CanMiss { acc: Accuracy::new(90) },
	dmg_mode : DMGMode ::Power   { power: Power::new(25), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(15) },
	custom_modifiers: DynamicArray::Static(&[]),
	effects_self  : DynamicArray::Static(GAWKY_EFFECTS_SELF),
	effects_target: DynamicArray::Static(GAWKY_EFFECTS_TARGET),
	caster_positions: positions!("✔️|✔️|🛑|🛑"),
	target_positions: positions!("✔️|✔️|🛑|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});