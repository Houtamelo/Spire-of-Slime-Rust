use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Arousal {
	pub duration_ms: Int,
	pub accumulated_ms: Int,
	pub lust_per_interval: Int,
}

impl IGirlStatusEffect for Arousal {
	fn duration_ms(&self) -> Int { self.duration_ms }
	fn set_duration(&mut self, ms: Int) { self.duration_ms = ms; }

	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> (CharacterTickResult, StatusTickResult) {
		const INTERVAL_MS: Int = int!(1000);

		let actual_ms = clamp_tick_ms(delta_ms, self.duration_ms);

		self.accumulated_ms += actual_ms;
		self.duration_ms -= actual_ms;

		let intervals_count = Int::from(self.accumulated_ms / INTERVAL_MS);
		if intervals_count > 0 {
			let lust_delta = {
				let mut temp = intervals_count;
				temp *= self.lust_per_interval;
				temp
			};

			self.accumulated_ms -= intervals_count * INTERVAL_MS;
			*girl.raw_stat_mut::<Lust>() += lust_delta;
		}

		let status_result = if self.duration_ms > 0 {
			StatusTickResult::Active
		} else {
			let lust_delta = {
				let mut temp = self.accumulated_ms;
				temp *= self.lust_per_interval;
				temp /= INTERVAL_MS;
				temp
			};

			*girl.raw_stat_mut::<Lust>() += lust_delta;
			StatusTickResult::Ended
		};

		(CharacterTickResult::Alive, status_result)
	}
}
