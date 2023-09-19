include!("offensive.rs");
include!("defensive.rs");
include!("lewd.rs");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
	key: Box<String>,
	recovery_ms: i64,
	charge_ms: i64,
	skill_type: SkillType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillType {
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
pub struct Positions {
	indexes: [bool; MAX_CHARACTERS_PER_TEAM],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowedTargets {
	Allies  { positions: Positions, multi_target: bool, allowance: AllyAllowance },
	Enemies { positions: Positions, multi_target: bool },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllyAllowance {
	CanSelf,
	NotSelf,
	OnlySelf
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: isize },
}