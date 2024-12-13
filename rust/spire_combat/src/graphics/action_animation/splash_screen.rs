use super::*;

/// The returned tween only ends when killed manually.
pub fn animate_movement(
	node: &mut Gd<Node2D>,
	start_local_pos: Vector2,
	speed: f64,
) -> SpireSequence {
	let mut sequence = SpireSequence::new();
	sequence.append_call({
		let mut node = node.clone();
		move || node.set_position(start_local_pos)
	});

	sequence.append(node.do_move_x(speed, 1.0).as_relative(0.));
	sequence
}
