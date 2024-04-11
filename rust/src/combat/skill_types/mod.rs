#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;

pub mod offensive;
pub mod defensive;
pub mod lewd;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Skill {
	Offensive(OffensiveSkill),
	Defensive(DefensiveSkill),
	Lewd(LewdSkill),
}

pub trait SkillTrait {
	fn name            (&self) -> SkillName;
	fn recovery_ms     (&self) -> &SaturatedU64;
	fn charge_ms       (&self) -> &SaturatedU64;
	fn crit            (&self) -> &CRITMode;
	fn effects_self    (&self) -> &[SelfApplier];
	fn effects_target  (&self) -> &[TargetApplier];
	fn caster_positions(&self) -> &PositionMatrix;
	fn target_positions(&self) -> &PositionMatrix;
	fn multi_target    (&self) -> &bool;
	fn use_counter     (&self) -> &UseCounter;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum ACCMode {
	CanMiss { acc: Accuracy },
	NeverMiss,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum DMGMode {
	Power { power: Power, toughness_reduction: Bound_u8<0, 100> },
	NoDamage,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum CRITMode {
	CanCrit { chance: CritRate },
	NeverCrit,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub struct PositionMatrix {
	pub positions: [bool; MAX_CHARACTERS_PER_TEAM as usize],
}

impl PositionMatrix {
	pub fn contains(&self, position: Position) -> bool {
		let order = position.order.squeeze_to_usize();
		let size = position.size.squeeze_to_usize();
		
		let plus_size = order + size;
		if plus_size > MAX_CHARACTERS_PER_TEAM as usize
		|| size == 0 {
			godot_error!("PositionMatrix::contains: position: {position:?}, size: {size} is out of bounds");
			return false;
		}

		return (order..plus_size).any(|index| self.positions[index]);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum AllyRequirement {
	CanSelf,
	NotSelf,
	OnlySelf
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: Bound_u8<1, 255> },
}