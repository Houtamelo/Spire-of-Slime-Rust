use crate::BoundU32;

#[derive(Debug, Clone)]
pub enum NemaPerk {
	AOE(Category_AOE),
	BattleMage(Category_BattleMage),
	Healer(Category_Healer),
	Poison(Category_Poison),
}

#[derive(Debug, Clone)]
pub enum Category_AOE {
	Grumpiness,
	Hatred { stacks: BoundU32<0, 4> },
	Loneliness,
	Regret
}

#[derive(Debug, Clone)]
pub enum Category_BattleMage {
	Agitation,
	Carefree,
	Triumph,
	Trust { stacks: BoundU32<0, 5>, accumulated_ms: i64 },
}

#[derive(Debug, Clone)]
pub enum Category_Healer {
	Adoration,
	Affection,
	Alarmed,
	Awe { stacks: BoundU32<0, 5>, accumulated_ms: i64 },
}

#[derive(Debug, Clone)]
pub enum Category_Poison {
	Acceptance { accumulated_ms: i64 },
	Disbelief,
	Madness,
	Melancholy
}