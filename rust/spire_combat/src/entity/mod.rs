#[allow(unused_imports)]
use crate::prelude::*;

pub mod girl;
pub mod position;
pub mod character;
pub mod skill_intention;
pub mod data;
pub mod stat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Entity {
	Character(CombatCharacter),
	Corpse(Corpse),
	DefeatedGirl(DefeatedGirl_Entity),
}

impl Entity {
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
	
	pub fn sprite_height(&self) -> f64 {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corpse {
	pub guid: Uuid,
	pub position: Position,
	pub data: EntityDataVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Race {
	Human,
	Plant,
	Mutation,
}

macro_rules! iter_allies_of {
	($character: expr, $entities: expr) => {
		$entities.values().filter(|entity| entity.position().side == $character.position().side)
	};
}

macro_rules! iter_mut_allies_of {
	($character: expr, $entities: expr) => {
		$entities.values_mut().filter(|entity| entity.position().side == $character.position().side)
	};
}

macro_rules! iter_enemies_of {
	($character: expr, $entities: expr) => {
		$entities.values().filter(|entity| entity.position().side != $character.position().side)
	};
}

#[allow(unused_macros)]
macro_rules! iter_mut_enemies_of {
	($character: expr, $entities: expr) => {
		$entities.values_mut().filter(|entity| entity.position().side != $character.position().side)
	};
}

#[allow(unused_imports)]
pub(crate) use {iter_allies_of, iter_mut_allies_of, iter_enemies_of, iter_mut_enemies_of};
