use houta_utils::prelude::BoundUSize;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub enum EthelPerk {
	Bruiser_DisruptiveManeuvers,
	Bruiser_EnragingPain { stacks: BoundUSize<0, 6> },
	Bruiser_FocusedSwings,
	Bruiser_Grudge { active: bool },
	Bruiser_Relentless { stacks: usize },
	Crit_Bold { used: bool },
	Crit_Reliable,
	Crit_StaggeringForce,
	Crit_Vicious { stacks: usize },
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
	Tank_ReactiveDefense { stacks: BoundUSize<0, 6> },
	Tank_Spikeful,
	Tank_Vanguard { cooldown_ms: i64},
}

#[derive(Debug, Clone)] pub struct AffectedByParalyzingToxins   { pub caster_guid: GUID, }
#[derive(Debug, Clone)] pub struct AffectedByConcentratedToxins { pub caster_guid: GUID, }