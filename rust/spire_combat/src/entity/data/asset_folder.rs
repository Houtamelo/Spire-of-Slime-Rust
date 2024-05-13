use crate::prelude::*;

#[enum_delegate::delegate(for(CharacterVariant))]
pub trait AssetFolder {
	fn asset_folder(&self) -> &'static str;
	fn asset_prefix(&self) -> &'static str;

	fn combat_portrait(&self) -> Result<Ref<Texture>> {
		let path = format!("{}/{}_portrait", self.asset_folder(), self.asset_prefix());
		load_resource_as(&path)
	}
}

impl AssetFolder for GirlVariant {
	fn asset_folder(&self) -> &'static str {
		match self {
			GirlVariant::Ethel => "res://Core/Combat/Characters/Ethel",
			GirlVariant::Nema => "res://Core/Combat/Characters/Nema",
		}
	}
	
	fn asset_prefix(&self) -> &'static str {
		match self {
			GirlVariant::Ethel => "ethel",
			GirlVariant::Nema => "nema",
		}
	}
}

impl AssetFolder for NPCVariant {
	fn asset_folder(&self) -> &'static str {
		match self {
			NPCVariant::Crabdra => "res://Core/Combat/Characters/Crabdra",
			NPCVariant::Trent => "res://Core/Combat/Characters/Trent",
			NPCVariant::Wolfhydra => "res://Core/Combat/Characters/Wolfhydra",
			NPCVariant::BellPlant => "res://Core/Combat/Characters/BellPlant",
		}
	}

	fn asset_prefix(&self) -> &'static str {
		match self {
			NPCVariant::Crabdra => "crabdra",
			NPCVariant::Trent => "trent",
			NPCVariant::Wolfhydra => "wolfhydra",
			NPCVariant::BellPlant => "bell-plant",
		}
	}
}
