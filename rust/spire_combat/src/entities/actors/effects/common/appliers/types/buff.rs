use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuffApplier {
	pub base_duration_ms: Int,
	pub stat: StatEnum,
	pub base_stat_increase: Int,
}

impl IApplyOnAny for BuffApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let stat_increase = {
			let mut temp = self.base_stat_increase;

			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		if stat_increase <= 0 {
			return;
		}

		let status = Buff {
			duration_ms: self.base_duration_ms,
			stat: self.stat,
			stat_increase,
		};

		target.add_status(status);
	}
}