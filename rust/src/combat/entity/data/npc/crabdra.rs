#[allow(unused_imports)]
use crate::*;

use std::num::NonZeroU16;
use enum_variant_type::EnumVariantType;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};

use proc_macros::positions;

use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::stat::{Accuracy, CheckedRange, CritRate, Power};
use crate::combat::skill_types::*;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::*;

#[repr(usize)]
#[derive(EnumVariantType)]
#[evt(derive(Clone, Copy, Debug, PartialEq, Eq, Hash))]
#[derive(VariantNames)]
#[derive(FromRepr)]
#[derive(EnumString)]
#[derive(EnumCount)]
#[derive(FromVariant, ToVariant)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrabdraSkill {
	Crush,
	Glare
}

pub static CRUSH: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillName::FromCrabdra(CrabdraSkill::Crush),
	recovery_ms: SaturatedU64::new(0),
	charge_ms: SaturatedU64::new(1500),
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: Accuracy::new(85) },
	dmg_mode: DMGMode ::Power   { power: Power::new(100), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(7) },
	custom_modifiers: DynamicArray::Static(&[]),
	effects_self: DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(&[]),
	caster_positions: positions!("✔️|✔️|🛑|🛑|"),
	target_positions: positions!("✔️|🛑|🛑|🛑|"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});

const GLARE_EFFECTS_TARGET: &[TargetApplier; 2] = &[
	TargetApplier::Lust { 
		delta: CheckedRange::new(5, 9).unwrap()
	}, 
	TargetApplier::Tempt { 
		intensity: NonZeroU16::new(100).unwrap() 
	}
];
pub static GLARE: Skill = Skill::Lewd(LewdSkill {
	skill_name: SkillName::FromCrabdra(CrabdraSkill::Glare),
	recovery_ms: SaturatedU64::new(0),
	charge_ms: SaturatedU64::new(1700),
	acc_mode: ACCMode::NeverMiss,
	dmg_mode: DMGMode::NoDamage,
	crit_mode: CRITMode::NeverCrit,
	effects_self: DynamicArray::Static(&[]),
	effects_target: DynamicArray::Static(GLARE_EFFECTS_TARGET),
	caster_positions: positions!("✔️|✔️|✔️|✔️|"),
	target_positions: positions!("✔️|✔️|🛑|🛑|"),
	multi_target: false,
	use_counter: UseCounter::Unlimited,
});