use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct PoisonApplier {
	pub base_duration_ms: Int,
	pub base_poison_per_interval: Int,
	pub base_apply_chance: Option<Int>,
	additives: HashSet<PoisonAdditive>,
}

impl IApplyOnAny for PoisonApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		// Apply chance is only used when the caster and target are enemies
		if let Some(base_apply_chance) = self.base_apply_chance
			&& are_enemies(caster, target)
		{
			let chance = {
				let mut temp = base_apply_chance;

				temp += caster.eval_dyn_stat::<PoisonRate>(ctx);
				temp -= target.eval_dyn_stat::<PoisonRes>(ctx);

				if is_crit {
					temp += CRIT_CHANCE_MODIFIER;
				}

				temp
			};

			if !ctx.rng.base100_chance(chance) {
				return;
			}
		}

		let poison_per_interval = {
			let mut temp = self.base_poison_per_interval;

			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		if poison_per_interval <= 0 {
			return;
		}

		let (duration_ms, interval_ms) = if any_matches!(self.additives, PoisonAdditive::Madness) {
			let mut temp = self.base_duration_ms;
			temp /= 2;
			(temp, int!(500))
		} else {
			(self.base_duration_ms, int!(1000))
		};

		let status = Poison {
			duration_ms,
			accumulated_ms: 0.into(),
			interval_ms,
			poison_per_interval,
			caster: caster.id,
			additives: self.additives.clone(),
		};

		target.add_status(status);
	}
}

impl PoisonApplier {
	pub fn from_caster(
		caster: &Ptr<Actor>,
		base_duration_ms: impl CramInto<Int>,
		base_poison_per_interval: impl CramInto<Int>,
		base_apply_chance: Option<impl CramInto<Int>>,
	) -> Self {
		let mut additives = HashSet::new();
		if caster.has_perk::<LingeringToxins>() {
			additives.insert(PoisonAdditive::LingeringToxins);
		}
		if caster.has_perk::<ParalyzingToxins>() {
			additives.insert(PoisonAdditive::ParalyzingToxins);
		}
		if caster.has_perk::<ConcentratedToxins>() {
			additives.insert(PoisonAdditive::ConcentratedToxins);
		}
		if caster.has_perk::<Madness>() {
			additives.insert(PoisonAdditive::Madness);
		}

		Self {
			base_duration_ms: base_duration_ms.cram_into(),
			base_poison_per_interval: base_poison_per_interval.cram_into(),
			base_apply_chance: base_apply_chance.map(|bc| bc.cram_into()),
			additives,
		}
	}
}
