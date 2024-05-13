#[allow(unused_imports)]
use crate::prelude::*;

use rand::prelude::IteratorRandom;
use super::dynamic_stat;

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

static ALL_DYNAMIC_STATS: [DynamicStat; 14] = [
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
		return *ALL_DYNAMIC_STATS.iter().choose(rng).unwrap();
	}
}

dynamic_stat!(struct Toughness , Bound_i8  < -100, 100 >,  i8, DynamicStat::Toughness );
dynamic_stat!(struct StunDef   , Bound_i16 < -100, 300 >, i16, DynamicStat::StunDef   );
dynamic_stat!(struct DebuffRes , Bound_i16 < -300, 300 >, i16, DynamicStat::DebuffRes );
dynamic_stat!(struct DebuffRate, Bound_i16 < -300, 300 >, i16, DynamicStat::DebuffRate);
dynamic_stat!(struct MoveRes   , Bound_i16 < -300, 300 >, i16, DynamicStat::MoveRes   );
dynamic_stat!(struct MoveRate  , Bound_i16 < -300, 300 >, i16, DynamicStat::MoveRate  );
dynamic_stat!(struct PoisonRes , Bound_i16 < -300, 300 >, i16, DynamicStat::PoisonRes );
dynamic_stat!(struct PoisonRate, Bound_i16 < -300, 300 >, i16, DynamicStat::PoisonRate);
dynamic_stat!(struct Speed, Bound_u16 <   20, 300 >, u16, DynamicStat::Speed  );
dynamic_stat!(struct Accuracy, Bound_i16 < -300, 300 >, i16, DynamicStat::Accuracy  );
dynamic_stat!(struct CritRate , Bound_i16 < -300, 300 >, i16, DynamicStat::Crit );
dynamic_stat!(struct Dodge, Bound_i16 < -300, 300 >, i16, DynamicStat::Dodge);
dynamic_stat!(struct Power, Bound_u16 <    0, 500 >, u16, DynamicStat::Power);
dynamic_stat!(struct Composure, Bound_i16 < -100, 300 >, i16, DynamicStat::Composure);