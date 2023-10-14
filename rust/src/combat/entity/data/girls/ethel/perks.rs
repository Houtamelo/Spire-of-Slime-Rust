use crate::BoundU32;

#[derive(Debug, Clone)]
pub enum EthelPerk {
	Bruiser(Category_Bruiser),
	Crit(Category_Crit),
	Debuffer(Category_Debuffer),
	Duelist(Category_Duelist),
	Poison(Category_Poison),
	Tank(Category_Tank),
}

#[derive(Debug, Clone)]
pub enum Category_Bruiser {
	DisruptiveManeuvers,
	EnragingPain { stacks: BoundU32<0, 6> },
	FocusedSwings,
	Grudge { active: bool },
	Relentless { stacks: usize },
}

#[derive(Debug, Clone)]
pub enum Category_Crit {
	Bold { used: bool },
	Reliable,
	StaggeringForce,
	Vicious { stacks: usize },
}

#[derive(Debug, Clone)]
pub enum Category_Debuffer {
	GoForTheEyes,
	HardNogging,
	NoQuarters,
	UnnervingAura,
	WhatDoesntKillYou,
}

#[derive(Debug, Clone)]
pub enum Category_Duelist {
	AlluringChallenger,
	Anticipation,
	EnGarde,
	Release
}

#[derive(Debug, Clone)]
pub enum Category_Poison {
	AggravatedToxins,
	ConcentratedToxins,
	ParalyzingToxins,
	PoisonCoating,
}

#[derive(Debug, Clone)]
pub enum Category_Tank {
	Conspicuous, //todo! needs AI implementation
	Energetic,
	ReactiveDefense { stacks: BoundU32<0, 6> },
	Spikeful,
	Vanguard,
}