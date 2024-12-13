use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct OffensiveSkill {
	pub ident: SkillIdent,
	pub recovery_ms: Int,
	pub charge_ms: Int,
	pub can_be_riposted: bool,
	pub acc_mode: AccuracyMode,
	pub dmg_mode: DmgMode,
	pub crit_mode: CritMode,
	pub custom_modifiers: Vec<CustomOffensiveModifier>,
	pub effects_caster: Vec<CasterApplierEnum>,
	pub effects_target: Vec<TargetApplierEnum>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target: bool,
	pub use_counter: UseCounter,
}

// todo! This is used as data but the calculations don't use it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomOffensiveModifier {
	BonusVsMarked { power: i16, acc: i16, crit: i16 },
}

impl SkillData for OffensiveSkill {
	fn variant(&self) -> SkillIdent { self.ident }
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

use AttackResult::*;
use GrapplerResult::*;

#[must_use]
#[derive(Debug, Copy, Clone)]
pub enum GrapplerResult {
	GrappleContinues,
	VictimReleased { victim_id: Id },
	GrapplerDied { victim_id: Id },
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct OffensiveResult {
	pub attack:  AttackResult,
	pub counter: Option<AttackResult>,
}

impl OffensiveResult {
	pub fn caster_died(&self) -> bool {
		matches!(self.counter, Some(Hit { lethal: true } | HitGrappler(GrapplerDied { .. })))
	}

	pub fn target_died(&self) -> bool {
		matches!(self.attack, Hit { lethal: true } | HitGrappler(GrapplerDied { .. }))
	}
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub enum AttackResult {
	Hit { lethal: bool },
	HitGrappler(GrapplerResult),
	Miss,
}

impl OffensiveSkill {
	pub fn cast(
		mut self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
	) -> Vec<(Ptr<Actor>, OffensiveResult)> {
		use EthelSkill::*;

		if caster.has_perk::<PoisonCoating>()
			&& matches!(self.variant(), SkillIdent::Ethel(Clash | Pierce | Sever))
		{
			self.effects_target
				.push(target_fx!(PoisonApplier::from_caster(caster, 3000, 1, Some(100))));
		} else if caster.has_perk::<AlluringChallenger>()
			&& matches!(self.variant(), SkillIdent::Ethel(Challenge))
		{
			self.effects_caster.push(caster_fx!(MarkApplier {
				base_duration_ms: 4000.into(),
			}));
		}

		self.handle_caster_effects_and_costs(ctx, caster);

		let mut results = Vec::new();

		results.push((target.clone(), self.resolve(ctx, caster, target)));

		if self.multi_target {
			let targets = ctx
				.iter_actors_on_side_except(target.team, target.id)
				.filter_map(|(actor, pos)| {
					pos.contains_any(actor.raw_stat::<Size>(), &self.target_positions)
						.then(|| actor.clone())
				})
				.collect::<Vec<_>>();

			for mut secondary_target in targets {
				results.push((
					secondary_target.clone(),
					self.resolve(ctx, caster, &mut secondary_target),
				));
			}
		}

		results
	}

	fn handle_caster_effects_and_costs(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>) {
		if self.recovery_ms > 0
			&& let Some(state) = ctx.actor_state_mut(caster.id)
		{
			*state = ActorState::Recovering {
				ticks: TrackedTicks::from_ms(self.recovery_ms),
			};
		}

		caster.increment_skill_counter(self.ident);

		let is_crit = self.crit_mode.eval_did_crit(ctx, caster);

		if is_crit && let Some(Vicious { stacks }) = caster.get_perk_mut() {
			*stacks -= 2;
		}

		for fx in &self.effects_caster {
			fx.apply(ctx, caster, is_crit);
		}
	}

	fn resolve(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
	) -> OffensiveResult {
		if !self.acc_mode.eval_did_hit(ctx, caster, target) {
			if let Some(Relentless { stacks }) = caster.get_perk_mut() {
				stacks.set(0);
			}

			if let Some(Grudge { active }) = caster.get_perk_mut() {
				*active = true;
			}

			let counter = self.check_counter(ctx, caster, target);
			return OffensiveResult {
				attack: Miss,
				counter,
			};
		}

		// On-Hit Perks
		if let Some(Vicious { stacks }) = caster.get_perk_mut() {
			*stacks += 1;
		}

		if let Some(EnragingPain { stacks }) = target.get_perk_mut() {
			*stacks += 1;
		}

		if target.has_perk::<Release>()
			&& let Some(target_girl) = &mut target.girl
		{
			*target_girl.raw_stat_mut::<Lust>() += 2;
		}

		if target.has_perk::<AlluringScent>()
			&& let Some(caster_girl) = &mut caster.girl
		{
			*caster_girl.raw_stat_mut::<Lust>() += 12;
		}

		if target.has_perk::<Grumpiness>() {
			Grumpiness::apply_status(ctx, target);
		}

		let is_crit = self
			.crit_mode
			.eval_did_crit(ctx, &caster)
			||
			// crit or bold perk, landing a natural crit doesn't spend `Bold`
			if let Some(Bold { used: used @ false }) = caster.get_perk_mut() {
				*used = true;
				true
			} else {
				false
			};

		if is_crit {
			if let Some(Vicious { stacks }) = caster.get_perk_mut() {
				*stacks -= 2;
			}

			if caster.has_perk::<StaggeringForce>() {
				const STAGGERING_FORCE: DebuffApplier = DebuffApplier {
					base_duration_ms: int!(4000),
					base_apply_chance: Some(int!(100)),
					applier_kind: DebuffApplierKind::StaggeringForce,
				};

				STAGGERING_FORCE.apply_on_target(ctx, caster, target, false);
			}
		}

		for fx in &self.effects_target {
			fx.apply(ctx, caster, target, is_crit);
		}

		let dmg = match self.resolve_damage(ctx, caster, target, is_crit) {
			Some(non_zero) => non_zero,
			None => {
				let counter = self.check_counter(ctx, caster, target);
				return OffensiveResult {
					attack: Hit { lethal: false },
					counter,
				};
			}
		};

		if let Some(Relentless { stacks }) = caster.get_perk_mut() {
			*stacks += 1;
			*caster.raw_stat_mut::<CurrentStamina>() += dmg.with_percent(30);
		}

		if let Some(Trust { accumulated_ms }) = target.get_perk_mut() {
			accumulated_ms.set(0);
		}

		if let Some(Hatred { stacks }) = target.get_perk_mut() {
			*stacks += 1;
		}

		target.last_damager = Some(caster.id);

		let state = ctx.actor_state_ptr(target.id);
		match state {
			Some(mut state) => {
				match &*state {
					| ActorState::Idle
					| ActorState::Charging { .. }
					| ActorState::Recovering { .. } => {
						*target.raw_stat_mut::<CurrentStamina>() -= dmg;

						// If the attack kills, the target can't counter.
						if target.stamina_alive() {
							let counter = self.check_counter(ctx, caster, target);
							OffensiveResult {
								attack: Hit { lethal: false },
								counter,
							}
						} else {
							OffensiveResult {
								attack:  Hit { lethal: true },
								counter: None,
							}
						}
					}

					| ActorState::Downed { .. }
					| ActorState::Stunned { .. }
					| ActorState::Defeated => {
						*target.raw_stat_mut::<CurrentStamina>() -= dmg;

						let lethal = !target.stamina_alive();
						OffensiveResult {
							attack:  Hit { lethal },
							counter: None,
						}
					}

					ActorState::Grappling(grappling) => {
						let old_stamina_percent = {
							let mut temp = target.raw_stat::<CurrentStamina>();
							temp *= 100;
							temp /= target.eval_dyn_stat::<MaxStamina>(ctx);
							*temp
						};

						*target.raw_stat_mut::<CurrentStamina>() -= dmg;

						let new_stamina_percent = {
							let mut temp = target.raw_stat::<CurrentStamina>();
							temp *= 100;
							temp /= target.eval_dyn_stat::<MaxStamina>(ctx);
							*temp
						};

						if target.stamina_dead() {
							OffensiveResult {
								attack:  HitGrappler(GrapplerDied {
									victim_id: grappling.victim_id,
								}),
								counter: None,
							}
						}
						// If a single blow deals more than 25% of the grappler's max_health, release the victim
						else if (old_stamina_percent - new_stamina_percent) >= 25 {
							OffensiveResult {
								attack:  HitGrappler(VictimReleased {
									victim_id: grappling.victim_id,
								}),
								counter: None,
							}
						} else {
							OffensiveResult {
								attack:  HitGrappler(GrappleContinues),
								counter: None,
							}
						}
					}
				}
			}
			_ => {
				*target.raw_stat_mut::<CurrentStamina>() -= dmg;

				// If the first attack kills, the target can't counter.
				if target.stamina_alive() {
					let counter = self.check_counter(ctx, caster, target);
					OffensiveResult {
						attack: Hit { lethal: false },
						counter,
					}
				} else {
					OffensiveResult {
						attack:  Hit { lethal: true },
						counter: None,
					}
				}
			}
		}
	}

	#[must_use]
	fn check_counter(
		&self,
		ctx: &mut ActorContext,
		riposter: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
	) -> Option<AttackResult> {
		if !self.can_be_riposted {
			return None;
		}

		let Riposte {
			skill_power,
			acc_mode,
			crit_mode,
			..
		} = riposter.get_status::<Riposte>().cloned()?;

		if !acc_mode.eval_did_hit(ctx, riposter, target) {
			return Some(Miss);
		}

		if riposter.has_perk::<Release>()
			&& let Some(girl) = &mut riposter.girl
		{
			let lust = girl.raw_stat_mut::<Lust>();
			if lust > 100 {
				*lust -= 4;
			}
		}

		if let Some(Vicious { stacks }) = riposter.get_perk_mut() {
			*stacks += 1;
		}

		let is_crit = crit_mode.eval_did_crit(ctx, riposter);

		if is_crit && let Some(Vicious { stacks }) = riposter.get_perk_mut() {
			*stacks -= 2;
		}

		let dmg_mode = DmgMode::Power {
			power: skill_power,
			toughness_reduction: riposter.eval_dyn_stat::<ToughnessReduction>(ctx),
		};

		let lethal = if let Some(dmg) = dmg_mode.eval_did_dmg(ctx, riposter, target, is_crit) {
			*target.raw_stat_mut::<CurrentStamina>() -= dmg;
			target.last_damager = Some(riposter.id);

			target.stamina_dead()
		} else {
			false
		};

		Some(Hit { lethal })
	}

	fn resolve_damage(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) -> Option<Int> {
		use EthelSkill::*;

		let mut dmg = self.dmg_mode.eval_did_dmg(ctx, caster, target, is_crit)?;

		if caster.has_perk::<FocusedSwings>()
			&& matches!(self.variant(), SkillIdent::Ethel(Clash | Sever))
		{
			const TOUGHNESS_DEBUFF: DebuffApplier = DebuffApplier {
				base_duration_ms: int!(4000),
				base_apply_chance: Some(int!(100)),
				applier_kind: DebuffApplierKind::Standard {
					stat: StatEnum::Toughness,
					base_stat_decrease: int!(25),
				},
			};

			TOUGHNESS_DEBUFF.apply_on_target(ctx, caster, target, is_crit);
		}

		if caster.has_perk::<GoForTheEyes>() {
			dmg.set_percent(90);

			let random_debuff = DebuffApplier {
				base_duration_ms: int!(4000),
				base_apply_chance: Some(int!(100)),
				applier_kind: DebuffApplierKind::Standard {
					stat: StatEnum::get_random(&mut ctx.rng),
					base_stat_decrease: int!(10),
				},
			};

			random_debuff.apply_on_target(ctx, caster, target, false);
		}

		if target.has_perk::<UnnervingAura>() && caster.has_status::<Debuff>() {
			dmg.set_percent(75);
		}

		if let Some(Grudge {
			active: active @ true,
		}) = caster.get_perk_mut()
		{
			*active = false;
			dmg.set_percent(130);
		}

		if caster.has_perk::<NoQuarters>() {
			let debuff_count = target
				.statuses
				.values()
				.filter(|effect| effect.is::<Debuff>())
				.count();

			let dmg_mod = {
				let mut temp = Int::clamp_rg(debuff_count * 50, 0..=250);

				if let Some(ActorState::Stunned { .. }) = ctx.actor_state(target.id) {
					temp += 100;
				}

				temp /= 10;
				temp
			};

			dmg.set_percent(100 + dmg_mod);
		}

		if let Some(Hatred { stacks }) = caster.get_perk_mut() {
			dmg.set_percent(100 + 15 * *stacks);
			stacks.set(0);
		}

		Some(dmg)
	}

	/*
	enum Continue {
		Yes,
		No,
	}

	fn handle_result(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		result: AttackResult,
	) -> Continue {
		fn on_death(deceased: Ptr<Actor>, ctx: &mut ActorContext) { todo!() }

		match result {
			| AttackResult::Missed { riposte } | AttackResult::SurvivedHit { riposte } => {
				match riposte {
					Some(RiposteResult::DiedFromHit) => {
						on_death(caster, ctx);
						Continue::No
					}

					| Some(RiposteResult::SurvivedHit | RiposteResult::Missed) | None => Continue::Yes,
				}
			}
			AttackResult::DiedFromHit => {
				on_death(target, ctx);
				Continue::Yes
			}
			AttackResult::GrapplerReleasedVictim { victim } => {
				let state = ActorState::insert_freed_victim(victim, ctx);
				ctx.insert(grappler.guid, grappler);
				Continue::Yes(caster)
			}
			AttackResult::GrapplerDied { victim } => {
				insert_freed_victim(victim, ctx);
				on_death(grappler.into(), ctx);
				Continue::Yes(caster)
			}
		}
	}
	*/
}
