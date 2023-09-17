include!("standard.rs");

pub enum ACCMode {
	CanMiss { acc: isize },
	NeverMiss,
}

pub enum DMGMode {
	Power { power: isize, toughness_reduction: isize },
	NoDamage,
}

pub enum CRITMode {
	CanCrit { crit: isize },
	NeverCrit,
}

pub struct PositionSetup {
	indexes: [bool; MAX_CHARACTERS_PER_TEAM],
}

pub enum TargetSetup {
	Allies  { positions: PositionSetup, multi_target: bool, allowance: TargetAllowance },
	Enemies { positions: PositionSetup, multi_target: bool },
}

pub enum TargetAllowance {
	CanSelf,
	NotSelf,
	OnlySelf
}

pub enum UseCounter {
	Unlimited,
	Limited { max_uses: isize },
}