use crate::BoundU32;

#[derive(Debug, Clone)]
pub enum NemaPerk {
	AOE_Grumpiness,
	AOE_Hatred { stacks: BoundU32<0, 4> },
	AOE_Loneliness,
	AOE_Regret,
	BattleMage_Agitation,
	BattleMage_Carefree,
	BattleMage_Triumph,
	BattleMage_Trust { accumulated_ms: i64 },
	Healer_Adoration,
	Healer_Affection,
	Healer_Alarmed { duration_remaining_ms: i64 },
	Healer_Awe { accumulated_ms: i64 },
	Poison_Acceptance { accumulated_ms: i64 },
	Poison_Disbelief,
	Poison_Madness,
	Poison_Melancholy
}