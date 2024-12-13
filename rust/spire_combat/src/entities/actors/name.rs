use super::*;

delegated_enum! {
	ENUM_OUT: {
		#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
		pub enum ActorName {
			Girl(GirlName),
			Npc(NpcName),
		}
	}

	DELEGATES: {
		impl trait DisplayName {
			[fn display_name(&self) -> &str]
		}

		impl trait AssetFolder {
			[fn asset_folder(&self) -> &'static str]
			[fn asset_prefix(&self) -> &'static str]
		}
	}
}
