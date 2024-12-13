use super::*;

mod defensive;
mod lewd;
mod offensive;

pub use defensive::*;
pub use lewd::*;
pub use offensive::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum Skill {
	Offensive(OffensiveSkill),
	Defensive(DefensiveSkill),
	Lewd(LewdSkill),
}

pub trait SkillData {
	fn variant(&self) -> SkillIdent;
	fn recovery_ms(&self) -> &Int;
	fn charge_ms(&self) -> &Int;
	fn crit(&self) -> &CritMode;
	fn effects_self(&self) -> &[CasterApplierEnum];
	fn effects_target(&self) -> &[TargetApplierEnum];
	fn caster_positions(&self) -> &PositionMatrix;
	fn target_positions(&self) -> &PositionMatrix;
	fn multi_target(&self) -> &bool;
	fn use_counter(&self) -> &UseCounter;
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum AccuracyMode {
	CanMiss { acc: Accuracy },
	NeverMiss,
}

pub enum HitMode {
	Chance(IntPercent),
	AlwaysHits,
}

impl AccuracyMode {
	pub fn eval_hit_chance(
		&self,
		ctx: &ActorContext,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
	) -> HitMode {
		match self {
			AccuracyMode::CanMiss { acc } => {
				let mut chance = Int::from(acc);
				chance += caster.eval_dyn_stat::<Accuracy>(ctx);
				chance -= target.eval_dyn_stat::<Dodge>(ctx);

				if target.has_perk::<Disbelief>() && caster.has_status::<Poison>() {
					chance -= 20;
				}

				HitMode::Chance(chance.into())
			}
			AccuracyMode::NeverMiss => HitMode::AlwaysHits,
		}
	}

	pub fn eval_did_hit(
		&self,
		ctx: &mut ActorContext,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
	) -> bool {
		match self.eval_hit_chance(ctx, caster, target) {
			HitMode::Chance(chance) => ctx.rng.base100_chance(chance),
			HitMode::AlwaysHits => true,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum DmgMode {
	Power {
		power: Power,
		toughness_reduction: ToughnessReduction,
	},
	NoDamage,
}

impl DmgMode {
	pub fn eval_dmg_range(
		&self,
		ctx: &ActorContext,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
		is_crit: bool,
	) -> Option<SaneRange> {
		match self {
			DmgMode::Power {
				power,
				toughness_reduction,
			} => {
				let toughness = {
					let base = target.eval_dyn_stat::<Toughness>(ctx);
					let min = i64::min(*base, 0);
					i64::max(min, *base - **toughness_reduction)
				};

				let total_power = {
					let mut temp = Int::from(power);
					temp *= caster.eval_dyn_stat::<Power>(ctx);
					temp *= 100 - toughness;
					temp /= 10000;
					temp
				};

				let (min, max) = {
					let (mut temp_min, mut temp_max) = caster.base_stat::<Damage>().deconstruct();

					temp_min *= total_power;
					temp_min /= 100;
					temp_max *= total_power;
					temp_max /= 100;

					if is_crit {
						temp_min *= 15;
						temp_min /= 10;
						temp_max *= 15;
						temp_max /= 10;
					}

					(temp_min, temp_max)
				};

				Some(SaneRange::floor(min, max))
			}
			DmgMode::NoDamage => None,
		}
	}

	pub fn eval_did_dmg(
		&self,
		ctx: &mut ActorContext,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
		is_crit: bool,
	) -> Option<Int> {
		let dmg_range = self.eval_dmg_range(ctx, caster, target, is_crit)?;
		let dmg = dmg_range.sample_single(&mut ctx.rng);
		(dmg > 0).then_some(dmg)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum CritMode {
	CanCrit { chance: CritRate },
	NeverCrit,
}

impl CritMode {
	pub fn eval_crit_chance(&self, ctx: &ActorContext, caster: &Ptr<Actor>) -> Option<IntPercent> {
		match self {
			CritMode::CanCrit { chance } => {
				let mut chance = Int::from(chance);
				chance += caster.eval_dyn_stat::<CritRate>(ctx);
				Some(chance.into())
			}
			CritMode::NeverCrit => None,
		}
	}

	pub fn eval_did_crit(&self, ctx: &mut ActorContext, caster: &Ptr<Actor>) -> bool {
		self.eval_crit_chance(ctx, caster)
			.map(|chance| ctx.rng.base100_chance(chance))
			.unwrap_or(false)
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct PositionMatrix(pub [bool; 4]);

impl PositionMatrix {
	pub const ANY: Self = Self([true; 4]);
}

impl Deref for PositionMatrix {
	type Target = [bool; 4];

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for PositionMatrix {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl PositionMatrix {
	pub fn contains(&self, position: Position, size: Size) -> bool {
		let lower_bound = *position;
		let upper_bound = lower_bound + size;

		(lower_bound..upper_bound).any(|index| self[index as usize])
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum AllyRequirement {
	CanSelf,
	NotSelf,
	OnlySelf,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum UseCounter {
	Unlimited,
	Limited { max_uses: BndInt<1, 255> },
}
