use super::*;

pub trait AssetFolder {
	fn asset_folder(&self) -> &'static str;
	fn asset_prefix(&self) -> &'static str;

	fn combat_portrait(&self) -> Result<Gd<Texture2D>> {
		let folder = self.asset_folder();
		let prefix = self.asset_prefix();
		load_resource_as(&format!("{folder}/{prefix}_portrait"))
	}
}

impl AssetFolder for GirlName {
	fn asset_folder(&self) -> &'static str {
		match self {
			GirlName::Ethel => "res://Core/Combat/Characters/Ethel",
			GirlName::Nema => "res://Core/Combat/Characters/Nema",
		}
	}

	fn asset_prefix(&self) -> &'static str {
		match self {
			GirlName::Ethel => "ethel",
			GirlName::Nema => "nema",
		}
	}
}

impl AssetFolder for NpcName {
	fn asset_folder(&self) -> &'static str {
		match self {
			NpcName::Crabdra => "res://Core/Combat/Characters/Crabdra",
			NpcName::Trent => "res://Core/Combat/Characters/Trent",
			NpcName::Wolfhydra => "res://Core/Combat/Characters/Wolfhydra",
			NpcName::BellPlant => "res://Core/Combat/Characters/BellPlant",
		}
	}

	fn asset_prefix(&self) -> &'static str {
		match self {
			NpcName::Crabdra => "crabdra",
			NpcName::Trent => "trent",
			NpcName::Wolfhydra => "wolfhydra",
			NpcName::BellPlant => "bell-plant",
		}
	}
}
