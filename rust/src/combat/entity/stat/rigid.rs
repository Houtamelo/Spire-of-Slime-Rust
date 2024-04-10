#[allow(unused_imports)]
use crate::*;

use super::rigid_stat;

pub trait RigidStatTrait where Self: DerefMut<Target = Self::Inner>,
                               Self::Inner: Clone + Copy {
	type Inner: From<isize>;
	fn stat_enum() -> RigidStat;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum RigidStat {
	Size,
	MaxStamina,
	CurrentStamina,
	ToughnessReduction,
	Lust,
	Temptation,
	Exhaustion,
	OrgasmLimit,
	OrgasmCount,
}

rigid_stat!(struct Lust, Bound_u8< 0, 200 >, u8, RigidStat::Lust);
rigid_stat!(struct Temptation, Bound_u8 < 0, 100 >, u8, RigidStat::Temptation);
rigid_stat!(struct Exhaustion, Bound_u8 < 0, 100 >, u8, RigidStat::Exhaustion);
rigid_stat!(struct OrgasmLimit, Bound_u8 < 1, 8 >, u8, RigidStat::OrgasmLimit);
rigid_stat!(struct OrgasmCount, Bound_u8 < 0, 8 >, u8, RigidStat::OrgasmCount);
rigid_stat!(struct MaxStamina, Bound_u16 < 1, 500 >, u16, RigidStat::MaxStamina);
rigid_stat!(struct CurrentStamina, Bound_u16 < 0, 500 >, u16, RigidStat::CurrentStamina);
rigid_stat!(struct ToughnessReduction, Bound_u8 < 0, 100 >, u8, RigidStat::ToughnessReduction);
rigid_stat!(struct Size, Bound_u8 < 1, {u8::MAX} >, u8, RigidStat::Size);