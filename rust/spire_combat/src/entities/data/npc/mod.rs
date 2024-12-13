use super::*;

mod bellplant;
mod crabdra;

pub use bellplant::*;
pub use crabdra::*;

#[repr(usize)]
#[derive(
	VariantNames,
	EnumCount,
	EnumString,
	FromRepr,
	Debug,
	Clone,
	Copy,
	Serialize,
	Deserialize,
	PartialEq,
	Eq,
	Hash,
)]
pub enum NpcName {
	Crabdra,
	Trent,
	Wolfhydra,
	BellPlant,
}

impl CharacterData for NpcName {
	fn variant(&self) -> CharacterVariant {
		match self {
			NpcName::Crabdra => CharacterVariant::NPC(NpcName::Crabdra),
			NpcName::Trent => CharacterVariant::NPC(NpcName::Trent),
			NpcName::Wolfhydra => CharacterVariant::NPC(NpcName::Wolfhydra),
			NpcName::BellPlant => CharacterVariant::NPC(NpcName::BellPlant),
		}
	}

	fn max_stamina(&self, level: i64) -> MaxStamina {
		match self {
			NpcName::Crabdra => 16 + (level * 20) / 10,
			NpcName::Trent => 18 + (level * 25) / 10,
			NpcName::Wolfhydra => 30 + (level * 15) / 10,
			NpcName::BellPlant => 12 + (level * 12) / 10,
		}
		.into()
	}

	fn dmg(&self, level: i64) -> SaneRange {
		let lower = match self {
			NpcName::Crabdra => 3 * (100 + level * 14),
			NpcName::Trent => 5 * (100 + level * 10),
			NpcName::Wolfhydra => 10 * (100 + level * 10),
			NpcName::BellPlant => 1 * (100 + level * 15),
		} / 100;

		let upper = ((100 + level * 10)
			* match self {
				NpcName::Crabdra => 6 * (100 + level * 14),
				NpcName::Trent => 10 * (100 + level * 8),
				NpcName::Wolfhydra => 4 * (100 + level * 12),
				NpcName::BellPlant => 3 * (100 + level * 15),
			}) / 10000;

		SaneRange::floor(lower, upper)
	}

	fn spd(&self, level: i64) -> Speed {
		match self {
			NpcName::Crabdra => 100 + (level * 13) / 10,
			NpcName::Trent => 100 + (level * 9) / 10,
			NpcName::Wolfhydra => 100 + (level * 12) / 10,
			NpcName::BellPlant => 100 + (level * 8) / 10,
		}
		.into()
	}

	fn acc(&self, level: i64) -> Accuracy {
		match self {
			NpcName::Crabdra => 0 + (level * 35) / 10,
			NpcName::Trent => 0 + (level * 32) / 10,
			NpcName::Wolfhydra => 0 + (level * 30) / 10,
			NpcName::BellPlant => 0 + (level * 35) / 10,
		}
		.into()
	}

	fn crit(&self, level: i64) -> CritRate {
		match self {
			NpcName::Crabdra => 0 + (level * 10) / 10,
			NpcName::Trent => 0 + (level * 15) / 10,
			NpcName::Wolfhydra => 0 + (level * 12) / 10,
			NpcName::BellPlant => 0 + (level * 8) / 10,
		}
		.into()
	}

	fn dodge(&self, level: i64) -> Dodge {
		match self {
			NpcName::Crabdra => 5 + (level * 35) / 10,
			NpcName::Trent => -10 + (level * 25) / 10,
			NpcName::Wolfhydra => 25 + (level * 35) / 10,
			NpcName::BellPlant => -20 + (level * 20) / 10,
		}
		.into()
	}

	fn toughness(&self, level: i64) -> Toughness {
		match self {
			NpcName::Crabdra => 10 + (level * 18) / 10,
			NpcName::Trent => 25 + (level * 16) / 10,
			NpcName::Wolfhydra => 0 + (level * 10) / 10,
			NpcName::BellPlant => 0 + (level * 7) / 10,
		}
		.into()
	}

	fn stun_def(&self, level: i64) -> StunDef {
		match self {
			NpcName::Crabdra => 20 + (level * 60) / 10,
			NpcName::Trent => 40 + (level * 70) / 10,
			NpcName::Wolfhydra => 25 + (level * 50) / 10,
			NpcName::BellPlant => -20 + (level * 30) / 10,
		}
		.into()
	}

	fn debuff_res(&self, level: i64) -> DebuffRes {
		match self {
			NpcName::Crabdra => 20 + (level * 50) / 10,
			NpcName::Trent => 30 + (level * 70) / 10,
			NpcName::Wolfhydra => 15 + (level * 60) / 10,
			NpcName::BellPlant => 0 + (level * 40) / 10,
		}
		.into()
	}

	fn debuff_rate(&self, level: i64) -> DebuffRate {
		match self {
			NpcName::Crabdra => 0 + (level * 50) / 10,
			NpcName::Trent => 0 + (level * 60) / 10,
			NpcName::Wolfhydra => 0 + (level * 65) / 10,
			NpcName::BellPlant => 0 + (level * 50) / 10,
		}
		.into()
	}

	fn move_res(&self, level: i64) -> MoveRes {
		match self {
			NpcName::Crabdra => 25 + (level * 60) / 10,
			NpcName::Trent => 100 + (level * 70) / 10,
			NpcName::Wolfhydra => 0 + (level * 50) / 10,
			NpcName::BellPlant => 50 + (level * 70) / 10,
		}
		.into()
	}

	fn move_rate(&self, level: i64) -> MoveRate {
		match self {
			NpcName::Crabdra => 0 + (level * 50) / 10,
			NpcName::Trent => 0 + (level * 50) / 10,
			NpcName::Wolfhydra => 0 + (level * 50) / 10,
			NpcName::BellPlant => 0 + (level * 50) / 10,
		}
		.into()
	}

	fn poison_res(&self, level: i64) -> PoisonRes {
		match self {
			NpcName::Crabdra => 0 + (level * 40) / 10,
			NpcName::Trent => 15 + (level * 55) / 10,
			NpcName::Wolfhydra => 0 + (level * 50) / 10,
			NpcName::BellPlant => 20 + (level * 70) / 10,
		}
		.into()
	}

	fn poison_rate(&self, level: i64) -> PoisonRate {
		match self {
			NpcName::Crabdra => 0 + (level * 50) / 10,
			NpcName::Trent => 0 + (level * 60) / 10,
			NpcName::Wolfhydra => 0 + (level * 50) / 10,
			NpcName::BellPlant => 0 + (level * 50) / 10,
		}
		.into()
	}

	fn skills(&self) -> &[Skill] { todo!() }
}

impl NPCData for NpcName {
	fn stamina_amplitude(&self, level: i64) -> Int {
		match self {
			NpcName::Crabdra => (2 * (100 + level * 15)) / 100,
			NpcName::Trent => (3 * (100 + level * 15)) / 100,
			NpcName::Wolfhydra => (3 * (100 + level * 15)) / 100,
			NpcName::BellPlant => (1 * (100 + level * 15)) / 100,
		}
		.into()
	}
}
