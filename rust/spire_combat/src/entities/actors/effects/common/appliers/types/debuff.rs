use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct DebuffApplier {
	pub base_duration_ms: Int,
	pub base_apply_chance: Option<Int>,
	pub applier_kind: DebuffApplierKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DebuffApplierKind {
	Standard {
		stat: StatEnum,
		base_stat_decrease: Int,
	},
	StaggeringForce,
}

impl IApplyOnAny for DebuffApplier {
	fn apply_on_any(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
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

		match self.applier_kind {
			DebuffApplierKind::Standard {
				stat,
				base_stat_decrease,
			} => {
				apply_regular_debuff(
					self.base_duration_ms,
					stat,
					base_stat_decrease,
					ctx,
					caster,
					target,
					is_crit,
				);
			}
			DebuffApplierKind::StaggeringForce => {
				apply_staggering_force(self.base_duration_ms, ctx, caster, target, is_crit);
			}
		}
	}
}

fn apply_regular_debuff(
	base_duration_ms: Int,
	mut stat: StatEnum,
	base_stat_decrease: Int,
	ctx: &mut ActorContext,
	caster: &mut Ptr<Actor>,
	target: &mut Ptr<Actor>,
	is_crit: bool,
) {
	let stat_decrease = {
		let mut temp = base_stat_decrease;

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
		stat = StatEnum::get_random_except(&mut ctx.rng, stat);

		let status = Debuff {
			duration_ms: base_duration_ms,
			kind: DebuffKind::Standard {
				stat: StatEnum::get_random_except(&mut ctx.rng, stat),
				stat_decrease,
			},
		};

		caster.add_status(status);
	}

	if target.has_perk::<WhatDoesntKillYou>() {
		let status = Buff {
			duration_ms: base_duration_ms,
			stat: StatEnum::get_random_except(&mut ctx.rng, stat),
			stat_increase: stat_decrease,
		};

		target.add_status(status);
	}

	let status = Debuff {
		duration_ms: base_duration_ms,
		kind: DebuffKind::Standard {
			stat,
			stat_decrease,
		},
	};

	target.add_status(status);
}

fn apply_staggering_force(
	base_duration_ms: Int,
	ctx: &mut ActorContext,
	caster: &mut Ptr<Actor>,
	target: &mut Ptr<Actor>,
	_is_crit: bool,
) {
	let status = Debuff {
		duration_ms: base_duration_ms,
		kind: DebuffKind::StaggeringForce,
	};

	target.add_status(status);
}
