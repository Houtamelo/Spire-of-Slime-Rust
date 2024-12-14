use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistentHeal {
	pub duration_ms: Int,
	pub accumulated_ms: Int,
	pub heal_per_interval: Int,
}

impl IStatusEffect for PersistentHeal {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }

	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> (CharacterTickResult, StatusTickResult) {
		const INTERVAL_MS: Int = int!(1000);

		let actual_ms = clamp_tick_ms(delta_ms, self.duration_ms);

		self.accumulated_ms += actual_ms;
		self.duration_ms -= actual_ms;

		let intervals_count = self.accumulated_ms / INTERVAL_MS;
		let heal_amount = {
			let mut temp = intervals_count;

			temp *= self.heal_per_interval;
			temp
		};

		if intervals_count > 0 {
			self.accumulated_ms -= intervals_count * INTERVAL_MS;
			*actor.raw_stat_mut::<CurrentStamina>() += heal_amount;
		}

		let status_result = if *self.duration_ms() > 0 {
			StatusTickResult::Active
		} else {
			StatusTickResult::Ended
		};

		(CharacterTickResult::Alive, status_result)
	}
}
