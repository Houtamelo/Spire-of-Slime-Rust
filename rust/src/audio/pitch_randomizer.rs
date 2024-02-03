use gdnative::api::AudioStreamPlayer2D;
use gdnative_export_node_as_path::extends;
use gdnative::prelude::*;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[extends(AudioStreamPlayer2D)]
#[derive(Debug)]
pub struct PitchRandomizer {
	original_pitch: f64,
}

#[methods]
impl PitchRandomizer {
	#[method]
	fn _ready(&mut self, #[base] owner: &AudioStreamPlayer2D) {
		self.original_pitch = owner.pitch_scale();
	}
	
	#[method]
	pub fn _play_custom(&self, #[base] owner: &AudioStreamPlayer2D) {
		let mut rng = Xoshiro256PlusPlus::from_entropy();
		let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
		owner.set_pitch_scale(pitch);
		owner.play(0.0);
	}
}