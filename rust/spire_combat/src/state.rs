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
			self.tick(next_event.time_frame_ms);
		}
	}

	fn tick(&mut self, delta_time_ms: Int) {
		todo!()
		/*
		*self.elapsed_ms += delta_time_ms;

		let guids_to_tick: HashSet<Uuid> =
			self.entities.values().map(|entity| entity.guid).collect();

		guids_to_tick.iter().for_each(|guid| {
			match self.entities.remove(guid) {
				Some(mut entity) => {
					entity.tick_status_effects(&mut self.entities, &mut self.seed, delta_time_ms);
					self.entities.insert(*guid, entity);
				}
				None => {}
			}
		});

		guids_to_tick
			.into_iter()
			.for_each(|guid| {
				match self.entities.remove(&guid) {
					Some(_character) => {
						todo!()
						//tick_actor(character, &mut self.entities, delta_time_ms);
					}
					None => {
						godot_warn!("Warning: Trying to tick actors with guid {guid:?}, but it was not found in the left or right entities!");
					}
				}
			});

		return;
		*/
	}

	fn get_timeline_events(&self) -> Vec<TimelineEvent> {
		todo!()
		/*
		let mut events = self
			.entities
			.values()
			.filter_map(|entity| {
				if let Entity::Actor(character) = entity {
					Some(character)
				} else {
					None
				}
			})
			.flat_map(|character| TimelineEvent::generate_events(character))
			.collect::<Vec<_>>();

		events.sort_by(|a, b| {
			a.time_frame_ms.cmp(&b.time_frame_ms)
		});

		events
		*/
	}
}
