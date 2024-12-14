pub use dyn_stats::*;
pub use raw_stats::*;

use super::*;

define_num_stats! {
	MaxStamina[1, 1337],
	ToughnessReduction[0, 100],
	Accuracy[-300, 300],
	CritRate[-300, 300],
	Dodge[-300, 300],
	Toughness[-100, 100],
	Power[0, 500],
	Speed[20, 300],
	DebuffRes[-300, 300],
	DebuffRate[-300, 300],
	PoisonRes[-300, 300],
	PoisonRate[-300, 300],
	MoveRes[-300, 300],
	MoveRate[-300, 300],
	StunDef[-100, 300],
	Size[1, 4],
	CurrentStamina[0, 1337],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
#[repr(transparent)]
pub struct Damage(SaneRange);

impl From<SaneRange> for Damage {
	fn from(value: SaneRange) -> Self { Self(value) }
}

impl From<Damage> for SaneRange {
	fn from(value: Damage) -> Self { value.0 }
}

mod dyn_stats {
	pub use type_table::BaseStats;
	pub use value_table::StatTable;

	use super::*;

	define_stat_tables! {
		StatEnum {
			MaxStamina,
			ToughnessReduction,
			Damage,
			Accuracy,
			CritRate,
			Dodge,
			Toughness,
			Power,
			Speed,
			DebuffRes,
			DebuffRate,
			PoisonRes,
			PoisonRate,
			MoveRes,
			MoveRate,
			StunDef,
		}

		TRAIT: AsCommonStat;
		TABLE: BaseStats;
		VALUE_TABLE: StatTable;
	}
}

mod raw_stats {
	pub use type_table::RawStats;
	pub use value_table::RawStatTable;

	use super::*;

	define_stat_tables! {
		RawStatEnum {
			Size,
			CurrentStamina,
		}

		TRAIT: AsRawStat;
		TABLE: RawStats;
		VALUE_TABLE: RawStatTable;
	}
}

impl Actor {
	pub fn eval_dyn_stat<Stat>(&self, ctx: &ActorContext) -> Stat
	where Self: EvalDynStat<Stat> {
		self.private_dyn_stat(ctx)
	}
}

trait EvalDynStat<Stat> {
	fn private_dyn_stat(&self, ctx: &ActorContext) -> Stat;
}

impl<Stat> EvalDynStat<Stat> for Actor
where
	Actor: EvalUnbuffed<Stat>,
	Int: From<Stat>,
	Stat: From<i64> + Clone + Copy + AsCommonStat + Sized,
{
	fn private_dyn_stat(&self, ctx: &ActorContext) -> Stat {
		let stat_enum = &Stat::as_enum();
		let mut unbuffed_stat: Int = self.eval_unbuffed(ctx).into();

		for status in self.statuses.values() {
			if let Some(Buff {
				stat,
				stat_increase,
				..
			}) = status.as_variant_ref()
				&& stat == stat_enum
			{
				unbuffed_stat += stat_increase;
			} else if let Some(Debuff { kind, .. }) = status.as_variant_ref() {
				match kind {
					DebuffKind::Standard {
						stat,
						stat_decrease,
					} => {
						if stat == stat_enum {
							unbuffed_stat -= stat_decrease;
						}
					}
					DebuffKind::StaggeringForce { .. } => {
						if matches!(
							stat_enum,
							StatEnum::Toughness
								| StatEnum::StunDef | StatEnum::DebuffRes
								| StatEnum::PoisonRes | StatEnum::MoveRes
						) {
							unbuffed_stat -= 10;
						}
					}
				}
			}
		}

		Stat::from(*unbuffed_stat)
	}
}

trait EvalUnbuffed<Stat> {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Stat;
}

impl StatEnum {
	pub fn get_random_except(rng: &mut Xoshiro256PlusPlus, except: StatEnum) -> StatEnum {
		loop {
			let stat = Self::get_random(rng);
			if stat != except {
				return stat;
			}
		}
	}

	pub fn get_random(rng: &mut Xoshiro256PlusPlus) -> StatEnum {
		Self::ALL.iter().choose(rng).cloned().unwrap()
	}
}

impl EvalUnbuffed<Accuracy> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Accuracy {
		let mut value: Int = self.base_stat::<Accuracy>().into();

		if let Some(Relentless { stacks }) = self.get_perk() {
			value -= stacks * 5;
		}

		if self.has_perk::<Carefree>() && !self.has_status::<Poison>() {
			value += 10;
		}

		if let Some(Trust { accumulated_ms }) = self.get_perk() {
			let stacks = Int::clamp_rg(accumulated_ms / 1000, 0..=7);
			value += stacks * 2;
		}

		if self.is_affected_by(PoisonAdditive::ConcentratedToxins) {
			value += 5;
		}

		value.into()
	}
}

impl EvalUnbuffed<CritRate> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> CritRate {
		let base: Int = self.base_stat::<CritRate>().into();
		let mut value = base;

		if let Some(Vicious { stacks }) = self.get_perk() {
			value += stacks * 10;
		}

		if self.has_perk::<Reliable>() {
			value -= base;
		}

		if let Some(Trust { accumulated_ms }) = self.get_perk() {
			let stacks = Int::clamp_rg(accumulated_ms / 1000, 0..=7);
			value += stacks * 2;
		}

		value.into()
	}
}

impl EvalUnbuffed<Dodge> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Dodge {
		let mut value: Int = self.base_stat::<Dodge>().into();

		if self.has_perk::<Anticipation>() && self.has_status::<Riposte>() {
			value += 15;
		}

		if let Some(Alarmed {
			duration_remaining_ms,
		}) = self.get_perk()
			&& duration_remaining_ms > 0
		{
			value += 50;
		}

		if self.has_perk::<Carefree>() && !self.has_status::<Debuff>() {
			value += 10;
		}

		value.into()
	}
}

impl EvalUnbuffed<MoveRes> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> MoveRes { self.base_stat::<MoveRes>() }
}

impl EvalUnbuffed<MoveRate> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> MoveRate { self.base_stat::<MoveRate>() }
}

impl EvalUnbuffed<Power> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Power {
		let mut value: Int = self.base_stat::<Power>().into();

		if self.has_perk::<Spikeful>() {
			// we care about the base toughness, not the dyn one.
			let base = self.base_stat::<Toughness>();
			value += Int::clamp_rg(base, 0..=30);
		}

		if let Some(EnragingPain { stacks }) = self.get_perk() {
			value += stacks * 5;
		}

		if self.has_perk::<Reliable>()
			&& let base_crit @ 1.. = *self.base_stat::<CritRate>()
		{
			value += base_crit;
		}

		if self.has_perk::<Agitation>()
			&& let spd_above_100 @ 1.. = (self.eval_dyn_stat::<Speed>(ctx) - 100)
		{
			value += spd_above_100;
		}

		if self.is_affected_by(PoisonAdditive::Madness) {
			value += 25;
		}

		value.into()
	}
}

impl EvalUnbuffed<Speed> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Speed {
		let mut value: Int = self.base_stat::<Speed>().into();

		let total_paralyzing_poison = self.statuses.values().fold(int!(0), |mut sum, effect| {
			if let Some(poison) = effect.as_variant_ref::<Poison>()
				&& any_matches!(poison.additives, PoisonAdditive::ParalyzingToxins)
			{
				sum += poison.poison_per_interval;
			}

			sum
		});

		value -= i64::clamp_rg(total_paralyzing_poison * 3, 0..=30);

		if self.has_perk::<EnGarde>() && self.has_status::<Riposte>() {
			value -= 20;
		}

		if let Some(Trust { accumulated_ms }) = self.get_perk() {
			let stacks = i64::clamp_rg(accumulated_ms / 1000, 0..=7);
			value += stacks * 3;
		}

		value.into()
	}
}

impl EvalUnbuffed<Toughness> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> Toughness {
		let mut value: Int = self.base_stat::<Toughness>().into();

		if let Some(ReactiveDefense { stacks }) = self.get_perk() {
			value += stacks * 4;
		}

		if self.has_perk::<Hatred>() {
			value += 10;
		}

		value.into()
	}
}

impl EvalUnbuffed<DebuffRes> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> DebuffRes {
		let mut value: Int = self.base_stat::<DebuffRes>().into();

		if self.has_perk::<HardNogging>()
			&& let Some(ActorState::Stunned { .. }) = ctx.actor_state(self.id)
		{
			value += 25;
		}

		value.into()
	}
}

impl EvalUnbuffed<DebuffRate> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> DebuffRate { self.base_stat::<DebuffRate>() }
}

impl EvalUnbuffed<PoisonRes> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> PoisonRes { self.base_stat::<PoisonRes>() }
}

impl EvalUnbuffed<PoisonRate> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> PoisonRate {
		let mut value: Int = self.base_stat::<PoisonRate>().into();

		if self.has_perk::<Melancholy>() {
			let stamina_lost = {
				let max_stamina = self.eval_dyn_stat::<MaxStamina>(ctx);
				let curr_stamina = self.raw_stat::<CurrentStamina>();
				max_stamina - *curr_stamina
			};

			value += i64::max(stamina_lost, 0);
		}

		value.into()
	}
}

impl EvalUnbuffed<StunDef> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> StunDef {
		let mut value: Int = self.base_stat::<StunDef>().into();

		if self.has_perk::<HardNogging>() && self.has_status::<Debuff>() {
			value += 25;
		}

		value.into()
	}
}

impl EvalUnbuffed<MaxStamina> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> MaxStamina {
		let mut value: Int = self.base_stat::<MaxStamina>().into();

		if self.has_perk::<Energetic>() {
			value *= 125;
			value /= 100;
		}

		value.into()
	}
}

impl EvalUnbuffed<ToughnessReduction> for Actor {
	fn eval_unbuffed(&self, ctx: &ActorContext) -> ToughnessReduction {
		self.base_stat::<ToughnessReduction>()
	}
}
