#[allow(unused_imports)]
use crate::prelude::*;

///! We'll hook these to the translation tables in the future

#[enum_delegate::delegate(for(CharacterVariant))]
pub trait DisplayName {
	fn display_name(&self) -> &str;
}

impl DisplayName for GirlVariant { 
	fn display_name(&self) -> &str { 
		match self {
			GirlVariant::Ethel => "Ethel",
			GirlVariant::Nema => "Nema",
		}
	}
}

impl DisplayName for NPCVariant { 
	fn display_name(&self) -> &str {
		match self {
			NPCVariant::Crabdra => "Crabdra",
			NPCVariant::Trent => "Trent",
			NPCVariant::Wolfhydra => "Wolfhydra",
			NPCVariant::BellPlant => "Bell Plant",
		}
	}
}

