#[allow(unused_imports)]
use crate::prelude::*;
use crate::graphics::action_animation::ActionParticipant;

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
	pub fn animate(&self, part: &ActionParticipant, duration: f64) -> TweenProperty_f64 {
		let x =
			match (self.movement_type, part.pos_before.side) {
				(MovementType::TowardsCenter(x), Side::Left) => x,
				(MovementType::TowardsCenter(x), Side::Right) => -x,
				(MovementType::TowardsEdge(x), Side::Left) => -x,
				(MovementType::TowardsEdge(x), Side::Right) => x,
			};
		
		part.godot.node()
			.do_move_x(x, duration)
			.as_relative(0.)
			.with_ease(self.ease.clone())
	}
}