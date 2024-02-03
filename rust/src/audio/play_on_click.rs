use gdnative::api::{AudioStreamPlayer2D, GlobalConstants, InputEventMouseButton};
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils_gdnative::prelude::*;
use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::util;

#[extends(AudioStreamPlayer2D)]
#[derive(Debug)]
pub struct PlayOnClickAndPitchRandomizer {
	original_pitch: f64,
}

#[methods]
impl PlayOnClickAndPitchRandomizer {
	#[method]
	fn _ready(&mut self, #[base] owner: &AudioStreamPlayer2D) {
		self.original_pitch = owner.pitch_scale();
		
		let owner_ref = unsafe { owner.assume_shared() };
		let parent_option = owner.get_parent();
		let parent = parent_option.unwrap_manual();
		parent.connect("gui_input", owner_ref, util::fn_name(&Self::_on_gui_input), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
	
	#[method]
	fn _on_gui_input(&self, #[base] owner: &AudioStreamPlayer2D, event: Ref<InputEvent>) {
		let event = unsafe { event.assume_safe() };
		
		if (event.cast::<InputEventMouseButton>().is_some_and(|e| e.is_pressed() && e.button_index() == GlobalConstants::BUTTON_LEFT)) 
			|| event.is_action_pressed("ui_accept", false, true) {
			let mut rng = Xoshiro256PlusPlus::from_entropy();
			let pitch = self.original_pitch * (0.9 + rng.gen_range(0.0..=0.2));
			owner.set_pitch_scale(pitch);
			owner.play(0.0);
		}
	}
}