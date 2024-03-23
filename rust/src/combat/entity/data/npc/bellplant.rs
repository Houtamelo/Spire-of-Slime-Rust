use std::num::{NonZeroU16, NonZeroU8};

use comfy_bounded_ints::prelude::Bound_u8;
use serde::{Deserialize, Serialize};
use util::prelude::DynamicArray;

use proc_macros::positions;

use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::stat::{CheckedRange, CritRate};
use crate::combat::skill_types::*;
use crate::combat::skill_types::defensive::*;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::misc::SaturatedU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BellPlantSkill {
	Engorge,
	InvigoratingFluids,
}

const ENGORGE_EFFECTS_TARGET: &[TargetApplier; 2] = &[
	TargetApplier::Lust {
		delta: CheckedRange::new(6, 10).unwrap()
	},
	TargetApplier::Tempt {
		intensity: NonZeroU16::new(80).unwrap()
	}
];

pub static ENGORGE: Skill = Skill::Lewd(LewdSkill {
	skill_name: SkillName::FromBellPlant(BellPlantSkill::Engorge),
	recovery_ms: SaturatedU64::new(0),
	charge_ms: SaturatedU64::new(2000),
	acc_mode: ACCMode::NeverMiss,
	dmg_mode: DMGMode::NoDamage,
	crit_mode: CRITMode::NeverCrit,
	effects_self: DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(ENGORGE_EFFECTS_TARGET),
	caster_positions: positions!("✔️|🛑|🛑|🛑"),
	target_positions: positions!("✔️|🛑|🛑|🛑"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const INVIGORATING_FLUIDS_EFFECTS_TARGET: &[TargetApplier; 1] = &[
	TargetApplier::PersistentHeal {
		duration_ms: SaturatedU64::new(4000),
		heal_per_interval: NonZeroU8::new(1).unwrap()
	}
];
pub static INVIGORATING_FLUIDS: Skill = Skill::Defensive(DefensiveSkill {
	skill_name: SkillName::FromBellPlant(BellPlantSkill::InvigoratingFluids),
	recovery_ms: SaturatedU64::new(0),
	charge_ms: SaturatedU64::new(2000),
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(5) },
	effects_self: DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(INVIGORATING_FLUIDS_EFFECTS_TARGET),
	caster_positions: positions!("🛑|✔️|✔️|✔️"),
	target_positions: positions!("✔️|✔️|✔️|✔️"),
	ally_requirement: AllyRequirement::CanSelf,
	multi_target: true,
	use_counter: UseCounter::Limited { max_uses: Bound_u8::new(2) },
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LurePerk { }
