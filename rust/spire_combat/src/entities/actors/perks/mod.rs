use super::*;

mod actor_extensions;
mod macros;

pub use actor_extensions::*;
pub use macros::*;

pub trait IPerk {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		ctx: &mut ActorContext,
		_delta_ms: Int,
	) -> PerkTickResult {
		PerkTickResult::Active
	}
}

pub trait IGirlPerk {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		girl: &mut Ptr<Girl>,
		ctx: &mut ActorContext,
		_delta_ms: Int,
	) -> PerkTickResult {
		PerkTickResult::Active
	}
}
