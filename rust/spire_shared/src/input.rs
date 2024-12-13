use super::*;

pub fn any_cancel_input(event: Gd<InputEvent>) -> bool {
	event.is_action("ui_cancel")
		|| event
			.clone()
			.try_cast::<InputEventMouseButton>()
			.ok()
			.is_some_and(|mouse_event| {
				mouse_event.is_pressed()
					&& mouse_event.get_button_index() == godot::global::MouseButton::LEFT
			}) || event
		.try_cast::<InputEventKey>()
		.ok()
		.is_some_and(|key_event| {
			key_event.is_pressed() && key_event.get_keycode() == godot::global::Key::ESCAPE // Escape key 
		})
}

pub fn is_confirm_input(event: Gd<InputEvent>) -> bool {
	event.is_action_pressed("ui_accept")
		|| event
			.try_cast::<InputEventMouseButton>()
			.ok()
			.is_some_and(|mouse_event| {
				mouse_event.is_pressed()
					&& mouse_event.get_button_index() == godot::global::MouseButton::LEFT
			})
}
