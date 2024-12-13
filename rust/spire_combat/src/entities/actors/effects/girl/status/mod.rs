pub use types::*;

mod types;

use super::*;

delegated_enum! {
	ENUM_OUT: {
	#[derive(Serialize, Deserialize, Clone)]
		pub enum GirlStatus {
			Arousal(Arousal),
			Buff(GirlBuff),
			Debuff(GirlDebuff),
		}
	}

	DELEGATES: {
		impl trait IGirlStatusEffect {
			[fn duration_ms(&self) -> Int]
			[fn set_duration(&mut self, ms: Int)]
			[fn tick(
				&mut self,
				actor: &mut Ptr<Actor>,
				girl: &mut Ptr<Girl>,
				ctx: &mut ActorContext,
				delta_ms: Int,
			) -> (CharacterTickResult, StatusTickResult)]
		}
	}
}

pub trait IGirlStatusEffect {
	fn duration_ms(&self) -> Int;
	fn set_duration(&mut self, ms: Int);

	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> (CharacterTickResult, StatusTickResult) {
		let duration = {
			let mut temp = self.duration_ms();
			temp -= delta_ms;
			temp
		};

		self.set_duration(duration);

		if *self.duration_ms() > 0 {
			(CharacterTickResult::Alive, StatusTickResult::Active)
		} else {
			(CharacterTickResult::Alive, StatusTickResult::Ended)
		}
	}
}
