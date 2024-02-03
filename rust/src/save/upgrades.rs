use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PrimaryUpgradeCount {
	acc: u8,
	dodge: u8,
	crit: u8,
	toughness: u8,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SecondaryUpgradeCount {
	stun_def: u8,
	move_res: u8,
	debuff_res: u8,
	poison_res: u8,
	move_rate: u8,
	debuff_rate: u8,
	poison_rate: u8,
	composure: u8,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimaryUpgrade {
	Acc = 0,
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