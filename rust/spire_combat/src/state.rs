#![allow(clippy::absurd_extreme_comparisons)]
use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct CombatState {
	ctx: ActorContext,
	elapsed_ms: Int,
}

impl CombatState {
	pub fn run(&mut self) {
		let events = self.get_timeline_events();
		if !events.is_empty() {
			let next_event = &events[0];
			self.tick(next_event.time_frame_ms.cram_into());
		}
	}

	fn tick(&mut self, delta_ms: Int) {
		self.elapsed_ms += delta_ms;

		// TODO: handle results
		self.ctx.tick_actors(delta_ms);
	}

	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		let mut events = self
			.ctx
			.iter_actors()
			.flat_map(|character| TimelineEvent::generate_events(&self.ctx, &**character))
			.collect::<Vec<_>>();

		events.sort_by_cached_key(|event| event.time_frame_ms);
		events
	}
}
