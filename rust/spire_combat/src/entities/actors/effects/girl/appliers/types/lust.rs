use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LustApplier {
	pub base_delta: SaneRange,
}

impl IApplyOnAnyGirl for LustApplier {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		let (min, max) = {
			let mut min_temp = self.base_delta.lower();
			let mut max_temp = self.base_delta.upper();

			if is_crit {
				min_temp *= CRIT_EFFECT_MULTIPLIER;
				min_temp /= 100;
				max_temp *= CRIT_EFFECT_MULTIPLIER;
				max_temp /= 100;
			}

			(min_temp, max_temp)
		};

		let range = SaneRange::new(min, max)?;
		*girl.raw_stat_mut::<Lust>() += range.sample_single(&mut ctx.rng);
	}
}
