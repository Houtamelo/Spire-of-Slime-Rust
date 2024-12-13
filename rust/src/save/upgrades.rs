use crate::internal_prelude::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PrimaryUpgradeCount {
	acc: Int,
	dodge: Int,
	crit: Int,
	toughness: Int,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SecondaryUpgradeCount {
	stun_def:    Int,
	move_res:    Int,
	debuff_res:  Int,
	poison_res:  Int,
	move_rate:   Int,
	debuff_rate: Int,
	poison_rate: Int,
	composure:   Int,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryUpgrade {
	Acc  = 0,
	Dodge = 1,
	Crit = 2,
	Toughness = 3,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecondaryUpgrade {
	StunDef = 0,
	MoveRes = 1,
	DebuffRes = 2,
	PoisonRes = 3,
	MoveRate = 4,
	DebuffRate = 5,
	PoisonRate = 6,
	Composure = 8,
}
