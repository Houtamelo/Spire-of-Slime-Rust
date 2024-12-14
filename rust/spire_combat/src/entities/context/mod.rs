use serde::ser::SerializeMap;
pub use tick::*;

use super::*;

mod serialization;
mod tick;
use serialization::*; // non pub

#[derive(Clone, Serialize, Deserialize)]
pub struct ActorContext {
	#[serde(
		serialize_with = "serialize_actors",
		deserialize_with = "deserialize_actors"
	)]
	actors: HashMap<Id, Ptr<Actor>>,
	#[serde(
		serialize_with = "serialize_states",
		deserialize_with = "deserialize_states"
	)]
	left_states: IndexedMap<Id, Ptr<ActorState>>,
	#[serde(
		serialize_with = "serialize_states",
		deserialize_with = "deserialize_states"
	)]
	right_states: IndexedMap<Id, Ptr<ActorState>>,
	pub rng: Xoshiro256PlusPlus,
}

impl ActorContext {
	pub fn actor_ptr(&self, id: impl Into<Id>) -> Option<Ptr<Actor>> {
		self.actors.get(&id.into()).cloned()
	}

	pub fn actor_state_ptr(&self, id: impl Into<Id>) -> Option<Ptr<ActorState>> {
		let actor = self.actors.get(&id.into())?;
		match actor.team {
			Team::Left => &self.left_states,
			Team::Right => &self.right_states,
		}
		.get_value(&actor.id)
		.cloned()
	}

	pub fn actor_state(&self, id: impl Into<Id>) -> Option<&ActorState> {
		let actor = self.actors.get(&id.into())?;
		match actor.team {
			Team::Left => &self.left_states,
			Team::Right => &self.right_states,
		}
		.get_value(&actor.id)
		.map(|ptr| ptr.deref())
	}

	pub fn actor_state_mut(&mut self, id: impl Into<Id>) -> Option<&mut ActorState> {
		let actor = self.actors.get(&id.into())?;
		match actor.team {
			Team::Left => &mut self.left_states,
			Team::Right => &mut self.right_states,
		}
		.get_value_mut(&actor.id)
		.map(|ptr| ptr.deref_mut())
	}

	pub fn iter_actors_on_side(&self, side: Team) -> impl Iterator<Item = (&Ptr<Actor>, Position)> {
		self.actors.values().scan(0, move |order, actor| {
			if actor.team == side {
				let pos = *order;
				*order += actor.raw_stat::<Size>();
				Some((actor, pos.into()))
			} else {
				None
			}
		})
	}

	pub fn iter_actors_on_side_mut(
		&mut self,
		side: Team,
	) -> impl Iterator<Item = (&mut Ptr<Actor>, Position)> {
		self.actors.values_mut().scan(0, move |order, actor| {
			if actor.team == side {
				let pos = *order;
				*order += actor.raw_stat::<Size>();
				Some((actor, pos.into()))
			} else {
				None
			}
		})
	}

	pub fn iter_actors_on_side_except(
		&self,
		side: Team,
		exception: impl Into<Id>,
	) -> impl Iterator<Item = (&Ptr<Actor>, Position)> {
		let id = exception.into();
		self.iter_actors_on_side(side)
			.filter(move |(actor, _)| actor.id != id)
	}

	pub fn iter_actors_on_side_except_mut(
		&mut self,
		side: Team,
		exception: impl Into<Id>,
	) -> impl Iterator<Item = (&mut Ptr<Actor>, Position)> {
		let id = exception.into();
		self.iter_actors_on_side_mut(side)
			.filter(move |(actor, _)| actor.id != id)
	}

	pub fn iter_actors(&self) -> impl Iterator<Item = &Ptr<Actor>> { self.actors.values() }

	pub fn position_of(&self, id: impl Into<Id>) -> Option<Position> {
		let id = id.into();

		let side = self.actors.get(&id).cloned()?.team;
		let fighters = if side == Team::Left {
			&self.left_states
		} else {
			&self.right_states
		};

		let mut order = Int::from(0);
		for (&fighter_id, fighter) in fighters {
			if fighter_id == id {
				return Some(order.into());
			} else {
				let size = self
					.actors
					.get(&fighter_id)
					.cloned()
					.map(|actor| actor.raw_stat::<Size>())
					.unwrap_or_else(|| {
						godot_warn!(
							"Fighter with id {fighter_id:?} exists but its actor wasn't found in the map."
						);
						Size::from(0)
					});

				order += size;
			}
		}

		None
	}

	/// returns direction of y from x
	pub fn direction_if_adjacent(
		&self,
		origin: impl Into<Id>,
		relative: impl Into<Id>,
	) -> Option<Direction> {
		let (origin, relative) = (origin.into(), relative.into());

		return check_in(&self.left_states, origin, relative)
			.or_else(|| check_in(&self.right_states, origin, relative));

		fn check_in(
			fighters: &IndexedMap<Id, Ptr<ActorState>>,
			origin: Id,
			relative: Id,
		) -> Option<Direction> {
			for idx in 0..(fighters.len() - 1) {
				let &curr = fighters.key_at(idx).unwrap_or_else(|| {
					panic!("Index `{idx}` out of bounds despite checking, this is a bug.")
				});
				let &next = fighters.key_at(idx + 1).unwrap_or_else(|| {
					panic!("Index `{}` out of bounds despite checking, this is a bug.", idx + 1)
				});

				if curr == origin && next == relative {
					return Some(Direction::Back);
				} else if curr == relative && next == origin {
					return Some(Direction::Front);
				}
			}

			None
		}
	}

	pub fn state_set_or_push(&mut self, actor: &Ptr<Actor>, new_state: ActorState) {
		if let Some(state) = self.actor_state_mut(actor) {
			*state = new_state;
		} else {
			self.push_back_line(actor, new_state);
		}
	}

	pub fn push_front_line(&mut self, actor: &Ptr<Actor>, state: ActorState) {
		match actor.team {
			Team::Left => &mut self.left_states,
			Team::Right => &mut self.right_states,
		}
		.insert(actor.id, Ptr::new(state), 0);
	}

	pub fn push_back_line(&mut self, actor: &Ptr<Actor>, state: ActorState) {
		match actor.team {
			Team::Left => &mut self.left_states,
			Team::Right => &mut self.right_states,
		}
		.push(actor.id, Ptr::new(state));
	}

	pub fn move_actor(&mut self, id: impl Into<Id>, direction: MoveDirection) {
		let id = id.into();

		let Some(side) = self.actors.get(&id).map(|actor| actor.team)
		else { return godot_warn!("Actor with id {id:?} doesn't exist. Cannot move it.") };

		let states = match side {
			Team::Left => &mut self.left_states,
			Team::Right => &mut self.right_states,
		};

		let Some(index) = states.key_index(&id)
		else {
			return godot_warn!("Actor with id {id:?} doesn't have a state. Cannot move it.")
		};

		let target_index = match direction {
			MoveDirection::Front(amount) => {
				let mut temp = Int::from(index);
				temp -= amount;
				cram(temp)
			}
			MoveDirection::Back(amount) => {
				let mut temp = Int::from(index);
				temp += amount;
				usize::clamp_rg(temp, ..={ states.len() - 1 })
			}
		};

		let (_, state) = states.remove_at(index).unwrap_or_else(|| {
			panic!("Index `{index}` out of bounds but we checked it, this is a bug.")
		});

		states.insert(id, state, target_index);
	}

	pub fn delete_actor(&mut self, id: impl Into<Id>) {
		let id = id.into();
		self.left_states.remove(&id);
		self.right_states.remove(&id);
		self.actors.remove(&id);
	}

	pub fn release_grappled_victim(&mut self, victim: impl Into<Id>) {
		let id = victim.into();

		let Some(victim) = self.actor_ptr(id)
		else { return godot_warn!("Expected released victim to exist. Id: {id}") };

		let victim_state = ActorState::Downed {
			ticks: TrackedTicks::from_ms(2000),
		};

		self.push_front_line(&victim, victim_state);
	}

	pub fn handle_zero_stamina(&mut self, actr: Ptr<Actor>, killer_option: Option<Ptr<Actor>>) {
		// Perk::Ethel_LingeringToxins
		{
			let (ally_adjacent_center, ally_adjacent_edge) = self
				.iter_actors_on_side_except(actr.team, &actr)
				.fold((None, None), |(center, edge), (ally, ally_pos)| {
					match self.direction_if_adjacent(&actr, ally) {
						Some(Direction::Front) => {
							debug_assert!(center.is_none());
							(Some(ally), edge)
						}
						Some(Direction::Back) => {
							debug_assert!(edge.is_none());
							(center, Some(ally))
						}
						None => (center, edge),
					}
				});

			actr.statuses
				.values()
				.filter_map(|status| {
					if let StatusEffect::Poison(poison) = status
						&& poison.additives.contains(&PoisonAdditive::LingeringToxins)
					{
						Some(poison)
					} else {
						None
					}
				})
				.for_each(|poison| {
					let halved_poison = Poison {
						duration_ms: int!(poison.duration_ms / 2),
						accumulated_ms: int!(poison.accumulated_ms / 2),
						..poison.clone()
					};

					if let Some(mut ally) = ally_adjacent_center.cloned() {
						ally.add_status(halved_poison.clone());
					}

					if let Some(mut ally) = ally_adjacent_edge.cloned() {
						ally.add_status(halved_poison);
					}
				});
		}

		// OnKill effects
		if let Some(mut killer) = killer_option {
			if killer.has_perk::<Triumph>() {
				let speed_buff = BuffApplier {
					base_duration_ms: 3000.into(),
					stat: StatEnum::Speed,
					base_stat_increase: int!(25),
				};

				speed_buff.apply_on_caster(self, &mut killer, false);

				if let Some(mut girl) = killer.girl.clone() {
					*girl.raw_stat_mut::<Lust>() -= 10;
				}
			}

			if killer.has_perk::<Regret>()
				&& let Some(mut girl) = killer.girl.clone()
			{
				let composure_debuff = GirlDebuffApplier {
					base_duration_ms: int!(5000),
					base_apply_chance: None,
					stat: GirlStatEnum::Composure,
					base_stat_decrease: int!(15),
				};

				composure_debuff.apply_on_caster_girl(self, &mut killer, &mut girl, false);
			}
		}

		if let Some(ActorState::Grappling(GrapplingState {
			victim_id: victim, ..
		})) = self.actor_state(&actr)
		{
			self.release_grappled_victim(*victim);
		}

		match actr.on_zero_stamina {
			OnZeroStamina::Corpse => {
				self.state_set_or_push(&actr, ActorState::Defeated);
			}
			OnZeroStamina::Downed => {
				let ticks = TrackedTicks::from_ms(8000);
				self.state_set_or_push(&actr, ActorState::Downed { ticks });
			}
			OnZeroStamina::Vanish => self.delete_actor(&actr),
		}
	}
}
