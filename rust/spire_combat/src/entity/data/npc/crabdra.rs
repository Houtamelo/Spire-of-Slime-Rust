#[allow(unused_imports)]
use crate::prelude::*;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};
use proc_macros::positions;
use crate::effects::*;
use crate::skill_types::*;
use crate::skill_types::offensive::*;

#[repr(usize)]
#[derive(Serialize, Deserialize)]
#[derive(FromVariant, ToVariant)]
#[derive(VariantNames, FromRepr, EnumString, EnumCount)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub enum CrabdraSkill {
	Crush,
	Glare
}

pub static CRUSH: Skill = Skill::Offensive(OffensiveSkill {
	skill_name: SkillVariant::Crabdra(CrabdraSkill::Crush),
	recovery_ms: SaturatedU64::new(0),
	charge_ms: SaturatedU64::new(1500),
	can_be_riposted: true,
	acc_mode: ACCMode ::CanMiss { acc: Accuracy::new(85) },
	dmg_mode: DMGMode ::Power   { power: Power::new(100), toughness_reduction: Bound_u8::new(0) },
	crit_mode: CRITMode::CanCrit { chance: CritRate::new(7) },
	custom_modifiers: Cow::Borrowed(&[]),
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
	skill_name: SkillVariant::Crabdra(CrabdraSkill::Glare),
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