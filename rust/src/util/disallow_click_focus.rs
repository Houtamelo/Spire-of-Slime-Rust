use gdnative::api::InputEventMouseButton;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct DisallowClickFocus { }

#[methods]
impl DisallowClickFocus {
	fn new(_owner: &Control) -> Self {
		DisallowClickFocus { }
	}

	#[method]
	fn _gui_input(&self, #[base] _owner: &Control, event: Ref<InputEvent>) {
		let event = unsafe { event.assume_safe() };
		if let Some(mouse_button_event) = event.cast::<InputEventMouseButton>() {
			if mouse_button_event.is_pressed() {
				_owner.release_focus();
			}
		}
	}
}