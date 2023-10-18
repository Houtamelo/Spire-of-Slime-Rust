use crate::BoundU32;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub enum EthelPerk {
	Bruiser_DisruptiveManeuvers,
	Bruiser_EnragingPain { stacks: BoundU32<0, 6> },
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
	Poison_LingeringToxins, //todo! renamed from "Lingering Toxins", update assets in the future!
	Poison_ConcentratedToxins,
	Poison_ParalyzingToxins,
	Poison_PoisonCoating,
	Tank_Conspicuous, //todo! needs AI implementation
	Tank_Energetic,
	Tank_ReactiveDefense { stacks: BoundU32<0, 6> },
	Tank_Spikeful,
	Tank_Vanguard,
}

#[derive(Debug, Clone)] pub struct AffectedByParalyzingToxins   { pub caster_guid: GUID, }
#[derive(Debug, Clone)] pub struct AffectedByConcentratedToxins { pub caster_guid: GUID, }