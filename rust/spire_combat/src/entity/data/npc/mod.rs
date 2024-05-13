#[allow(unused_imports)]
use crate::prelude::*;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};

pub mod bellplant;
pub mod crabdra;

#[repr(usize)]
#[derive(FromVariant, ToVariant)]
#[derive(VariantNames, EnumCount, EnumString, FromRepr)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NPCVariant {
	Crabdra,
	Trent,
	Wolfhydra,
	BellPlant,
}

impl CharacterData for NPCVariant {
	fn variant(&self) -> CharacterVariant {
		match self {
			NPCVariant::Crabdra => CharacterVariant::NPC(NPCVariant::Crabdra),
			NPCVariant::Trent => CharacterVariant::NPC(NPCVariant::Trent),
			NPCVariant::Wolfhydra => CharacterVariant::NPC(NPCVariant::Wolfhydra),
			NPCVariant::BellPlant => CharacterVariant::NPC(NPCVariant::BellPlant),
		}
	}
	
	fn max_stamina(&self, level: u8) -> MaxStamina {
		let level: i64 = level.squeeze_to();

		let base = match self {
			NPCVariant::Crabdra   => 16 + (level * 20) / 10,
			NPCVariant::Trent     => 18 + (level * 25) / 10,
			NPCVariant::Wolfhydra => 30 + (level * 15) / 10,
			NPCVariant::BellPlant => 12 + (level * 12) / 10,
		};
		
		MaxStamina::new(base.squeeze_to())
	}

	fn dmg(&self, level: u8) -> CheckedRange {
		let level: i64 = level.squeeze_to();
		
		let lower = match self {
			NPCVariant::Crabdra   =>  3 * (100 + level * 14),
			NPCVariant::Trent     =>  5 * (100 + level * 10),
			NPCVariant::Wolfhydra => 10 * (100 + level * 10),
			NPCVariant::BellPlant =>  1 * (100 + level * 15),
		} / 100;

		let upper = ((100 + level * 10) * match self {
			NPCVariant::Crabdra   =>  6 * (100 + level * 14),
			NPCVariant::Trent     => 10 * (100 + level *  8),
			NPCVariant::Wolfhydra =>  4 * (100 + level * 12),
			NPCVariant::BellPlant =>  3 * (100 + level * 15),
		}) / 10000;

		return CheckedRange::floor(lower.squeeze_to(), upper.squeeze_to());
	}

	fn spd(&self, level: u8) -> Speed {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 100 + (level * 13) / 10,
			NPCVariant::Trent     => 100 + (level *  9) / 10,
			NPCVariant::Wolfhydra => 100 + (level * 12) / 10,
			NPCVariant::BellPlant => 100 + (level *  8) / 10,
		};
		
		return Speed::new(value.squeeze_to());
	}

	fn acc(&self, level: u8) -> Accuracy {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 0 + (level * 35) / 10,
			NPCVariant::Trent     => 0 + (level * 32) / 10,
			NPCVariant::Wolfhydra => 0 + (level * 30) / 10,
			NPCVariant::BellPlant => 0 + (level * 35) / 10,
		};
		
		return Accuracy::new(value.squeeze_to());
	}

	fn crit(&self, level: u8) -> CritRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 0 + (level * 10) / 10,
			NPCVariant::Trent     => 0 + (level * 15) / 10,
			NPCVariant::Wolfhydra => 0 + (level * 12) / 10,
			NPCVariant::BellPlant => 0 + (level *  8) / 10,
		};

		return CritRate::new(value.squeeze_to());
	}

	fn dodge(&self, level: u8) -> Dodge {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   =>   5 + (level * 35) / 10,
			NPCVariant::Trent     => -10 + (level * 25) / 10,
			NPCVariant::Wolfhydra =>  25 + (level * 35) / 10,
			NPCVariant::BellPlant => -20 + (level * 20) / 10,
		};

		return Dodge::new(value.squeeze_to());
	}

	fn toughness(&self, level: u8) -> Toughness {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 10 + (level * 18) / 10,
			NPCVariant::Trent     => 25 + (level * 16) / 10,
			NPCVariant::Wolfhydra =>  0 + (level * 10) / 10,
			NPCVariant::BellPlant =>  0 + (level *  7) / 10,
		};

		return Toughness::new(value.squeeze_to());
	}

	fn stun_def(&self, level: u8) -> StunDef {
		let level: i64 = level.squeeze_to();
		let value = match self {
			NPCVariant::Crabdra   =>  20 + (level * 60) / 10,
			NPCVariant::Trent     =>  40 + (level * 70) / 10,
			NPCVariant::Wolfhydra =>  25 + (level * 50) / 10,
			NPCVariant::BellPlant => -20 + (level * 30) / 10,
		};

		return StunDef::new(value.squeeze_to());
	}

	fn debuff_res(&self, level: u8) -> DebuffRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 20 + (level * 50) / 10,
			NPCVariant::Trent     => 30 + (level * 70) / 10,
			NPCVariant::Wolfhydra => 15 + (level * 60) / 10,
			NPCVariant::BellPlant =>  0 + (level * 40) / 10,
		};

		return DebuffRes::new(value.squeeze_to());
	}

	fn debuff_rate(&self, level: u8) -> DebuffRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 0 + (level * 50) / 10,
			NPCVariant::Trent     => 0 + (level * 60) / 10,
			NPCVariant::Wolfhydra => 0 + (level * 65) / 10,
			NPCVariant::BellPlant => 0 + (level * 50) / 10,
		};

		return DebuffRate::new(value.squeeze_to());
	}

	fn move_res(&self, level: u8) -> MoveRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   =>  25 + (level * 60) / 10,
			NPCVariant::Trent     => 100 + (level * 70) / 10,
			NPCVariant::Wolfhydra =>   0 + (level * 50) / 10,
			NPCVariant::BellPlant =>  50 + (level * 70) / 10,
		};

		return MoveRes::new(value.squeeze_to());
	}

	fn move_rate(&self, level: u8) -> MoveRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 0 + (level * 50) / 10,
			NPCVariant::Trent     => 0 + (level * 50) / 10,
			NPCVariant::Wolfhydra => 0 + (level * 50) / 10,
			NPCVariant::BellPlant => 0 + (level * 50) / 10,
		};

		return MoveRate::new(value.squeeze_to());
	}

	fn poison_res(&self, level: u8) -> PoisonRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   =>  0 + (level * 40) / 10,
			NPCVariant::Trent     => 15 + (level * 55) / 10,
			NPCVariant::Wolfhydra =>  0 + (level * 50) / 10,
			NPCVariant::BellPlant => 20 + (level * 70) / 10,
		};

		return PoisonRes::new(value.squeeze_to());
	}

	fn poison_rate(&self, level: u8) -> PoisonRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCVariant::Crabdra   => 0 + (level * 50) / 10,
			NPCVariant::Trent     => 0 + (level * 60) / 10,
			NPCVariant::Wolfhydra => 0 + (level * 50) / 10,
			NPCVariant::BellPlant => 0 + (level * 50) / 10,
		};

		return PoisonRate::new(value.squeeze_to());
	}

	fn skills<'a>(&'a self) -> &Cow<'a, [Skill]> {
		todo!()
	}
}

impl NPCData for NPCVariant {
	fn stamina_amplitude(&self, level: u8) -> u16 {
		match self {
			NPCVariant::Crabdra => (2 * (100 + level * 15)) / 100,
			NPCVariant::Trent => (3 * (100 + level * 15)) / 100,
			NPCVariant::Wolfhydra => (3 * (100 + level * 15)) / 100,
			NPCVariant::BellPlant => (1 * (100 + level * 15)) / 100,
		}.into()
	}
}
