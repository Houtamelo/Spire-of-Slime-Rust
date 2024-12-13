use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GirlBuffApplier {
	pub base_duration_ms: Int,
	pub stat: GirlStatEnum,
	pub base_stat_increase: Int,
}

impl IApplyOnAnyGirl for GirlBuffApplier {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
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

		let status = GirlBuff {
			duration_ms: self.base_duration_ms,
			stat: self.stat,
			stat_increase,
		};

		girl.add_status(status);
	}
}
