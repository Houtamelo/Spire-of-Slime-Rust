use super::dynamic_stat;
use comfy_bounded_ints::prelude::*;
use std::ops::{Deref, DerefMut};
use comfy_bounded_ints::prelude::Bound_i8;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

pub trait DynamicStatTrait where Self: DerefMut<Target = Self::Inner> {
	type Inner: Clone + Copy;
	fn stat_enum() -> DynamicStat;
	fn from_i64(value: i64) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum DynamicStat {
	Accuracy,
	Crit,
	Dodge,
	Toughness,
	Power,
	Speed,
	DebuffRes,
	DebuffRate,
	PoisonRes,
	PoisonRate,
	MoveRes,
	MoveRate,
	StunDef,
	Composure
}

static AllDynamicStats: [DynamicStat; 14] = [
	DynamicStat::Accuracy,
	DynamicStat::Crit,
	DynamicStat::Dodge,
	DynamicStat::Toughness,
	DynamicStat::Power,
	DynamicStat::Speed,
	DynamicStat::DebuffRes,
	DynamicStat::DebuffRate,
	DynamicStat::PoisonRes,
	DynamicStat::PoisonRate,
	DynamicStat::MoveRes,
	DynamicStat::MoveRate,
	DynamicStat::StunDef,
	DynamicStat::Composure,
];

use rand::prelude::IteratorRandom;

impl DynamicStat {
	pub fn get_random_except(rng: &mut Xoshiro256PlusPlus, except: DynamicStat) -> DynamicStat {
		loop {
			let stat = Self::get_random(rng);
			if stat != except {
				return stat;
			}
		}
	}

	pub fn get_random(rng: &mut Xoshiro256PlusPlus) -> DynamicStat {
		return *AllDynamicStats.iter().choose(rng).unwrap();
	}
}

dynamic_stat!(struct Toughness , Bound_i8  < -100, 100 >,  i8, DynamicStatTrait, DynamicStat, DynamicStat::Toughness );
dynamic_stat!(struct StunDef   , Bound_i16 < -100, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::StunDef   );
dynamic_stat!(struct DebuffRes , Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::DebuffRes );
dynamic_stat!(struct DebuffRate, Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::DebuffRate);
dynamic_stat!(struct MoveRes   , Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::MoveRes   );
dynamic_stat!(struct MoveRate  , Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::MoveRate  );
dynamic_stat!(struct PoisonRes , Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::PoisonRes );
dynamic_stat!(struct PoisonRate, Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::PoisonRate);
dynamic_stat!(struct Speed, Bound_u16 <   20, 300 >, u16, DynamicStatTrait, DynamicStat, DynamicStat::Speed  );
dynamic_stat!(struct Accuracy, Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::Accuracy  );
dynamic_stat!(struct CritChance , Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::Crit );
dynamic_stat!(struct Dodge, Bound_i16 < -300, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::Dodge);
dynamic_stat!(struct Power, Bound_u16 <    0, 500 >, u16, DynamicStatTrait, DynamicStat, DynamicStat::Power);
dynamic_stat!(struct Composure, Bound_i16 < -100, 300 >, i16, DynamicStatTrait, DynamicStat, DynamicStat::Composure);