#![allow(unused)] //todo!

use super::*;

#[derive(Clone)]
pub struct TimelineEvent {
	pub time_frame_ms:  Int,
	pub event_type:     EventType,
	pub character_guid: Uuid,
}

#[derive(Clone)]
pub enum EventType {
	TurnBegin,
	PoisonTick { poison: Int },
	LustTick { lust: Int },
	TemptationTick { temptation: Int },
	HealTick { heal: Int },
	StunEnd,
	DownedEnd,
	StatusEnd { effect_clone: StatusEffect },
	GrapplingEnd,
	SkillIntention { intention_clone: SkillIntention },
}

impl TimelineEvent {
	pub fn generate_events<T>(character: &ActorBase) -> Vec<TimelineEvent> {
		todo!()
		/*
		let mut events = Vec::new();

		let character_guid = character.guid;
		match &character.state {
			FighterState::Idle => {
				let event = TimelineEvent {
					time_frame_ms: 0.into(),
					event_type: EventType::TurnBegin,
					character_guid
				};
				events.push(event);
			}
			FighterState::Grappling(GrapplingState { victim, lust_per_interval, temptation_per_interval,
										  duration_ms, accumulated_ms }) => {
				const INTERVAL_MS: u64 = 1000;

				let event = TimelineEvent {
					time_frame_ms: *duration_ms,
					event_type: EventType::GrapplingEnd,
					character_guid
				};
				events.push(event);

				let total_time_ms = {
					let mut temp = duration_ms.into();
					temp += accumulated_ms;
					temp.squeeze_to_u64()
				};

				let total_intervals_count = total_time_ms / INTERVAL_MS;

				if total_intervals_count < 1 {
					let lust_option = {
						let mut temp = lust_per_interval.into();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::LustTick { lust },
							character_guid: victim.guid()
						};

						events.push(event);
					});

					let temptation_option = {
						let mut temp = temptation_per_interval.into();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};
					temptation_option.map(|temptation| {
						let event = TimelineEvent {
							time_frame_ms: *duration_ms,
							event_type: EventType::TemptationTick { temptation },
							character_guid: victim.guid()
						};

						events.push(event);
					});

					return events;
				}

				let mut current_ms = 0.into();

				if accumulated_ms < INTERVAL_MS {
					current_ms *= -1;
					current_ms *= accumulated_ms;
				} else {
					let interval_count = accumulated_ms / INTERVAL_MS;

					let lust_option = {
						let mut temp = lust_per_interval.into();
						temp *= interval_count;
						int!(temp.cram())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::LustTick { lust },
							character_guid: victim.guid()
						};

						events.push(event);
					});

					let temptation_option = {
						let mut temp = temptation_per_interval.into();
						temp *= interval_count;
						int!(temp.cram())
					};

					temptation_option.map(|temptation| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::TemptationTick { temptation },
							character_guid: victim.guid()
						};

						events.push(event);
					});

					let mult = {
						let mut temp = accumulated_ms.into();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp
					};
					current_ms *= mult;
				}

				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;
					let event = TimelineEvent {
						time_frame_ms: current_ms.into(),
						event_type: EventType::LustTick { lust: *lust_per_interval },
						character_guid
					};

					events.push(event);
				}

				let remaining_ms = {
					let mut temp = total_time_ms.into();
					temp -= total_intervals_count * INTERVAL_MS;
					temp.squeeze_to_u64()
				};

				if remaining_ms > 0 {
					current_ms += remaining_ms;

					let lust_option = {
						let mut temp = lust_per_interval.into();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};
					lust_option.map(|lust| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::LustTick { lust },
							character_guid,
						};

						events.push(event);
					});

					let temptation_option = {
						let mut temp = temptation_per_interval.into();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};
					temptation_option.map(|temptation| {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::TemptationTick { temptation },
							character_guid
						};

						events.push(event);
					});
				}
			}
			FighterState::Downed { ticks: TrackedTicks { remaining_ms, .. } } => {
				if remaining_ms > 0 {
					let event = TimelineEvent {
						time_frame_ms: *remaining_ms,
						event_type: EventType::DownedEnd,
						character_guid
					};

					events.push(event);
				}
			}
			FighterState::Stunned { ticks: stunned_ticks, state_before_stunned } => {
				if stunned_ticks.remaining_ms > 0 {
					let event = TimelineEvent {
						time_frame_ms: stunned_ticks.remaining_ms.into(),
						event_type: EventType::StunEnd,
						character_guid
					};

					events.push(event);
				}

				match state_before_stunned {
					StateBeforeStunned::Recovering { ticks: recovering_ticks } => {
						if recovering_ticks.remaining_ms > 0 {
							let time_frame_ms = {
								let mut temp = recovering_ticks.remaining_ms.into();
								temp += stunned_ticks.remaining_ms;
								temp.into()
							};

							let event = TimelineEvent {
								time_frame_ms,
								event_type: EventType::TurnBegin,
								character_guid
							};

							events.push(event);
						}
					},
					StateBeforeStunned::Charging { skill_intention } => {
						if skill_intention.charge_ticks.remaining_ms > 0 {
							let time_frame_ms = {
								let mut temp = skill_intention.charge_ticks.remaining_ms.into();
								temp += stunned_ticks.remaining_ms;
								temp.into()
							};

							let event = TimelineEvent {
								time_frame_ms,
								event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() },
								character_guid
							};

							events.push(event);
						}
					}
					StateBeforeStunned::Idle => {
						let event = TimelineEvent {
							time_frame_ms: stunned_ticks.remaining_ms.into(),
							event_type: EventType::TurnBegin,
							character_guid
						};

						events.push(event);
					}
				}
			},
			FighterState::Charging { skill_intention } => {
				let estimated_charge_ms = FighterState::calc_spd_charge_ms(
					skill_intention.charge_ticks.remaining_ms, character.dyn_stat::<Speed>());
				if estimated_charge_ms > 0 {
					let event = TimelineEvent {
						time_frame_ms: estimated_charge_ms,
						event_type: EventType::SkillIntention { intention_clone: skill_intention.clone() },
						character_guid
					};

					events.push(event);
				}
			}
			FighterState::Recovering { ticks } => {
				let estimated_recovery_ms = FighterState::calc_spd_recovery_ms(
					ticks.remaining_ms, character.dyn_stat::<Speed>());
				if estimated_recovery_ms > 0 {
					let event = TimelineEvent {
						time_frame_ms: estimated_recovery_ms,
						event_type: EventType::TurnBegin,
						character_guid
					};

					events.push(event);
				}
			}
		}

		character.status_effects.iter().for_each(|status|
			Self::register_status(status, character, &mut events));

		return events;
		*/
	}

	fn register_status<T>(
		status: &StatusEffect,
		owner: &ActorBase,
		events: &mut Vec<TimelineEvent>,
	) {
		todo!()
		/*
		let event_end_ms = status.duration_ms();
		if event_end_ms <= 0 {
			godot_warn!("{}(): Trying to register an event from status with negative duration: {:?}, duration: {:?}",
				full_fn_name(&Self::register_status), status, event_end_ms);
			return;
		}

		let event = TimelineEvent {
			time_frame_ms: event_end_ms,
			event_type: EventType::StatusEnd { effect_clone: status.clone() },
			character_guid: owner.guid
		};
		events.push(event);

		match status {
			| CommonStatusEnum::Buff(_) | CommonStatusEnum::Debuff(_)
			| CommonStatusEnum::Guarded(_) | CommonStatusEnum::Marked(_)
			| CommonStatusEnum::Riposte(_) => {}
			CommonStatusEnum::Poison(poison) => {
				let total_time_ms = {
					let mut temp = poison.duration_ms.into();
					temp += poison.accumulated_ms;
					temp.squeeze_to_u64()
				};

				let total_intervals_count = total_time_ms / poison.interval_ms;

				if total_intervals_count < 1 {
					let option = {
						let mut temp = poison.poison_per_interval.into();
						temp *= total_time_ms;
						temp /= poison.interval_ms;
						int!(temp.squeeze_to_u8())
					};

					if let Some(poison_dmg) = option {
						let event = TimelineEvent {
							time_frame_ms: poison.duration_ms,
							event_type: EventType::PoisonTick { poison: poison_dmg },
							character_guid: owner.guid
						};

						events.push(event);
					}

					return;
				}

				let mut current_ms =
					if poison.accumulated_ms < poison.interval_ms {
						let mut temp = poison.accumulated_ms.into();
						temp *= -1;
						temp
					} else {
						let interval_count = poison.accumulated_ms / poison.interval_ms;

						let option = {
							let mut temp = poison.poison_per_interval.into();
							temp *= interval_count;
							int!(temp.squeeze_to_u8())
						};

						if let Some(poison_dmg) = option {
							let event = TimelineEvent {
								time_frame_ms: 0.into(),
								event_type: EventType::PoisonTick { poison: poison_dmg },
								character_guid: owner.guid
							};

							events.push(event);
						}

						let mut temp = poison.accumulated_ms.into();
						temp -= interval_count * poison.interval_ms;
						temp *= -1;
						temp
					};

				for _ in 0..total_intervals_count {
					current_ms += poison.interval_ms;

					let event = TimelineEvent {
						time_frame_ms: current_ms.into(),
						event_type: EventType::PoisonTick { poison: poison.poison_per_interval },
						character_guid: owner.guid,
					};

					events.push(event);
				}

				let option = {
					let mut temp = total_time_ms.into();
					temp -= total_intervals_count * poison.interval_ms;
					int!(temp.cram())
				};

				if let Some(remaining_ms) = option {
					current_ms += remaining_ms;

					let option = {
						let mut temp = poison.poison_per_interval.into();
						temp *= remaining_ms;
						temp /= poison.interval_ms;
						int!(temp.cram())
					};

					if let Some(poison_dmg) = option {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::PoisonTick { poison: poison_dmg },
							character_guid: owner.guid
						};

						events.push(event);
					}
				}
			}
			CommonStatusEnum::Heal(heal) => {
				const INTERVAL_MS: u64 = 1000;

				let total_time_ms = heal.duration_ms + heal.accumulated_ms;
				let total_intervals_count = total_time_ms / INTERVAL_MS;

				if total_intervals_count < 1 {
					let option = {
						let mut temp = heal.heal_per_interval.into();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};

					if let Some(heal_amount) = option {
						let event = TimelineEvent {
							time_frame_ms: heal.duration_ms,
							event_type: EventType::HealTick { heal: heal_amount },
							character_guid: owner.guid
						};

						events.push(event);
					}

					return;
				}

				let mut current_ms =
					if heal.accumulated_ms < INTERVAL_MS {
						let mut temp = heal.accumulated_ms.into();
						temp *= -1;
						temp
					} else {
						let interval_count = heal.accumulated_ms / 1000;
						let option = {
							let mut temp = heal.heal_per_interval.into();
							temp *= interval_count;
							int!(temp.cram())
						};

						if let Some(heal_amount) = option {
							let event = TimelineEvent {
								time_frame_ms: 0.into(),
								event_type: EventType::HealTick { heal: heal_amount },
								character_guid: owner.guid
							};

							events.push(event);
						}

						let mut temp = heal.accumulated_ms.into();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp
					};

				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;

					let event = TimelineEvent {
						time_frame_ms: current_ms.into(),
						event_type: EventType::HealTick { heal: heal.heal_per_interval },
						character_guid: owner.guid
					};

					events.push(event);
				}

				let option = {
					let mut temp = total_time_ms.into();
					temp -= total_intervals_count * INTERVAL_MS;
					int!(temp.cram())
				};

				if let Some(remaining_ms) = option {
					current_ms += remaining_ms;

					let option = {
						let mut temp = heal.heal_per_interval.into();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};

					if let Some(heal_amount) = option {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::HealTick { heal: heal_amount },
							character_guid: owner.guid
						};

						events.push(event);
					}
				}
			}
			//todo!()
			/*
			CommonStatusEnum::Arousal(arousal) => {
				const INTERVAL_MS: u64 = 1000;

				let total_time_ms = arousal.duration_ms + arousal.accumulated_ms;
				let total_intervals_count = total_time_ms / INTERVAL_MS;

				if total_intervals_count < 1 {
					let option = {
						let mut temp = arousal.lust_per_interval.into();
						temp *= total_time_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};

					if let Some(lust) = option {
						let event = TimelineEvent {
							time_frame_ms: arousal.duration_ms,
							event_type: EventType::LustTick { lust },
							character_guid: owner.guid
						};

						events.push(event);
					}

					return;
				}

				let mut current_ms =
					if arousal.accumulated_ms < INTERVAL_MS {
						let mut temp = arousal.accumulated_ms.into();
						temp *= -1;
						temp
					} else {
						let interval_count = arousal.accumulated_ms / INTERVAL_MS;

						let option = {
							let mut temp = arousal.lust_per_interval.into();
							temp *= interval_count;
							int!(temp.cram())
						};

						if let Some(lust) = option {
							let event = TimelineEvent {
								time_frame_ms: 0.into(),
								event_type: EventType::LustTick { lust },
								character_guid: owner.guid
							};

							events.push(event);
						}

						let mut temp = arousal.accumulated_ms.into();
						temp -= interval_count * INTERVAL_MS;
						temp *= -1;
						temp
					};

				for _ in 0..total_intervals_count {
					current_ms += INTERVAL_MS;

					let event = TimelineEvent {
						time_frame_ms: current_ms.into(),
						event_type: EventType::LustTick { lust: arousal.lust_per_interval },
						character_guid: owner.guid
					};

					events.push(event);
				}

				// todo! This was incomplete last time I checked, check if it's still the case
				let option = {
					let mut temp = total_time_ms.into();
					temp -= total_intervals_count * INTERVAL_MS;
					int!(temp.cram())
				};

				if let Some(remaining_ms) = option {
					current_ms += remaining_ms;

					let option = {
						let mut temp = arousal.lust_per_interval.into();
						temp *= remaining_ms;
						temp /= INTERVAL_MS;
						int!(temp.cram())
					};

					if let Some(lust) = option {
						let event = TimelineEvent {
							time_frame_ms: current_ms.into(),
							event_type: EventType::LustTick { lust },
							character_guid: owner.guid
						};

						events.push(event);
					}
				}
			}
			*/
		}
		*/
	}
}
