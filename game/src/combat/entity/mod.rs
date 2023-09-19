use std::cmp::Ordering;
include!("skill_intention.rs");
include!("character.rs");
include!("position.rs");

#[derive(Debug)]
pub enum Entity {
	Character(CombatCharacter),
	Corpse(Position),
}

impl Entity {
	pub fn compare_position(&self, other: &Self) -> Ordering {
		return self.position().cmp(other.position());
	}
	
	pub fn position(&self) -> &Position {
		match self {
			Entity::Character(character) => &character.position,
			Entity::Corpse(position) => position,
		}
	}
}