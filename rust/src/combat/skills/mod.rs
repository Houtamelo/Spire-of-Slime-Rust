use gdnative::godot_error;
use entity::position::Position;
use crate::combat::entity;
use crate::combat::skills::defensive::DefensiveSkill;
use crate::combat::skills::lewd::LewdSkill;
use crate::combat::skills::offensive::OffensiveSkill;
use crate::MAX_CHARACTERS_PER_TEAM;

pub mod offensive;
pub mod defensive;
pub mod lewd;

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
	CanCrit { crit_chance: isize },
	NeverCrit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionMatrix {
	pub indexed_positions: [bool; MAX_CHARACTERS_PER_TEAM],
}

impl PositionMatrix {
	pub fn contains(&self, position: Position) -> bool {
		let (order, size) = position.deconstruct();
		
		if (order + size > MAX_CHARACTERS_PER_TEAM) || (order < 0) || (size == 0) {
			godot_error!("PositionMatrix::contains: position: {position:?}, size: {size} is out of bounds");
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