use super::*;

define_girl_perks! {
	EthelPerk as GirlPerk::Ethel {
		// Bruiser
		DisruptiveManeuvers,
		FocusedSwings,
		Relentless { stacks: BndInt<0, 10> },
		EnragingPain { stacks: BndInt<0, 6> },
		Grudge { active: bool },

		// Crit
		Bold { used: bool },
		Reliable,
		StaggeringForce,
		Vicious { stacks: BndInt<0, 10> },

		// Debuffer
		GoForTheEyes,
		HardNogging,
		NoQuarters,
		UnnervingAura,
		WhatDoesntKillYou,

		// Duelist
		AlluringChallenger,
		Anticipation,
		EnGarde,
		Release,

		// Poison
		PoisonousSkin, //todo! needs lewd skill resolving implementation
		LingeringToxins, //todo! renamed from "Lingering Toxins", update assets in the future! Needs implementation
		ConcentratedToxins,
		ParalyzingToxins,
		PoisonCoating,
		AffectedByParalyzingToxins { caster_guid: Uuid },
		AffectedByConcentratedToxins { caster_guid: Uuid },

		// Tank
		Conspicuous, //todo! needs AI implementation
		Energetic,
		Spikeful,
		ReactiveDefense { stacks: BndInt<0, 6> },
		@NO_IMPL Vanguard { cooldown_ms: BndInt<0, 10000> },
	}
}

impl IGirlPerk for Vanguard {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		self.cooldown_ms -= delta_ms;
		PerkTickResult::Active
	}
}
