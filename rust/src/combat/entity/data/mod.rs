#[allow(unused_imports)]
use crate::*;
pub mod character;
pub mod npc;
pub mod girls;
pub mod skill_name;

use serde::{Deserialize, Serialize};
use crate::combat::entity::data::character::CharacterData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityData {
	Character(CharacterData),
}