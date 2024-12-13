use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StunApplier {
	pub base_force: Int,
}

impl IApplyOnAny for StunApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let force = {
			let mut temp = self.base_force;

			if is_crit {
				temp += CRIT_CHANCE_MODIFIER;
			}

			*temp as f64
		};

		let def = *target.eval_dyn_stat::<StunDef>(ctx) as f64;

		let dividend = force + (force * force / 500.0) - def - (def * def / 500.0);
		let divisor = 125.0 + (force * 0.25) + (def * 0.25) + (force * def * 0.0005);

		let bonus_redundancy_f64 = (dividend / divisor) * 4000.0;

		if bonus_redundancy_f64 > 0. {
			let bonus_redundancy_f64 = f64::clamp(bonus_redundancy_f64, 1., u64::MAX as f64);
			let bonus_redundancy_ms = Int::from(bonus_redundancy_f64.round() as u64);

			*target.stun_redundancy_ms.get_or_insert_default() += bonus_redundancy_ms;
		}
	}
}
