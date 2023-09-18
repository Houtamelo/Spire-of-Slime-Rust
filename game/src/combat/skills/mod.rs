include!("offensive.rs");
include!("defensive.rs");
include!("lewd.rs");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Skill {
	Offensive(OffensiveSkill),
	Defensive(DefensiveSkill),
	Lewd(LewdSkill),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ACCMode {
	CanMiss { acc: isize },
	NeverMiss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DMGMode {
	Power { power: isize, toughness_reduction: isize },
	NoDamage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CRITMode {
	CanCrit { crit: isize },
	NeverCrit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionSetup {
	indexes: [bool; MAX_CHARACTERS_PER_TEAM],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetSetup {
	Allies  { positions: PositionSetup, multi_target: bool, allowance: TargetAllowance },
	Enemies { positions: PositionSetup, multi_target: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetAllowance {
	CanSelf,
	NotSelf,
	OnlySelf
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: isize },
}