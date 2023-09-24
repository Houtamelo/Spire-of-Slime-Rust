use std::cmp::Ordering;
include!("skill_intention.rs");
include!("character.rs");
include!("position.rs");

#[derive(Debug)]
pub enum Entity {
	Character(CombatCharacter),
	Corpse(Position),
	DefeatedGirl(DefeatedGirl_Entity),
}

impl Entity {
	pub fn compare_position(&self, other: &Self) -> Ordering {
		return self.position().cmp(other.position());
	}
	
	pub fn position(&self) -> &Position {
		match self {
			Entity::Character(character) => &character.position,
			Entity::Corpse(position) => position,
			Entity::DefeatedGirl(defeated_girl) => &defeated_girl.position,
		}
	}
	
	pub fn position_mut(&mut self) -> &mut Position {
		match self {
			Entity::Character(character) => &mut character.position,
			Entity::Corpse(position) => position,
			Entity::DefeatedGirl(defeated_girl) => &mut defeated_girl.position,
		}
	}
}