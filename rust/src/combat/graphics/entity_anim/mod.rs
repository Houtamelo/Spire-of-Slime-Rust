use enum_dispatch::enum_dispatch;
#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;

pub mod character_node;
pub mod default_position;

#[enum_dispatch]
pub trait EntityAnim {
	fn prefab_path(&self) -> &'static str;
	fn required_height(&self) -> f64;
	fn required_width(&self) -> f64;
}

impl EntityAnim for NPCName {
	fn prefab_path(&self) -> &'static str {
		match self {
			NPCName::Crabdra => "res://Core/Combat/Crabdra/crabdra.tscn",
			NPCName::Trent => "res://Core/Combat/Trent/trent.tscn",
			NPCName::Wolfhydra => "res://Core/Combat/Wolfhydra/wolfhydra.tscn",
			NPCName::BellPlant => "res://Core/Combat/BellPlant/bell-plant.tscn",
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
}

impl EntityAnim for GirlName {
	fn prefab_path(&self) -> &'static str {
		match self {
			GirlName::Ethel => "res://Core/Combat/Ethel/ethel.tscn",
			GirlName::Nema => "res://Core/Combat/Nema/nema.tscn",
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
}