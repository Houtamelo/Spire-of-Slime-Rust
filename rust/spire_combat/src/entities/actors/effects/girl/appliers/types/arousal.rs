use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArousalApplier {
	pub base_duration_ms: Int,
	pub base_lust_per_interval: Int,
}

impl IApplyOnAnyGirl for ArousalApplier {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		let lust_per_interval = {
			let mut temp = self.base_lust_per_interval;
			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		if lust_per_interval <= 0 {
			return;
		}

		let status = Arousal {
			duration_ms: self.base_duration_ms,
			accumulated_ms: 0.into(),
			lust_per_interval,
		};

		girl.add_status(status);
	}
}
