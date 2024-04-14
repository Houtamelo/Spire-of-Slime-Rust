use enum_variant_type::EnumVariantType;
#[allow(unused_imports)]
use crate::*;

use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};
use crate::combat::shared::*;

pub mod bellplant;
pub mod crabdra;

#[repr(usize)]
#[derive(EnumVariantType)]
#[evt(derive(Clone, Copy, Debug, PartialEq, Eq, Hash))]
#[derive(VariantNames)]
#[derive(FromRepr)]
#[derive(EnumCount)]
#[derive(EnumString)]
#[derive(FromVariant, ToVariant)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NPCName {
	Crabdra,
	Trent,
	Wolfhydra,
	BellPlant,
}

// Q: Why bundle all the NPC data into one enum impl?
// R: It makes it easier to compare their stats which helps with balancing.
impl CharacterDataTrait for NPCName {
	fn max_stamina(&self, level: u8, rng: Option<&mut Xoshiro256PlusPlus>) -> MaxStamina {
		let level: i64 = level.squeeze_to();
		
		let base = match self {
			NPCName::Crabdra   => 16 + (level * 20) / 10,
			NPCName::Trent     => 18 + (level * 25) / 10,
			NPCName::Wolfhydra => 30 + (level * 15) / 10,
			NPCName::BellPlant => 12 + (level * 12) / 10,
		};

		if rng.is_none() {
			return MaxStamina::new(base.squeeze_to());
		}

		let amplitude = match self {
			NPCName::Crabdra   => (2 * (100 + level * 15)) / 100,
			NPCName::Trent     => (3 * (100 + level * 15)) / 100,
			NPCName::Wolfhydra => (3 * (100 + level * 15)) / 100,
			NPCName::BellPlant => (1 * (100 + level * 15)) / 100,
		};

		let rng = rng.unwrap();
		let extra_from_rng = rng.gen_range(0..=amplitude);
		
		return MaxStamina::new((base + extra_from_rng).squeeze_to());
	}

	fn dmg(&self, level: u8) -> CheckedRange {
		let level: i64 = level.squeeze_to();
		
		let lower = match self {
			NPCName::Crabdra   =>  3 * (100 + level * 14),
			NPCName::Trent     =>  5 * (100 + level * 10),
			NPCName::Wolfhydra => 10 * (100 + level * 10),
			NPCName::BellPlant =>  1 * (100 + level * 15),
		} / 100;

		let upper = ((100 + level * 10) * match self {
			NPCName::Crabdra   =>  6 * (100 + level * 14),
			NPCName::Trent     => 10 * (100 + level *  8),
			NPCName::Wolfhydra =>  4 * (100 + level * 12),
			NPCName::BellPlant =>  3 * (100 + level * 15),
		}) / 10000;

		return CheckedRange::floor(lower.squeeze_to(), upper.squeeze_to());
	}

	fn spd(&self, level: u8) -> Speed {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 100 + (level * 13) / 10,
			NPCName::Trent     => 100 + (level *  9) / 10,
			NPCName::Wolfhydra => 100 + (level * 12) / 10,
			NPCName::BellPlant => 100 + (level *  8) / 10,
		};
		
		return Speed::new(value.squeeze_to());
	}

	fn acc(&self, level: u8) -> Accuracy {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 0 + (level * 35) / 10,
			NPCName::Trent     => 0 + (level * 32) / 10,
			NPCName::Wolfhydra => 0 + (level * 30) / 10,
			NPCName::BellPlant => 0 + (level * 35) / 10,
		};
		
		return Accuracy::new(value.squeeze_to());
	}

	fn crit(&self, level: u8) -> CritRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 0 + (level * 10) / 10,
			NPCName::Trent     => 0 + (level * 15) / 10,
			NPCName::Wolfhydra => 0 + (level * 12) / 10,
			NPCName::BellPlant => 0 + (level *  8) / 10,
		};

		return CritRate::new(value.squeeze_to());
	}

	fn dodge(&self, level: u8) -> Dodge {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   =>   5 + (level * 35) / 10,
			NPCName::Trent     => -10 + (level * 25) / 10,
			NPCName::Wolfhydra =>  25 + (level * 35) / 10,
			NPCName::BellPlant => -20 + (level * 20) / 10,
		};

		return Dodge::new(value.squeeze_to());
	}

	fn toughness(&self, level: u8) -> Toughness {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 10 + (level * 18) / 10,
			NPCName::Trent     => 25 + (level * 16) / 10,
			NPCName::Wolfhydra =>  0 + (level * 10) / 10,
			NPCName::BellPlant =>  0 + (level *  7) / 10,
		};

		return Toughness::new(value.squeeze_to());
	}

	fn stun_def(&self, level: u8) -> StunDef {
		let level: i64 = level.squeeze_to();
		let value = match self {
			NPCName::Crabdra   =>  20 + (level * 60) / 10,
			NPCName::Trent     =>  40 + (level * 70) / 10,
			NPCName::Wolfhydra =>  25 + (level * 50) / 10,
			NPCName::BellPlant => -20 + (level * 30) / 10,
		};

		return StunDef::new(value.squeeze_to());
	}

	fn debuff_res(&self, level: u8) -> DebuffRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 20 + (level * 50) / 10,
			NPCName::Trent     => 30 + (level * 70) / 10,
			NPCName::Wolfhydra => 15 + (level * 60) / 10,
			NPCName::BellPlant =>  0 + (level * 40) / 10,
		};

		return DebuffRes::new(value.squeeze_to());
	}

	fn debuff_rate(&self, level: u8) -> DebuffRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 0 + (level * 50) / 10,
			NPCName::Trent     => 0 + (level * 60) / 10,
			NPCName::Wolfhydra => 0 + (level * 65) / 10,
			NPCName::BellPlant => 0 + (level * 50) / 10,
		};

		return DebuffRate::new(value.squeeze_to());
	}

	fn move_res(&self, level: u8) -> MoveRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   =>  25 + (level * 60) / 10,
			NPCName::Trent     => 100 + (level * 70) / 10,
			NPCName::Wolfhydra =>   0 + (level * 50) / 10,
			NPCName::BellPlant =>  50 + (level * 70) / 10,
		};

		return MoveRes::new(value.squeeze_to());
	}

	fn move_rate(&self, level: u8) -> MoveRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 0 + (level * 50) / 10,
			NPCName::Trent     => 0 + (level * 50) / 10,
			NPCName::Wolfhydra => 0 + (level * 50) / 10,
			NPCName::BellPlant => 0 + (level * 50) / 10,
		};

		return MoveRate::new(value.squeeze_to());
	}

	fn poison_res(&self, level: u8) -> PoisonRes {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   =>  0 + (level * 40) / 10,
			NPCName::Trent     => 15 + (level * 55) / 10,
			NPCName::Wolfhydra =>  0 + (level * 50) / 10,
			NPCName::BellPlant => 20 + (level * 70) / 10,
		};

		return PoisonRes::new(value.squeeze_to());
	}

	fn poison_rate(&self, level: u8) -> PoisonRate {
		let level: i64 = level.squeeze_to();
		
		let value = match self {
			NPCName::Crabdra   => 0 + (level * 50) / 10,
			NPCName::Trent     => 0 + (level * 60) / 10,
			NPCName::Wolfhydra => 0 + (level * 50) / 10,
			NPCName::BellPlant => 0 + (level * 50) / 10,
		};

		return PoisonRate::new(value.squeeze_to());
	}
}