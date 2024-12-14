mod types;

pub use types::*;

use super::*;

delegated_enum! {
	ENUM_OUT: {
	#[derive(Serialize, Deserialize, Clone, Debug)]
		pub enum StatusEffect {
			Buff(Buff),
			Debuff(Debuff),
			Guarded(Guarded),
			PersistentHeal(PersistentHeal),
			Mark(Mark),
			Poison(Poison),
			Riposte(Riposte),
		}
	}

	DELEGATES: {
		impl trait IStatusEffect {
			[fn duration_ms(&self) -> Int]
			[fn set_duration(&mut self, ms: Int)]
			[fn tick(
				&mut self,
				actor: &mut Ptr<Actor>,
				ctx: &mut ActorContext,
				delta_ms: Int,
			) -> (CharacterTickResult, StatusTickResult)]
		}
	}
}

pub trait IStatusEffect {
	fn duration_ms(&self) -> Int;
	fn set_duration(&mut self, ms: Int);

	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> (CharacterTickResult, StatusTickResult) {
		self.set_duration({
			let mut duration = self.duration_ms();
			duration -= delta_ms;
			duration
		});

		if *self.duration_ms() > 0 {
			(CharacterTickResult::Alive, StatusTickResult::Active)
		} else {
			(CharacterTickResult::Alive, StatusTickResult::Ended)
		}
	}
}
