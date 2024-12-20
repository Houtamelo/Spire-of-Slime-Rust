#[allow(unused_imports)]
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};
use proc_macros::positions;
use crate::effects::*;
use crate::skill_types::*;
use crate::skill_types::defensive::*;

#[repr(usize)]
#[derive(Serialize, Deserialize)]
#[derive(FromVariant, ToVariant)]
#[derive(VariantNames, FromRepr, EnumString, EnumCount)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
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
	skill_name: SkillVariant::BellPlant(BellPlantSkill::Engorge),
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
	skill_name: SkillVariant::BellPlant(BellPlantSkill::InvigoratingFluids),
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
