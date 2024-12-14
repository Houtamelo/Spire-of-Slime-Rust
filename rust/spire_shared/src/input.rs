use super::*;

pub fn any_cancel_input(event: &Gd<InputEvent>) -> bool {
	if event.is_action("ui_cancel") {
		return true;
	}

	if let Ok(mouse) = event.clone().try_cast::<InputEventMouseButton>()
		&& mouse.is_pressed()
		&& mouse.get_button_index() == godot::global::MouseButton::LEFT
	{
		return true;
	}

	if let Ok(key) = event.clone().try_cast::<InputEventKey>()
		&& key.is_pressed()
		&& key.get_keycode() == godot::global::Key::ESCAPE
	{
		return true;
	}

	false
}

pub fn is_confirm_input(event: &Gd<InputEvent>) -> bool {
	if event.is_action_pressed("ui_accept") {
		return true;
	}

	if let Ok(mouse) = event.clone().try_cast::<InputEventMouseButton>()
		&& mouse.is_pressed()
		&& mouse.get_button_index() == godot::global::MouseButton::LEFT
	{
		return true;
	}

	false
}
