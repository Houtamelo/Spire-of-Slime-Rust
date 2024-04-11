#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;

pub mod character_node;

pub trait EntityAnim {
	fn prefab_path(&self) -> &'static str;
}

impl EntityAnim for CharacterName {
	fn prefab_path(&self) -> &'static str {
		match self {
			CharacterName::Girl(girl) => girl.prefab_path(),
			CharacterName::NPC(npc) => npc.prefab_path(),
		}
	}
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
}

impl EntityAnim for GirlName {
	fn prefab_path(&self) -> &'static str {
		match self {
			GirlName::Ethel => "res://Core/Combat/Ethel/ethel.tscn",
			GirlName::Nema => "res://Core/Combat/Nema/nema.tscn",
		}
	}
}