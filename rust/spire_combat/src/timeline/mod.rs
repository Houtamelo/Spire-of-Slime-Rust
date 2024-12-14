use super::*;

#[derive(Clone)]
pub struct TimelineEvent {
	pub time_frame_ms: i64,
	pub event_type: EventType,
	pub actor_id: Id,
}

#[derive(Clone)]
pub enum EventType {
	TurnBegin,
	PoisonTick { poison: i64 },
	LustTick { lust: i64 },
	TemptationTick { temptation: i64 },
	HealTick { heal: i64 },
	StunEnd,
	DownedEnd,
	StatusEnd { effect_clone: StatusEffect },
	GrapplingEnd,
	SkillIntention { intention_clone: SkillIntention },
}

impl TimelineEvent {
	pub fn generate_events(ctx: &ActorContext, actr: &Actor) -> Vec<TimelineEvent> {
		let mut events = Vec::new();
		let actor_id = actr.id;

		if let Some(state) = ctx.actor_state(actr.id) {
			match state {
				ActorState::Idle => {
					events.push(TimelineEvent {
						time_frame_ms: 0,
						event_type: EventType::TurnBegin,
						actor_id,
					});
				}
				ActorState::Grappling(GrapplingState {
					lust_per_interval,
					temptation_per_interval,
					duration_ms,
					accumulated_ms,
					victim_id,
					victim_defeated,
				}) => {
					const INTERVAL_MS: i64 = 1000;

					events.push(TimelineEvent {
						time_frame_ms: **duration_ms,
						event_type: EventType::GrapplingEnd,
						actor_id,
					});

					let total_time_ms = duration_ms + accumulated_ms;
					let total_intervals_count = total_time_ms / INTERVAL_MS;
					if total_intervals_count < 1 {
						let lust = (lust_per_interval * total_time_ms) / INTERVAL_MS;
						if lust != 0 {
							events.push(TimelineEvent {
								time_frame_ms: **duration_ms,
								event_type: EventType::LustTick { lust },
								actor_id: *victim_id,
							});
						}

						let temptation = (temptation_per_interval * total_time_ms) / INTERVAL_MS;
						if temptation != 0 {
							events.push(TimelineEvent {
								time_frame_ms: **duration_ms,
								event_type: EventType::TemptationTick { temptation },
								actor_id: *victim_id,
							});
						}
					} else {
						let mut current_ms = int!(0);

						if accumulated_ms < INTERVAL_MS {
							current_ms *= -1 * accumulated_ms;
						} else {
							let interval_count = accumulated_ms / INTERVAL_MS;

							let lust = lust_per_interval * interval_count;
							if lust != 0 {
								events.push(TimelineEvent {
									time_frame_ms: *current_ms,
									event_type: EventType::LustTick { lust },
									actor_id: *victim_id,
								});
							}

							let temptation = temptation_per_interval * interval_count;
							if temptation != 0 {
								events.push(TimelineEvent {
									time_frame_ms: *current_ms,
									event_type: EventType::TemptationTick { temptation },
									actor_id: *victim_id,
								});
							}

							current_ms *= -1 * (accumulated_ms - (interval_count * INTERVAL_MS));
						}

						for _ in 0..total_intervals_count {
							current_ms += INTERVAL_MS;

							events.push(TimelineEvent {
								time_frame_ms: *current_ms,
								event_type: EventType::LustTick {
									lust: **lust_per_interval,
								},
								actor_id,
							});
						}

						let remaining_ms = total_time_ms - (total_intervals_count * INTERVAL_MS);
						if remaining_ms > 0 {
							current_ms += remaining_ms;

							let lust = (lust_per_interval * remaining_ms) / INTERVAL_MS;
							if lust != 0 {
								events.push(TimelineEvent {
									time_frame_ms: *current_ms,
									event_type: EventType::LustTick { lust },
									actor_id,
								});
							}

							let temptation = (temptation_per_interval * remaining_ms) / INTERVAL_MS;
							if temptation != 0 {
								events.push(TimelineEvent {
									time_frame_ms: *current_ms,
									event_type: EventType::TemptationTick { temptation },
									actor_id,
								});
							}
						}
					}
				}
				ActorState::Downed {
					ticks: TrackedTicks { remaining_ms, .. },
				} => {
					if remaining_ms > 0 {
						events.push(TimelineEvent {
							time_frame_ms: **remaining_ms,
							event_type: EventType::DownedEnd,
							actor_id,
						});
					}
				}
				ActorState::Stunned {
					ticks: stunned_ticks,
					state_before_stunned,
				} => {
					if stunned_ticks.remaining_ms > 0 {
						events.push(TimelineEvent {
							time_frame_ms: *stunned_ticks.remaining_ms,
							event_type: EventType::StunEnd,
							actor_id,
						});
					}

					match state_before_stunned {
						StateBeforeStunned::Recovering {
							ticks: recovering_ticks,
						} => {
							if recovering_ticks.remaining_ms > 0 {
								let time_frame_ms =
									recovering_ticks.remaining_ms + stunned_ticks.remaining_ms;
								events.push(TimelineEvent {
									time_frame_ms,
									event_type: EventType::TurnBegin,
									actor_id,
								});
							}
						}
						StateBeforeStunned::Charging { skill_intention } => {
							if skill_intention.charge_ticks.remaining_ms > 0 {
								let time_frame_ms = skill_intention.charge_ticks.remaining_ms
									+ stunned_ticks.remaining_ms;

								events.push(TimelineEvent {
									time_frame_ms,
									event_type: EventType::SkillIntention {
										intention_clone: skill_intention.clone(),
									},
									actor_id,
								});
							}
						}
						StateBeforeStunned::Idle => {
							events.push(TimelineEvent {
								time_frame_ms: *stunned_ticks.remaining_ms,
								event_type: EventType::TurnBegin,
								actor_id,
							});
						}
					}
				}
				ActorState::Charging { skill_intention } => {
					let estimated_charge_ms = ActorState::calc_spd_charge_ms(
						skill_intention.charge_ticks.remaining_ms,
						actr.eval_dyn_stat::<Speed>(ctx),
					);
					if estimated_charge_ms > 0 {
						events.push(TimelineEvent {
							time_frame_ms: *estimated_charge_ms,
							event_type: EventType::SkillIntention {
								intention_clone: skill_intention.clone(),
							},
							actor_id,
						});
					}
				}
				ActorState::Recovering { ticks } => {
					let estimated_recovery_ms = ActorState::calc_spd_recovery_ms(
						ticks.remaining_ms,
						actr.eval_dyn_stat::<Speed>(ctx),
					);
					if estimated_recovery_ms > 0 {
						events.push(TimelineEvent {
							time_frame_ms: *estimated_recovery_ms,
							event_type: EventType::TurnBegin,
							actor_id,
						});
					}
				}
				ActorState::Defeated => {}
			}
		}

		for status in actr.statuses.values() {
			Self::register_status(status, actr, &mut events);
		}

		return events;
	}

	fn register_status(status: &StatusEffect, actr: &Actor, events: &mut Vec<TimelineEvent>) {
		let event_end_ms = status.duration_ms();
		if event_end_ms <= 0 {
			godot_warn!(
				"{}(): Trying to register an event from status with negative duration: {:?}, duration: {:?}",
				full_fn_name(&Self::register_status),
				status,
				event_end_ms
			);
			return;
		}

		let actor_id = actr.id;

		events.push(TimelineEvent {
			time_frame_ms: *event_end_ms,
			event_type: EventType::StatusEnd {
				effect_clone: status.clone(),
			},
			actor_id,
		});

		match status {
			| StatusEffect::Buff(_)
			| StatusEffect::Debuff(_)
			| StatusEffect::Guarded(_)
			| StatusEffect::Mark(_)
			| StatusEffect::Riposte(_) => {}

			StatusEffect::Poison(poison) => {
				let total_time_ms = poison.duration_ms + poison.accumulated_ms;
				let total_intervals_count = total_time_ms / poison.interval_ms;
				if total_intervals_count < 1 {
					let poison_dmg =
						(poison.poison_per_interval * total_time_ms) / poison.interval_ms;
					if poison_dmg > 0 {
						events.push(TimelineEvent {
							time_frame_ms: *poison.duration_ms,
							event_type: EventType::PoisonTick { poison: poison_dmg },
							actor_id,
						});
					}
				} else {
					let mut current_ms: Int = if poison.accumulated_ms < poison.interval_ms {
						-1 * poison.accumulated_ms
					} else {
						let interval_count = poison.accumulated_ms / poison.interval_ms;

						let poison_dmg = poison.poison_per_interval * interval_count;
						if poison_dmg > 0 {
							events.push(TimelineEvent {
								time_frame_ms: 0,
								event_type: EventType::PoisonTick { poison: poison_dmg },
								actor_id,
							});
						}

						-1 * (poison.accumulated_ms - (interval_count * poison.interval_ms))
					}
					.cram_into();

					for _ in 0..total_intervals_count {
						current_ms += poison.interval_ms;

						events.push(TimelineEvent {
							time_frame_ms: *current_ms,
							event_type: EventType::PoisonTick {
								poison: *poison.poison_per_interval,
							},
							actor_id,
						});
					}

					let remaining_ms = total_time_ms - (total_intervals_count * poison.interval_ms);
					if remaining_ms > 0 {
						current_ms += remaining_ms;

						let poison_dmg =
							(poison.poison_per_interval * remaining_ms) / poison.interval_ms;
						if poison_dmg > 0 {
							events.push(TimelineEvent {
								time_frame_ms: *current_ms,
								event_type: EventType::PoisonTick { poison: poison_dmg },
								actor_id,
							});
						}
					}
				}
			}
			StatusEffect::PersistentHeal(heal) => {
				const INTERVAL_MS: i64 = 1000;

				let total_time_ms = heal.duration_ms + heal.accumulated_ms;
				let total_intervals_count = total_time_ms / INTERVAL_MS;
				if total_intervals_count < 1 {
					let heal_amount = (heal.heal_per_interval * total_time_ms) / INTERVAL_MS;
					if heal_amount > 0 {
						events.push(TimelineEvent {
							time_frame_ms: *heal.duration_ms,
							event_type: EventType::HealTick { heal: heal_amount },
							actor_id,
						});
					}
				} else {
					let mut current_ms: Int = if heal.accumulated_ms < INTERVAL_MS {
						-1 * heal.accumulated_ms
					} else {
						let interval_count = heal.accumulated_ms / 1000;

						let heal_amount = heal.heal_per_interval * interval_count;
						if heal_amount > 0 {
							events.push(TimelineEvent {
								time_frame_ms: 0,
								event_type: EventType::HealTick { heal: heal_amount },
								actor_id,
							});
						}

						-1 * (heal.accumulated_ms - (interval_count * INTERVAL_MS))
					}
					.cram_into();

					for _ in 0..total_intervals_count {
						current_ms += INTERVAL_MS;

						events.push(TimelineEvent {
							time_frame_ms: *current_ms,
							event_type: EventType::HealTick {
								heal: *heal.heal_per_interval,
							},
							actor_id,
						});
					}

					let remaining_ms = total_time_ms - (total_intervals_count * INTERVAL_MS);
					if remaining_ms > 0 {
						current_ms += remaining_ms;

						let heal_amount = (heal.heal_per_interval * remaining_ms) / INTERVAL_MS;
						if heal_amount > 0 {
							events.push(TimelineEvent {
								time_frame_ms: *current_ms,
								event_type: EventType::HealTick { heal: heal_amount },
								actor_id,
							});
						}
					}
				}
			}
		}
	}

	fn register_girl_status(status: &GirlStatus, actr: &Actor, events: &mut Vec<TimelineEvent>) {
		let actor_id = actr.id;

		match status {
			| GirlStatus::Buff(_) | GirlStatus::Debuff(_) => {}

			GirlStatus::Arousal(arousal) => {
				const INTERVAL_MS: i64 = 1000;

				let total_time_ms = arousal.duration_ms + arousal.accumulated_ms;
				let total_intervals_count = total_time_ms / INTERVAL_MS;
				if total_intervals_count < 1 {
					let lust = (arousal.lust_per_interval * total_time_ms) / INTERVAL_MS;
					if lust != 0 {
						events.push(TimelineEvent {
							time_frame_ms: *arousal.duration_ms,
							event_type: EventType::LustTick { lust },
							actor_id,
						});
					}
				} else {
					let mut current_ms: Int = if arousal.accumulated_ms < INTERVAL_MS {
						-1 * arousal.accumulated_ms
					} else {
						let interval_count = arousal.accumulated_ms / INTERVAL_MS;

						let lust = arousal.lust_per_interval * interval_count;
						if lust != 0 {
							events.push(TimelineEvent {
								time_frame_ms: 0,
								event_type: EventType::LustTick { lust },
								actor_id,
							});
						}

						-1 * (arousal.accumulated_ms - (interval_count * INTERVAL_MS))
					}
					.cram_into();

					for _ in 0..total_intervals_count {
						current_ms += INTERVAL_MS;

						events.push(TimelineEvent {
							time_frame_ms: *current_ms,
							event_type: EventType::LustTick {
								lust: *arousal.lust_per_interval,
							},
							actor_id,
						});
					}

					let remaining_ms = total_time_ms - (total_intervals_count * INTERVAL_MS);
					if remaining_ms > 0 {
						current_ms += remaining_ms;

						let lust = (arousal.lust_per_interval * remaining_ms) / INTERVAL_MS;
						if lust != 0 {
							events.push(TimelineEvent {
								time_frame_ms: *current_ms,
								event_type: EventType::LustTick { lust },
								actor_id,
							});
						}
					}
				}
			}
		}
	}
}
