use gdnative::api::*;
use gdnative::derive::NativeClass;
use gdnative::prelude::*;
use uuid::Uuid;

#[derive(NativeClass, Debug)]
#[no_constructor]
#[inherit(Reference)]
pub struct CharacterNode {
	owner: Ref<Node2D>,
	guid: Uuid,
}

impl CharacterNode {
	pub fn owner(&self) -> Ref<Node2D> { self.owner }
	pub fn guid(&self) -> Uuid { self.guid }
	pub fn sprite_height(&self) -> f64 { todo!() }
}