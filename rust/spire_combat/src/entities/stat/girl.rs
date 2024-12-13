pub use dyn_stats::*;
pub use raw_stats::*;

use super::*;

define_num_stats! {
	Composure[-100, 300],
	OrgasmLimit[1, 8],
	Lust[0, 200],
	Temptation[0, 100],
	Exhaustion[0, 100],
	OrgasmCount[0, 8],
	Corruption[0, 100],
}

mod dyn_stats {
	pub use type_table::{BaseGirlStats, GetInTable as GetGirlCommon};
	pub use value_table::{GetInTable as GetInGirlStatTable, GirlStatTable};

	use super::*;

	define_stat_tables! {
		GirlStatEnum {
			Composure,
			OrgasmLimit,
		}

		TRAIT: AsGirlStat;
		TABLE: BaseGirlStats;
		VALUE_TABLE: GirlStatTable;
	}
}

mod raw_stats {
	pub use type_table::{GetInTable as GetGirlRawCommon, RawGirlStats};
	pub use value_table::{GetInTable as GetGirlRawInStatTable, RawGirlStatTable};

	use super::*;

	define_stat_tables! {
		RawGirlStatEnum {
			Lust,
			Temptation,
			Exhaustion,
			OrgasmCount,
			Corruption,
		}

		TRAIT: AsRawGirlStat;
		TABLE: RawGirlStats;
		VALUE_TABLE: RawGirlStatTable;
	}
}

impl Actor {
	pub fn eval_dyn_girl_stat<Stat>(&self, girl: &Girl, ctx: &ActorContext) -> Stat
	where Self: EvalDynStat<Stat> {
		self.private_dyn_girl_stat(girl, ctx)
	}
}

trait EvalDynStat<Stat> {
	fn private_dyn_girl_stat(&self, girl: &Girl, ctx: &ActorContext) -> Stat;
}

impl<Stat> EvalDynStat<Stat> for Actor
where
	Actor: EvalUnbuffed<Stat>,
	Int: From<Stat>,
	Stat: From<Int> + Clone + Copy + AsGirlStat + Sized,
{
	fn private_dyn_girl_stat(&self, girl: &Girl, ctx: &ActorContext) -> Stat {
		let stat_enum = Stat::as_enum();
		let mut unbuffed_stat: Int = self.eval_unbuffed(girl, ctx).into();

		for status in girl.statuses.values() {
			if let Some(GirlBuff {
				stat,
				stat_increase,
				..
			}) = status.as_variant_ref()
				&& *stat == stat_enum
			{
				unbuffed_stat += stat_increase;
			} else if let Some(GirlDebuff {
				stat,
				stat_decrease,
				..
			}) = status.as_variant_ref()
			{
				if *stat == stat_enum {
					unbuffed_stat -= stat_decrease;
				}
			}
		}

		Stat::from(unbuffed_stat)
	}
}

impl GirlStatEnum {
	pub fn get_random(rng: &mut Xoshiro256PlusPlus) -> GirlStatEnum {
		Self::ALL
			.iter()
			.choose(rng)
			.cloned()
			.unwrap_or(GirlStatEnum::Composure)
	}
}

trait EvalUnbuffed<Stat> {
	fn eval_unbuffed(&self, girl: &Girl, ctx: &ActorContext) -> Stat;
}

impl EvalUnbuffed<Composure> for Actor {
	fn eval_unbuffed(&self, girl: &Girl, ctx: &ActorContext) -> Composure {
		let mut value = *girl.base_stat::<Composure>();

		if self.has_perk::<Agitation>()
			&& let spd_bellow_100 @ ..=-1 = (100 - self.eval_dyn_stat::<Speed>(ctx))
		{
			value += spd_bellow_100;
		}

		if self.has_perk::<Grumpiness>()
			&& let Some(ActorState::Downed { .. }) = ctx.actor_state(self.id)
		{
			value += 30;
		}

		value.into()
	}
}

impl EvalUnbuffed<OrgasmLimit> for Actor {
	fn eval_unbuffed(&self, girl: &Girl, ctx: &ActorContext) -> OrgasmLimit {
		girl.base_stat::<OrgasmLimit>()
	}
}
