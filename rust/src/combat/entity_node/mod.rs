#[allow(unused_imports)]
use crate::*;

use crate::combat::entity::data::character::CharacterName;

#[derive(NativeClass, Debug)]
#[no_constructor]
#[user_data(GoodCellData<CharacterNode>)]
#[inherit(Node2D)]
pub struct CharacterNode {
	owner: Ref<Node2D>,
	guid: Uuid,
	character: CharacterName,
}

impl CharacterNode {
	pub fn owner(&self) -> Ref<Node2D> { self.owner }
	pub fn guid(&self) -> Uuid { self.guid }
	pub fn character(&self) -> CharacterName { self.character }
	
	pub fn sprite_height(&self) -> f64 { todo!() }
	
}