#[allow(unused_imports)]
use crate::*;
use crate::combat::graphics::action_animation::skills::anim_utils::node_show;
use crate::combat::shared::*;

pub mod character_node;
pub mod default_position;

#[enum_delegate::implement_for{
	CharacterName,
	enum CharacterName {
		Girl(GirlName),
		NPC(NPCName),
	}
}]
pub trait EntityAnim {
	fn prefab_path(&self) -> &'static str;
	fn required_height(&self) -> f64;
	fn required_width(&self) -> f64;
	fn position_size(&self) -> Bound_u8<1, { u8::MAX }>;
	
	fn to_idle_anim(&self, character: CharacterNode) {
		character.node().touch_assert_sane(|node| {
			node_show(node, "anims/idle");
		});
	}
}

impl EntityAnim for NPCName {
	fn prefab_path(&self) -> &'static str {
		match self {
			NPCName::Crabdra => "res://Core/Combat/Characters/Crabdra/crabdra.tscn",
			NPCName::Trent => "res://Core/Combat/Characters/Trent/trent.tscn",
			NPCName::Wolfhydra => "res://Core/Combat/Characters/Wolfhydra/wolfhydra.tscn",
			NPCName::BellPlant => "res://Core/Combat/Characters/BellPlant/bell-plant.tscn",
		}
	}
	
	fn required_height(&self) -> f64 {
		match self {
			NPCName::Crabdra => 360.,
			NPCName::Trent => 360.,
			NPCName::Wolfhydra => 360.,
			NPCName::BellPlant => 360.,
		}
	}
	
	fn required_width(&self) -> f64 {
		match self {
			NPCName::Crabdra => 360.,
			NPCName::Trent => 360.,
			NPCName::Wolfhydra => 360.,
			NPCName::BellPlant => 360.,
		}
	}

	fn position_size(&self) -> Bound_u8<1, { u8::MAX }> {
		match self {
			| NPCName::Crabdra
			| NPCName::Trent
			| NPCName::BellPlant => 1.into(),
			| NPCName::Wolfhydra => 2.into(),
		}
	}
}

impl EntityAnim for GirlName {
	fn prefab_path(&self) -> &'static str {
		match self {
			GirlName::Ethel => "res://Core/Combat/Characters/Ethel/ethel.tscn",
			GirlName::Nema => "res://Core/Combat/Characters/Nema/nema.tscn",
		}
	}
	
	fn required_height(&self) -> f64 {
		match self {
			GirlName::Ethel => 360.,
			GirlName::Nema => 360.,
		}
	}
	
	fn required_width(&self) -> f64 {
		match self {
			GirlName::Ethel => 360.,
			GirlName::Nema => 360.,
		}
	}

	fn position_size(&self) -> Bound_u8<1, { u8::MAX }> {
		match self {
			| GirlName::Ethel
			| GirlName::Nema => 1.into(),
		}
	}
}