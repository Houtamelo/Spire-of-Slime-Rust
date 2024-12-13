use super::*;

define_girl_perks! {
	NemaPerk as GirlPerk::Nema {
		// AOE
		Grumpiness, //todo! Needs lewd resolving implementation
		Hatred { stacks: BndInt<0, 4> },
		Loneliness,
		Regret,

		// Battle Mage
		Agitation,
		Carefree,
		Triumph,

		// Healer
		Adoration,
		Affection,

		// Poison
		Disbelief,
		Madness,
		Melancholy,

		// Uncategorized Todo!
		@NO_IMPL Awe { accumulated_ms: BndInt<0, 8000> },
		@NO_IMPL Alarmed { duration_remaining_ms: Int },
		@NO_IMPL Trust { accumulated_ms: BndInt<0, 7000> },
		@NO_IMPL Acceptance { accumulated_ms: Int },
	}
}

impl Grumpiness {
	pub fn apply_status(ctx: &mut ActorContext, actor: &mut Ptr<Actor>) {
		const SPD_BUFF: BuffApplier = BuffApplier {
			base_duration_ms: int!(3000),
			stat: StatEnum::Speed,
			base_stat_increase: int!(15),
		};
		SPD_BUFF.apply_on_caster(ctx, actor, false);

		const TOUGHNESS_BUFF: BuffApplier = BuffApplier {
			base_duration_ms: int!(4000),
			stat: StatEnum::Toughness,
			base_stat_increase: int!(15),
		};
		TOUGHNESS_BUFF.apply_on_caster(ctx, actor, false);

		if let Some(girl) = &mut actor.girl.clone() {
			const COMPOSURE_DEBUFF: GirlDebuffApplier = GirlDebuffApplier {
				base_duration_ms: int!(4000),
				base_apply_chance: None,
				stat: GirlStatEnum::Composure,
				base_stat_decrease: int!(15),
			};
			COMPOSURE_DEBUFF.apply_on_caster_girl(ctx, actor, girl, false);
		}
	}
}

impl IGirlPerk for Awe {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		self.accumulated_ms += delta_ms;
		PerkTickResult::Active
	}
}

impl IGirlPerk for Alarmed {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		self.duration_remaining_ms -= delta_ms;
		PerkTickResult::Active
	}
}

impl IGirlPerk for Trust {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		self.accumulated_ms += delta_ms;
		PerkTickResult::Active
	}
}

impl IGirlPerk for Acceptance {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		if actor.has_status::<Poison>() {
			self.accumulated_ms += delta_ms;
		}

		let round_seconds = self.accumulated_ms / 1000;
		if round_seconds > 0 {
			self.accumulated_ms -= round_seconds * 1000;
			*girl.raw_stat_mut::<Lust>() -= round_seconds * 3;
		}

		PerkTickResult::Active
	}
}
