use keyframe::*;

pub struct Movement {
	pub movement_type: MovementType,
	pub curve: AnimationSequence<f64>,
}

pub enum MovementType {
	TowardsCenter,
	TowardsEdge,
}
