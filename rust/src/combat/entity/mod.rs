pub mod girl;
pub mod position;
pub mod character;
pub mod skill_intention;
pub mod data;
pub mod stat;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use position::Position;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::entity::data::EntityData;
use crate::combat::entity::girl::DefeatedGirl_Entity;


#[derive(Debug, Clone, Serialize, Deserialize)]
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
	
	pub fn guid(&self) -> Uuid {
		return match self {
			Entity::Character(character) => character.guid,
			Entity::Corpse(corpse) => corpse.guid, 
			Entity::DefeatedGirl(defeated_girl) => defeated_girl.guid,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corpse {
	pub guid: Uuid,
	pub position: Position,
	pub data: EntityData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
	Human,
	Plant,
	Mutation,
}

macro_rules! iter_allies_of {
	($character: expr, $entities: expr) => {
		$entities.values().filter(|entity| $crate::combat::Position::is_same_side(entity.position(), $character.position()))
	};
}

macro_rules! iter_mut_allies_of {
	($character: expr, $entities: expr) => {
		$entities.values_mut().filter(|entity| $crate::combat::Position::is_same_side(entity.position(), $character.position()))
	};
}

macro_rules! iter_enemies_of {
	($character: expr, $entities: expr) => {
		$entities.values().filter(|entity| $crate::combat::Position::is_opposite_side(entity.position(), $character.position()))
	};
}

#[allow(unused_macros)]
macro_rules! iter_mut_enemies_of {
	($character: expr, $entities: expr) => {
		$entities.values_mut().filter(|entity| $crate::combat::Position::is_opposite_side(entity.position(), $character.position()))
	};
}

#[allow(unused_imports)]
pub(crate) use {iter_allies_of, iter_mut_allies_of, iter_enemies_of, iter_mut_enemies_of};
