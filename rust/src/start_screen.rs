#[allow(unused_imports)]
use crate::*;

#[derive(Debug)]
enum State {
	OnPage(usize),
	Transitioning,
}

impl Default for State {
	fn default() -> Self {
		return State::OnPage(0);
	}
}

#[extends(Control)]
#[derive(Debug)]
pub struct StartScreenController {
	#[export_path] pages: Vec<Ref<Control>>,
	state: State,
}

#[methods]
impl StartScreenController {
	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
	}

	#[method]
	fn _unhandled_input(&mut self, #[base] owner: &Control, event: Ref<InputEvent>) {
		let event = unsafe { event.assume_safe() };

		if let Some(keyboard_event) = event.cast::<InputEventKey>()
			&& keyboard_event.is_pressed() {
			match keyboard_event.scancode() {
				GlobalConstants::KEY_ESCAPE => {
					owner.get_tree()
					     .unwrap_manual()
					     .quit(0);
				},
				GlobalConstants::KEY_ALT
				| GlobalConstants::KEY_F4 => {}
				_ => { self.next_page(owner) }
			}
		} else if let Some(mouse_event) = event.cast::<InputEventMouseButton>()
			&& mouse_event.is_pressed() {
			self.next_page(owner);
		}
	}

	fn next_page(&mut self, owner: &Control) {
		match self.state {
			State::OnPage(index) => {
				let new_index = index + 1;
				if new_index < self.pages.len() {
					self.state = State::OnPage(new_index);

					self.pages[index].touch_assert_sane(|page| {
						page.hide()
					});

					self.pages[new_index].touch_assert_sane(|page| {
						page.show()
					});
				} else {
					self.state = State::Transitioning;
					owner.set_process_input(false);

					owner.do_color(Color::from_rgb(0., 0., 0.), 2.)
					     .method_when_finished(owner, fn_name(&Self::_fade_finished), vec![])
					     .register()
					     .log_if_err();
				}
			}
			State::Transitioning => {}
		}
	}

	#[method]
	fn _fade_finished(&self, #[base] owner: &Control) {
		owner.get_tree()
		     .touch_assert_sane(|tree| {
			     let scene =
				     load_resource_as::<PackedScene>("res://game_manager.tscn")
					     .expect("Failed to load game_manager.tscn");

			     tree.change_scene_to(scene)
			         .log_if_err();
		     });
	}
}
