use crate::combat::entity;
include!("offensive.rs");
include!("defensive.rs");
include!("lewd.rs");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
	key: Box<String>,
	recovery_ms: i64,
	charge_ms: i64,
	skill_type: SkillType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillType {
	Offensive(OffensiveSkill),
	Defensive(DefensiveSkill),
	Lewd(LewdSkill),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ACCMode {
	CanMiss { acc: isize },
	NeverMiss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DMGMode {
	Power { power: isize, toughness_reduction: isize },
	NoDamage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CRITMode {
	CanCrit { crit: isize },
	NeverCrit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionMatrix {
	pub indexed_positions: [bool; MAX_CHARACTERS_PER_TEAM],
}

impl PositionMatrix {
	pub fn contains(&self, position: entity::Position) -> bool {
		let (order, size) = position.deconstruct();
		
		if (order + size - 1 >= MAX_CHARACTERS_PER_TEAM) || (order < 0) || (size == 0) {
			godot_error!("PositionMatrix::contains: position: {position}, size: {size} is out of bounds");
			return false;
		}

		for index in order..(order + size) {
			if self.indexed_positions[index] {
				return true;
			}
		}
		
		return false;
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllyRequirement {
	CanSelf,
	NotSelf,
	OnlySelf
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: isize },
}