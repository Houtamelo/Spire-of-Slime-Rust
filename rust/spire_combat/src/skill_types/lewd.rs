use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct LewdSkill {
	pub skill_name: SkillIdent,
	pub recovery_ms: Int,
	pub charge_ms: Int,
	pub acc_mode: AccuracyMode,
	pub dmg_mode: DmgMode,
	pub crit_mode: CritMode,
	pub effects_self: Vec<CasterApplierEnum>,
	pub effects_target: Vec<TargetApplierEnum>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target: bool,
	pub use_counter: UseCounter,
}

impl LewdSkill {
	//todo! these should just call offensive::calc_dmg
	pub fn calc_dmg<Caster: 'static, Target: 'static>(
		&self,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
		ctx: &ActorContext,
		is_crit: bool,
	) -> Option<SaneRange> {
		let DmgMode::Power {
			power,
			toughness_reduction,
		} = self.dmg_mode
		else {
			return None;
		};

		let toughness = {
			let base = target.eval_dyn_stat::<Toughness>(ctx);
			let min = i64::min(*base, 0);
			i64::max(min, *base - toughness_reduction)
		};

		let total_power = {
			let mut temp = power;
			temp *= caster.eval_dyn_stat::<Power>(ctx);
			temp *= 100 - toughness;
			temp /= 10000;
			temp
		};

		let (final_min, final_max) = {
			let (mut temp_min, mut temp_max) = caster.base_stat::<Damage>().deconstruct();

			temp_min.set_percent(total_power);
			temp_max.set_percent(total_power);

			if is_crit {
				temp_min.set_percent(150);
				temp_max.set_percent(150);
			}

			(temp_min, temp_max)
		};

		Some(SaneRange::floor(final_min, final_max))
	}

	pub fn calc_hit_chance<Caster: 'static, Target: 'static>(
		&self,
		caster: &Ptr<Actor>,
		target: &Ptr<Actor>,
		ctx: &ActorContext,
	) -> Option<IntPercent> {
		match self.acc_mode {
			AccuracyMode::CanMiss { acc } => {
				let final_acc = {
					let mut temp = acc;
					temp += caster.eval_dyn_stat::<Accuracy>(ctx);
					temp -= target.eval_dyn_stat::<Dodge>(ctx);
					temp
				};

				Some(final_acc.into())
			}
			AccuracyMode::NeverMiss => None,
		}
	}

	pub fn calc_crit_chance(&self, caster: &Ptr<Actor>, ctx: &ActorContext) -> Option<IntPercent> {
		match self.crit_mode {
			CritMode::CanCrit { chance } => {
				let final_chance = {
					let mut temp = Int::from(chance);
					temp += caster.eval_dyn_stat::<CritRate>(ctx);
					temp
				};

				Some(final_chance.into())
			}
			CritMode::NeverCrit => None,
		}
	}
}

impl SkillData for LewdSkill {
	fn variant(&self) -> SkillIdent { self.skill_name }
	fn recovery_ms(&self) -> &Int { &self.recovery_ms }
	fn charge_ms(&self) -> &Int { &self.charge_ms }
	fn crit(&self) -> &CritMode { &self.crit_mode }
	fn effects_self(&self) -> &[CasterApplierEnum] { &self.effects_self }
	fn effects_target(&self) -> &[TargetApplierEnum] { &self.effects_target }
	fn caster_positions(&self) -> &PositionMatrix { &self.caster_positions }
	fn target_positions(&self) -> &PositionMatrix { &self.target_positions }
	fn multi_target(&self) -> &bool { &self.multi_target }
	fn use_counter(&self) -> &UseCounter { &self.use_counter }
}
