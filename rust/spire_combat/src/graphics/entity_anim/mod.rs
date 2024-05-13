#[allow(unused_imports)]
use crate::prelude::*;
use crate::graphics::action_animation::skills::anim_utils::node_show;

pub mod character_node;
pub mod default_position;

#[enum_delegate::delegate(for(CharacterVariant))]
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

impl EntityAnim for NPCVariant {
	fn prefab_path(&self) -> &'static str {
		match self {
			NPCVariant::Crabdra => "res://Core/Combat/Characters/Crabdra/crabdra.tscn",
			NPCVariant::Trent => "res://Core/Combat/Characters/Trent/trent.tscn",
			NPCVariant::Wolfhydra => "res://Core/Combat/Characters/Wolfhydra/wolfhydra.tscn",
			NPCVariant::BellPlant => "res://Core/Combat/Characters/BellPlant/bell-plant.tscn",
		}
	}
	
	fn required_height(&self) -> f64 {
		match self {
			NPCVariant::Crabdra => 360.,
			NPCVariant::Trent => 360.,
			NPCVariant::Wolfhydra => 360.,
			NPCVariant::BellPlant => 360.,
		}
	}
	
	fn required_width(&self) -> f64 {
		match self {
			NPCVariant::Crabdra => 360.,
			NPCVariant::Trent => 360.,
			NPCVariant::Wolfhydra => 360.,
			NPCVariant::BellPlant => 360.,
		}
	}

	fn position_size(&self) -> Bound_u8<1, { u8::MAX }> {
		match self {
			| NPCVariant::Crabdra
			| NPCVariant::Trent
			| NPCVariant::BellPlant => 1.into(),
			| NPCVariant::Wolfhydra => 2.into(),
		}
	}
}

impl EntityAnim for GirlVariant {
	fn prefab_path(&self) -> &'static str {
		match self {
			GirlVariant::Ethel => "res://Core/Combat/Characters/Ethel/ethel.tscn",
			GirlVariant::Nema => "res://Core/Combat/Characters/Nema/nema.tscn",
		}
	}
	
	fn required_height(&self) -> f64 {
		match self {
			GirlVariant::Ethel => 360.,
			GirlVariant::Nema => 360.,
		}
	}
	
	fn required_width(&self) -> f64 {
		match self {
			GirlVariant::Ethel => 360.,
			GirlVariant::Nema => 360.,
		}
	}

	fn position_size(&self) -> Bound_u8<1, { u8::MAX }> {
		match self {
			| GirlVariant::Ethel
			| GirlVariant::Nema => 1.into(),
		}
	}
}