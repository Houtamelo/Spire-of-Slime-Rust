use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct DefensiveSkill {
	pub skill_name: SkillIdent,
	pub recovery_ms: Int,
	pub charge_ms: Int,
	pub crit_mode: CritMode,
	pub effects_caster: Vec<CasterApplierEnum>,
	pub effects_target: Vec<TargetApplierEnum>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub ally_requirement: AllyRequirement,
	pub multi_target: bool,
	pub use_counter: UseCounter,
}

impl DefensiveSkill {
	pub fn padding(&self) -> DefensivePadding { todo!() }

	pub fn final_crit_chance(&self, ctx: &ActorContext, caster: &Ptr<Actor>) -> Option<IntPercent> {
		match self.crit_mode {
			CritMode::CanCrit { chance } => {
				let final_chance = {
					let mut temp = chance;
					temp += caster.eval_dyn_stat::<CritRate>(ctx);
					temp
				};

				Some(final_chance.into())
			}
			CritMode::NeverCrit => None,
		}
	}

	pub fn cast(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>, target: &mut Ptr<Actor>) {
		self.handle_caster_effects_and_costs(ctx, caster);
		self.resolve_target(ctx, caster, target);

		if self.multi_target {
			let targets = ctx
				.iter_actors_on_side_except(target.team, target.id)
				.filter_map(|(entity, pos)| {
					pos.contains_any(entity.raw_stat::<Size>(), &self.target_positions)
						.then_some(entity.clone())
				})
				.collect::<Vec<_>>();

			for mut target in targets {
				self.resolve_target(ctx, caster, &mut target);
			}
		}
	}

	fn handle_caster_effects_and_costs(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>) {
		if self.recovery_ms > 0
			&& let Some(state) = ctx.actor_state_mut(caster.id)
		{
			let ticks = TrackedTicks::from_ms(self.recovery_ms);
			*state = ActorState::Recovering { ticks };
		}

		let is_crit = self
			.final_crit_chance(ctx, caster)
			.is_some_and(|chance| ctx.rng.base100_chance(chance));

		if is_crit && let Some(Vicious { stacks }) = caster.get_perk_mut() {
			*stacks -= 2;
		}

		for fx in &self.effects_caster {
			fx.apply(ctx, caster, is_crit);
		}
	}

	fn resolve_target(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
	) {
		let is_crit = self
			.final_crit_chance(ctx, caster)
			.is_some_and(|chance| ctx.rng.base100_chance(chance));

		if is_crit && let Some(Vicious { stacks }) = caster.get_perk_mut() {
			*stacks -= 2;
		}

		for fx in &self.effects_target {
			fx.apply(ctx, caster, target, is_crit);
		}

		if target.has_perk::<Grumpiness>() {
			Grumpiness::apply_status(ctx, target);
		}
	}
}

impl SkillData for DefensiveSkill {
	fn variant(&self) -> SkillIdent { self.skill_name }
	fn recovery_ms(&self) -> &Int { &self.recovery_ms }
	fn charge_ms(&self) -> &Int { &self.charge_ms }
	fn crit(&self) -> &CritMode { &self.crit_mode }
	fn effects_self(&self) -> &[CasterApplierEnum] { &self.effects_caster }
	fn effects_target(&self) -> &[TargetApplierEnum] { &self.effects_target }
	fn caster_positions(&self) -> &PositionMatrix { &self.caster_positions }
	fn target_positions(&self) -> &PositionMatrix { &self.target_positions }
	fn multi_target(&self) -> &bool { &self.multi_target }
	fn use_counter(&self) -> &UseCounter { &self.use_counter }
}
