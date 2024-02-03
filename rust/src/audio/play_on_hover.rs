use gdnative_export_node_as_path::extends;
use gdnative::prelude::*;
use houta_utils_gdnative::prelude::*;
use crate::audio::PitchRandomizer;
use crate::util;

#[extends(Control)]
#[derive(Debug)]
pub struct PlayOnHover {
	#[export_path] audio_stream_player: Option<Instance<PitchRandomizer>>,
}

#[methods]
impl PlayOnHover {
	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		
		let Some(stream_player) = &self.audio_stream_player
			else {
				godot_error!("PlayOnHover::_ready: No AudioStreamPlayer2D found!\n{owner:?}");
				return;
			};
		
		owner.connect("mouse_entered", stream_player, util::fn_name(&PitchRandomizer::_play_custom),
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
}