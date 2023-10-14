use gdnative::godot_error;
use entity::position::Position;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::entity;
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;
use crate::MAX_CHARACTERS_PER_TEAM;

pub mod offensive;
pub mod defensive;
pub mod lewd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Skill {
	Offensive(OffensiveSkill),
	Defensive(DefensiveSkill),
	Lewd(LewdSkill),
}

pub trait SkillTrait {
	fn name            (&self) -> SkillName;
	fn recovery_ms     (&self) -> &i64;
	fn charge_ms       (&self) -> &i64;
	fn crit            (&self) -> &CRITMode;
	fn effects_self    (&self) -> &Vec<SelfApplier>;
	fn effects_target  (&self) -> &Vec<TargetApplier>;
	fn caster_positions(&self) -> &PositionMatrix;
	fn target_positions(&self) -> &PositionMatrix;
	fn multi_target    (&self) -> &bool;
	fn use_counter     (&self) -> &UseCounter;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ACCMode {
	CanMiss { acc: isize },
	NeverMiss,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum DMGMode {
	Power { power: isize, toughness_reduction: isize },
	NoDamage,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CRITMode {
	CanCrit { crit_chance: isize },
	NeverCrit,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct PositionMatrix {
	pub positions: [bool; MAX_CHARACTERS_PER_TEAM],
}

impl PositionMatrix {
	pub fn contains(&self, position: Position) -> bool {
		let (order, size): (usize, usize) = position.deconstruct();
		
		if (order + size > MAX_CHARACTERS_PER_TEAM) || (size == 0) {
			godot_error!("PositionMatrix::contains: position: {position:?}, size: {size} is out of bounds");
			return false;
		}

		for index in order..(order + size) {
			if self.positions[index] {
				return true;
			}
		}
		
		return false;
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum AllyRequirement {
	CanSelf,
	NotSelf,
	OnlySelf
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: isize },
}