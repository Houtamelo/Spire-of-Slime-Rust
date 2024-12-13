use super::*;

const MAX_ALPHA: f64 = 160. / 255.;

#[derive(GodotClass)]
#[class(base = Node2D, init)]
pub struct SpeedLines {
	base: Base<Node2D>,
	default_pos: Vector2,
}

#[godot_api]
impl INode2D for SpeedLines {
	fn ready(&mut self) { self.default_pos = self.base().get_position(); }
}

impl SpeedLines {
	pub fn animate(&mut self, fade_duration: f64, stay_duration: f64, speed: f64) -> SpireSequence {
		let default_pos = self.default_pos;
		let mut base = self.base_mut();
		base.set_position(default_pos);

		let mut seq = SpireSequence::new();
		seq.join(base.do_fade(MAX_ALPHA, fade_duration));
		seq.join(
			base.do_move_x(speed, fade_duration * 2. + stay_duration)
				.as_relative(0.),
		);
		seq
	}
}
