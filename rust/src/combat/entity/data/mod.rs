use crate::combat::entity::data::character::CharacterData;

pub mod character;
pub mod npc;
pub mod girls;
pub mod skill_name;

#[derive(Debug)]
pub enum EntityData {
	Character(CharacterData),
}