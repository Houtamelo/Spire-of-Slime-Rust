#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NemaPerk {
	Grumpiness, //todo! Needs lewd resolving implementation
	AOE_Hatred { stacks: Bound_u8<0, 4> },
	AOE_Loneliness,
	Regret,
	BattleMage_Agitation,
	BattleMage_Carefree,
	BattleMage_Triumph,
	BattleMage_Trust { accumulated_ms: SaturatedU64 },
	Healer_Adoration,
	Healer_Affection,
	Healer_Alarmed { duration_remaining_ms: SaturatedU64 },
	Healer_Awe { accumulated_ms: SaturatedU64 },
	Poison_Acceptance { accumulated_ms: SaturatedU64 },
	Poison_Disbelief,
	Poison_Madness,
	Poison_Melancholy
}