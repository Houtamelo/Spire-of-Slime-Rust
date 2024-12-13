use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealApplier {
	pub base_multiplier: Int,
}

impl IApplyOnAny for HealApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		let multiplier = {
			let mut temp = self.base_multiplier;

			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		let dmg = caster.base_stat::<Damage>();

		if dmg.upper() <= 0 {
			return;
		}

		let heal_amount = {
			let mut temp = dmg.sample_single(&mut ctx.rng);

			temp *= multiplier;
			temp /= 100;

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

		if heal_amount <= 0 {
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

		let stamina = {
			let mut temp = target.raw_stat::<CurrentStamina>();
			temp += heal_amount;

			i64::clamp_rg(temp, ..=target.eval_dyn_stat::<MaxStamina>(ctx))
		};

		target.raw_stat_mut::<CurrentStamina>().set(stamina);
	}
}
