use super::*;

pub struct CharacterMovement {
	pub movement_type: MovementType,
	pub max: f64,
	pub ease: Ease,
}

#[derive(Debug, Copy, Clone)]
pub enum MovementType {
	TowardsCenter(f64),
	TowardsEdge(f64),
}

impl CharacterMovement {
	pub fn animate(&self, actr: &mut ActorScreenData, duration: f64) -> SpireTween<Property<f64>> {
		let x = match (self.movement_type, actr.team) {
			(MovementType::TowardsCenter(x), Team::Left) => x,
			(MovementType::TowardsCenter(x), Team::Right) => -x,
			(MovementType::TowardsEdge(x), Team::Left) => -x,
			(MovementType::TowardsEdge(x), Team::Right) => x,
		};

		actr.godot
			.node()
			.do_move_x(x, duration)
			.as_relative(0.)
			.with_ease(self.ease.clone())
	}
}
