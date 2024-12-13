use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct GirlDebuffApplier {
	pub base_duration_ms: Int,
	pub base_apply_chance: Option<Int>,
	pub stat: GirlStatEnum,
	pub base_stat_decrease: Int,
}

impl IApplyOnAnyGirl for GirlDebuffApplier {
	fn apply_on_any_girl(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		is_crit: bool,
	) {
		//apply chance is only used when the caster and target are enemies
		if let Some(base_apply_chance) = self.base_apply_chance
			&& are_enemies(caster, target)
		{
			let chance = {
				let mut temp = base_apply_chance;
				temp += caster.eval_dyn_stat::<DebuffRate>(ctx);
				temp -= target.eval_dyn_stat::<DebuffRes>(ctx);
				if is_crit {
					temp += CRIT_CHANCE_MODIFIER;
				}
				temp
			};

			if !ctx.rng.base100_chance(chance) {
				// roll failed
				return;
			}
		}

		let stat_decrease = {
			let mut temp = self.base_stat_decrease;

			if is_crit {
				temp *= CRIT_EFFECT_MULTIPLIER;
				temp /= 100;
			}

			temp
		};

		if stat_decrease <= 0 {
			return;
		}

		if target.has_perk::<DisruptiveManeuvers>() {
			let stat_to_caster = StatEnum::get_random(&mut ctx.rng);

			let random_debuff = Debuff {
				duration_ms: self.base_duration_ms,
				kind: DebuffKind::Standard {
					stat: stat_to_caster,
					stat_decrease,
				},
			};

			caster.add_status(random_debuff);

			let stat = StatEnum::get_random(&mut ctx.rng);

			let status = Debuff {
				duration_ms: self.base_duration_ms,
				kind: DebuffKind::Standard {
					stat,
					stat_decrease,
				},
			};

			target.add_status(status);
		} else {
			let status = GirlDebuff {
				duration_ms: self.base_duration_ms,
				stat: self.stat,
				stat_decrease,
			};

			girl.add_status(status);
		}

		if target.has_perk::<WhatDoesntKillYou>() {
			let status = Buff {
				duration_ms: self.base_duration_ms,
				stat: StatEnum::get_random(&mut ctx.rng),
				stat_increase: stat_decrease,
			};

			target.add_status(status);
		}
	}
}
