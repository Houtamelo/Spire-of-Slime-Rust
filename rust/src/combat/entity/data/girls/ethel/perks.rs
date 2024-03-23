use comfy_bounded_ints::prelude::Bound_u8;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::misc::SaturatedU64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthelPerk {
	Bruiser_DisruptiveManeuvers,
	Bruiser_EnragingPain { stacks: Bound_u8<0, 6> },
	Bruiser_FocusedSwings,
	Bruiser_Grudge { active: bool },
	Bruiser_Relentless { stacks: Bound_u8<0, 10> },
	Crit_Bold { used: bool },
	Crit_Reliable,
	Crit_StaggeringForce,
	Crit_Vicious { stacks: Bound_u8<0, 10> },
	Debuffer_GoForTheEyes,
	Debuffer_HardNogging,
	Debuffer_NoQuarters,
	Debuffer_UnnervingAura,
	Debuffer_WhatDoesntKillYou,
	Duelist_AlluringChallenger,
	Duelist_Anticipation,
	Duelist_EnGarde,
	Duelist_Release,
	Poison_PoisonousSkin, //todo! needs lewd skill resolving implementation
	Poison_LingeringToxins, //todo! renamed from "Lingering Toxins", update assets in the future! Needs implementation
	Poison_ConcentratedToxins,
	Poison_ParalyzingToxins,
	Poison_PoisonCoating,
	Tank_Conspicuous, //todo! needs AI implementation
	Tank_Energetic,
	Tank_ReactiveDefense { stacks: Bound_u8<0, 6> },
	Tank_Spikeful,
	Tank_Vanguard { cooldown_ms: SaturatedU64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AffectedByParalyzingToxins   { 
	pub caster_guid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AffectedByConcentratedToxins { 
	pub caster_guid: Uuid,
}