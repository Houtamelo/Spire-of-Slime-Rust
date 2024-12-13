use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentHealApplier {
	pub base_duration_ms: Int,
	pub base_heal_per_interval: Int,
}

impl IApplyOnAny for PersistentHealApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let heal_per_interval = {
			let mut temp = self.base_heal_per_interval;

			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			if caster.has_perk::<Affection>()
				&& (target.has_status::<Debuff>() || target.has_status::<Poison>())
			{
				temp *= 130;
				temp /= 100;
			}

			if let Some(Awe { accumulated_ms }) = caster.get_perk_mut() {
				let stacks = Int::clamp_rg(*accumulated_ms / 1000, 0..=8);
				temp *= 100 + stacks * 5;
				temp /= 100;
				accumulated_ms.set(0);
			}

			temp
		};

		if heal_per_interval <= 0 {
			return;
		}

		if caster.has_perk::<Adoration>() {
			let toughness_buff = BuffApplier {
				base_duration_ms: 4000.into(),
				stat: StatEnum::Toughness,
				base_stat_increase: 10.into(),
			};

			toughness_buff.apply_on_target(ctx, caster, target, false);

			if let Some(girl) = target.girl.clone().as_mut() {
				*girl.raw_stat_mut::<Lust>() -= 4;

				let composure_buff = GirlBuffApplier {
					base_duration_ms: 4000.into(),
					stat: GirlStatEnum::Composure,
					base_stat_increase: 10.into(),
				};

				composure_buff.apply_on_target_girl(ctx, caster, target, girl, false);
			}
		}

		let status = PersistentHeal {
			duration_ms: self.base_duration_ms,
			accumulated_ms: 0.into(),
			heal_per_interval,
		};

		target.add_status(status);
	}
}
