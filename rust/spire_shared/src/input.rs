use util_gdnative::prelude::*;

pub fn any_cancel_input(event: &InputEvent) -> bool {
	return event.is_action("ui_cancel", false)
		|| event.cast::<InputEventMouseButton>()
		        .is_some_and(|mouse_event| {
			        mouse_event.button_index() == 2
		        })
		|| event.cast::<InputEventKey>()
		        .is_some_and(|key_event| {
			        key_event.scancode() == 16777217 // Escape key 
		        });
}

pub fn is_confirm_input(event: &InputEvent) -> bool {
	return event.is_action_pressed("ui_accept", false, true)
		|| event.cast::<InputEventMouseButton>()
		        .is_some_and(|mouse_event| {
			             mouse_event.is_pressed()
				      && mouse_event.button_index() == GlobalConstants::BUTTON_LEFT
		             });
}