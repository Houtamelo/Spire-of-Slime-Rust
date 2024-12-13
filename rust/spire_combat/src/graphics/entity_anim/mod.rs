use super::*;

mod character_node;
mod default_position;

pub use character_node::*;
pub use default_position::*;

pub trait EntityAnim {
	fn prefab_path(&self) -> &'static str;
	fn required_height(&self) -> f64;
	fn required_width(&self) -> f64;
	fn position_size(&self) -> Int;

	fn to_idle_anim(&self, character: ActorNode) { node_show(&character.node(), "anims/idle"); }
}

impl EntityAnim for NpcName {
	fn prefab_path(&self) -> &'static str {
		match self {
			NpcName::Crabdra => "res://Core/Combat/Characters/Crabdra/crabdra.tscn",
			NpcName::Trent => "res://Core/Combat/Characters/Trent/trent.tscn",
			NpcName::Wolfhydra => "res://Core/Combat/Characters/Wolfhydra/wolfhydra.tscn",
			NpcName::BellPlant => "res://Core/Combat/Characters/BellPlant/bell-plant.tscn",
		}
	}

	fn required_height(&self) -> f64 {
		match self {
			NpcName::Crabdra => 360.,
			NpcName::Trent => 360.,
			NpcName::Wolfhydra => 360.,
			NpcName::BellPlant => 360.,
		}
	}

	fn required_width(&self) -> f64 {
		match self {
			NpcName::Crabdra => 360.,
			NpcName::Trent => 360.,
			NpcName::Wolfhydra => 360.,
			NpcName::BellPlant => 360.,
		}
	}

	fn position_size(&self) -> Int {
		match self {
			| NpcName::Crabdra | NpcName::Trent | NpcName::BellPlant => 1.into(),
			| NpcName::Wolfhydra => 2.into(),
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

	fn position_size(&self) -> Int {
		match self {
			| GirlName::Ethel | GirlName::Nema => 1.into(),
		}
	}
}
