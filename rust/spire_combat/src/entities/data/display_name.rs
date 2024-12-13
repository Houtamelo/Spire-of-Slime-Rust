use super::*;
///! We'll hook these to the translation tables in the future

pub trait DisplayName {
	fn display_name(&self) -> &str;
}

impl DisplayName for GirlName {
	fn display_name(&self) -> &str {
		match self {
			GirlName::Ethel => "Ethel",
			GirlName::Nema => "Nema",
		}
	}
}

impl DisplayName for NpcName {
	fn display_name(&self) -> &str {
		match self {
			NpcName::Crabdra => "Crabdra",
			NpcName::Trent => "Trent",
			NpcName::Wolfhydra => "Wolfhydra",
			NpcName::BellPlant => "Bell Plant",
		}
	}
}
