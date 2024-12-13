use godot::global::Error;

use crate::internal_prelude::*;

#[derive(Debug)]
enum State {
	OnPage(usize),
	Transitioning,
}

impl Default for State {
	fn default() -> Self { State::OnPage(0) }
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct StartScreenController {
	base:  Base<Control>,
	#[export]
	pages: Array<Gd<Control>>,
	state: State,
}

#[godot_api]
impl IControl for StartScreenController {
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		let key_option = event
			.clone()
			.try_cast::<InputEventKey>()
			.ok()
			.and_then(|keyboard| keyboard.is_pressed().then_some(keyboard.get_keycode()));

		if let Some(key_code) = key_option {
			match key_code {
				Key::ESCAPE => {
					self.base().get_tree().unwrap().quit();
				}
				| Key::ALT | Key::F4 => {}
				_ => self.next_page(),
			}
		} else if event
			.try_cast::<InputEventMouseButton>()
			.ok()
			.is_some_and(|mouse| mouse.is_pressed())
		{
			self.next_page();
		}
	}
}

#[godot_api]
impl StartScreenController {
	fn next_page(&mut self) {
		let State::OnPage(index) = self.state
		else { return };

		let new_index = index + 1;
		if new_index < self.pages.len() {
			self.state = State::OnPage(new_index);
			self.pages.get(index).unwrap().hide();
			self.pages.get(new_index).unwrap().show();
		} else {
			self.state = State::Transitioning;
			let mut base = self.base_mut();
			base.set_process_input(false);

			base.do_color(Color::from_rgb(0., 0., 0.), 2.)
				.on_finish({
					let base = base.to_godot();
					move || {
						if let Some(mut tree) = base.get_tree() {
							let scene =
								load_resource_as::<PackedScene>("res://game_manager.tscn").unwrap();
							match tree.change_scene_to_packed(&scene) {
								Error::OK => {}
								err => {
									godot_error!("Failed to change scene: {err:?}")
								}
							}
						} else {
							godot_error!("Failed to get tree");
						}
					}
				})
				.register();
		}
	}
}
