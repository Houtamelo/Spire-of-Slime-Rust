use super::*;

#[must_use]
pub enum CharacterTickResult {
	Alive,
	Dead { killer: Id },
}

#[must_use]
pub enum StateTickResult {
	NoMajorChanges,
	Idle,
	CastSkill(SkillIntention),
	DownedEnded,
	StunEnded,
	GrapplingEnded { victim_id: Id },
}

#[must_use]
pub enum StatusTickResult {
	Active,
	Ended,
}

#[must_use]
pub enum PerkTickResult {
	Active,
	Ended,
}

impl ActorContext {
	// TODO: return results
	pub fn tick_actors(&mut self, delta_ms: Int) {
		let ids_to_tick = self
			.left_states
			.keys()
			.chain(self.right_states.keys())
			.cloned()
			.collect::<Vec<_>>();

		ids_to_tick.into_iter().for_each(|id| {
			let mut actor = self.actor_ptr(id)?;
			self.tick_actor(delta_ms, &mut actor);
		});
	}

	fn tick_actor(&mut self, delta_ms: Int, actor: &mut Ptr<Actor>) {
		self.tick_actor_perks(delta_ms, actor);

		if let Some(girl) = &mut actor.girl.clone() {
			self.tick_girl_perks(delta_ms, actor, girl);
		}

		// todo! Handle results
		let statuses_result = self.tick_actor_statuses(delta_ms, actor);

		if let Some(girl) = &mut actor.girl.clone() {
			let girl_statuses_result = self.tick_girl_statuses(delta_ms, actor, girl);
		}

		if let Some(mut state) = self.actor_state_ptr(&*actor) {
			let state_result = self.tick_actor_state(delta_ms, actor, &mut state);
		}
	}

	fn tick_actor_perks(&mut self, delta_ms: Int, actor: &mut Ptr<Actor>) {
		let ids_to_tick = actor.perks.keys().copied().collect::<Vec<_>>();

		for id in ids_to_tick {
			if let Some(mut perk) = actor.perks.remove(&id) {
				match perk.tick(actor, self, delta_ms) {
					PerkTickResult::Active => {
						actor.perks.insert(perk.id(), perk);
					}
					PerkTickResult::Ended => {}
				}
			} else {
				godot_warn!("Expected perk with id {id:?} to still be in perks map.")
			}
		}
	}

	fn tick_girl_perks(&mut self, delta_ms: Int, actor: &mut Ptr<Actor>, girl: &mut Ptr<Girl>) {
		let ids_to_tick = girl.perks.keys().copied().collect::<Vec<_>>();

		for id in ids_to_tick {
			if let Some(mut perk) = girl.perks.remove(&id) {
				match perk.tick(actor, girl, self, delta_ms) {
					PerkTickResult::Active => {
						girl.perks.insert(perk.id(), perk);
					}
					PerkTickResult::Ended => {}
				}
			} else {
				godot_warn!("Expected perk with id {id:?} to still be in perks map.")
			}
		}
	}

	fn tick_actor_statuses(
		&mut self,
		delta_ms: Int,
		actor: &mut Ptr<Actor>,
	) -> CharacterTickResult {
		let ids_to_tick = actor.statuses.keys().copied().collect::<Vec<_>>();

		for id in ids_to_tick {
			if let Some(mut eff) = actor.statuses.remove(&id) {
				let result = eff.tick(actor, self, delta_ms);
				match result {
					(CharacterTickResult::Dead { killer }, _) => {
						return CharacterTickResult::Dead { killer };
					}
					(CharacterTickResult::Alive, StatusTickResult::Active) => {
						actor.add_status(eff);
					}
					(CharacterTickResult::Alive, StatusTickResult::Ended) => {}
				}
			} else {
				godot_warn!("Expected status with id {id:?} to still be in statuses map.")
			}
		}

		CharacterTickResult::Alive
	}

	fn tick_girl_statuses(
		&mut self,
		delta_ms: Int,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
	) -> CharacterTickResult {
		let ids_to_tick = girl.statuses.keys().copied().collect::<Vec<_>>();

		for id in ids_to_tick {
			if let Some(mut eff) = girl.statuses.remove(&id) {
				let result = eff.tick(actor, girl, self, delta_ms);
				match result {
					(CharacterTickResult::Dead { killer }, _) => {
						return CharacterTickResult::Dead { killer };
					}
					(CharacterTickResult::Alive, StatusTickResult::Active) => {
						girl.add_status(eff);
					}
					(CharacterTickResult::Alive, StatusTickResult::Ended) => {}
				}
			} else {
				godot_warn!("Expected status with id {id:?} to still be in statuses map.")
			}
		}

		CharacterTickResult::Alive
	}

	fn tick_actor_state(
		&mut self,
		delta_ms: Int,
		actor: &mut Ptr<Actor>,
		state: &mut Ptr<ActorState>,
	) -> StateTickResult {
		match &mut **state {
			ActorState::Idle => {
				return StateTickResult::Idle;
			}
			ActorState::Downed { ticks } => {
				ticks.remaining_ms -= delta_ms;

				if ticks.remaining_ms <= 0 {
					**state = ActorState::Idle;

					let min_stamina = {
						let mut temp: Int = actor.eval_dyn_stat::<MaxStamina>(self).cram_into();
						temp.with_percent(50)
					};

					let curr_stamina = actor.raw_stat_mut::<CurrentStamina>();

					if curr_stamina < min_stamina {
						*curr_stamina = min_stamina.into();
					}

					return StateTickResult::DownedEnded;
				}
			}
			ActorState::Stunned {
				ticks,
				state_before_stunned,
			} => {
				ticks.remaining_ms -= delta_ms;

				if ticks.remaining_ms <= 0 {
					**state = match state_before_stunned.clone() {
						StateBeforeStunned::Recovering { ticks } => {
							ActorState::Recovering { ticks }
						}
						StateBeforeStunned::Charging { skill_intention } => {
							ActorState::Charging { skill_intention }
						}
						StateBeforeStunned::Idle => ActorState::Idle,
					};

					return StateTickResult::StunEnded;
				}
			}
			ActorState::Charging { skill_intention } => {
				let charge_ms =
					ActorState::calc_spd_charge_ms(delta_ms, actor.eval_dyn_stat::<Speed>(self));

				skill_intention.charge_ticks.remaining_ms -= charge_ms;

				if skill_intention.charge_ticks.remaining_ms <= 0 {
					let skill_intention = skill_intention.clone();

					**state = match skill_intention.recovery_after_complete {
						Some(ticks) => ActorState::Recovering { ticks },
						None => ActorState::Idle,
					};

					return StateTickResult::CastSkill(skill_intention);
				}
			}
			ActorState::Recovering { ticks } => {
				let recovery_ms =
					ActorState::calc_spd_recovery_ms(delta_ms, actor.eval_dyn_stat::<Speed>(self));

				ticks.remaining_ms -= recovery_ms;

				if ticks.remaining_ms <= 0 {
					**state = ActorState::Idle;
					return StateTickResult::Idle;
				}
			}
			ActorState::Grappling(GrapplingState {
				lust_per_interval,
				temptation_per_interval,
				duration_ms,
				accumulated_ms,
				victim_id,
				victim_defeated: is_defeated,
			}) => {
				let Some(mut victim) = self.actor_ptr(*victim_id)
				else {
					return StateTickResult::GrapplingEnded {
						victim_id: *victim_id,
					}
				};

				const INTERVAL_MS: Int = int!(1000);

				*duration_ms -= delta_ms;

				if duration_ms <= 0 {
					// duration is over so time to cum!
					let remaining_intervals =
						(*accumulated_ms + delta_ms - *duration_ms) / INTERVAL_MS;

					if let Some(girl) = &mut victim.girl {
						*girl.raw_stat_mut::<Lust>() += remaining_intervals * *lust_per_interval;
						*girl.raw_stat_mut::<Temptation>() +=
							remaining_intervals * *temptation_per_interval;

						self.release_grappled_victim(*victim_id);
					}

					return StateTickResult::GrapplingEnded {
						victim_id: *victim_id,
					};
				}

				*accumulated_ms += delta_ms;

				if accumulated_ms < INTERVAL_MS {
					return StateTickResult::NoMajorChanges;
				}

				if let Some(girl) = victim.girl.clone().as_mut() {
					let interval_count = *accumulated_ms / INTERVAL_MS;
					*accumulated_ms -= interval_count * INTERVAL_MS;

					*girl.raw_stat_mut::<Lust>() += interval_count * *lust_per_interval;
					*girl.raw_stat_mut::<Temptation>() += interval_count * *temptation_per_interval;

					if girl.raw_stat::<Lust>() >= Lust::MAX {
						girl.raw_stat_mut::<Lust>().set(0);

						let orgasm_limit = victim.eval_dyn_girl_stat::<OrgasmLimit>(girl, self);

						*girl.raw_stat_mut::<OrgasmCount>() += 1;
						*girl.raw_stat_mut::<Temptation>() -= 40;

						if girl.raw_stat::<OrgasmCount>() >= orgasm_limit {
							*is_defeated = true;
						}
					}
				}
			}
			ActorState::Defeated => {}
		}

		StateTickResult::NoMajorChanges
	}
}
