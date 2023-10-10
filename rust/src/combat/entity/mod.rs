pub mod girl;
pub mod position;
pub mod character;
pub mod skill_intention;

use std::cmp::Ordering;
use position::Position;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::entity::girl::DefeatedGirl_Entity;
use crate::util::GUID;

#[derive(Debug)]
pub enum Entity {
	Character(CombatCharacter),
	Corpse(Corpse),
	DefeatedGirl(DefeatedGirl_Entity),
}

impl Entity {
	pub fn compare_position(&self, other: &Self) -> Ordering {
		return self.position().cmp(other.position());
	}
	
	pub fn position(&self) -> &Position {
		return match self {
			Entity::Character(character) => &character.position,
			Entity::Corpse(corpse) => &corpse.position,
			Entity::DefeatedGirl(defeated_girl) => &defeated_girl.position,
		}
	}
	
	pub fn position_mut(&mut self) -> &mut Position {
		return match self {
			Entity::Character(character) => &mut character.position,
			Entity::Corpse(corpse) => &mut corpse.position,
			Entity::DefeatedGirl(defeated_girl) => &mut defeated_girl.position,
		}
	}
	
	pub fn guid(&self) -> GUID {
		return match self {
			Entity::Character(character) => character.guid,
			Entity::Corpse(corpse) => corpse.guid, 
			Entity::DefeatedGirl(defeated_girl) => defeated_girl.guid,
		}
	}
}

#[derive(Debug)]
pub struct Corpse {
	pub(crate) guid: GUID,
	pub(crate) position: Position,
}