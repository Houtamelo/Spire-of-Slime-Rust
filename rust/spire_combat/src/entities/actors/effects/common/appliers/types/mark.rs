use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkApplier {
	pub base_duration_ms: Int,
}

impl IApplyOnAny for MarkApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let duration_ms = {
			let mut temp = self.base_duration_ms;

			if is_crit {
				temp *= CRIT_DURATION_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		target.add_status(Mark { duration_ms });
	}
}
