use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Poison {
	pub duration_ms: Int,
	pub accumulated_ms: Int,
	pub interval_ms: Int,
	pub poison_per_interval: Int,
	pub additives: HashSet<PoisonAdditive>,
	pub caster: Id,
}

impl IStatusEffect for Poison {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }

	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> (CharacterTickResult, StatusTickResult) {
		let actual_ms = clamp_tick_ms(delta_ms, self.duration_ms);

		self.accumulated_ms += actual_ms;
		self.duration_ms -= actual_ms;

		let intervals_count = Int::from(self.accumulated_ms / self.interval_ms);

		if intervals_count > 0 {
			let dmg = {
				let mut temp = intervals_count;
				temp *= self.poison_per_interval;
				temp
			};

			self.accumulated_ms -= intervals_count * self.interval_ms;
			*actor.raw_stat_mut::<CurrentStamina>() -= dmg;
		}

		let status_result = if self.duration_ms > 0 {
			StatusTickResult::Active
		} else {
			let partial_interval_dmg = {
				let mut temp = self.accumulated_ms;
				temp += intervals_count * self.interval_ms;
				temp *= self.poison_per_interval;
				temp /= self.interval_ms;
				temp
			};

			*actor.raw_stat_mut::<CurrentStamina>() -= partial_interval_dmg;
			StatusTickResult::Ended
		};

		let character_result = if actor.stamina_alive() {
			CharacterTickResult::Alive
		} else {
			CharacterTickResult::Dead {
				killer: self.caster,
			}
		};

		(character_result, status_result)
	}
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub enum PoisonAdditive {
	LingeringToxins,
	ParalyzingToxins,
	ConcentratedToxins,
	Madness,
}
