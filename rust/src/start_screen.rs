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
				| GlobalConstants::KEY_F4 => {
				}
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
					self.pages[index]
						.unwrap_manual()
						.hide();
					self.pages[new_index]
						.unwrap_manual()
						.show();
				} else {
					self.state = State::Transitioning;
					owner.set_process_input(false);
					let owner_ref = unsafe { owner.assume_shared() };
					
					let scene_tree_option = owner.get_tree();
					let scene_tree = scene_tree_option.unwrap_manual();
					let tween_ref = scene_tree
						.create_tween()
						.unwrap();
					let tween = unsafe { tween_ref.assume_safe() };
					tween.tween_property(owner_ref, "modulate", Color::from_rgb(0., 0., 0.), 2.);
					tween.connect("finished", owner_ref, fn_name(&Self::_fade_finished), 
							VariantArray::new_shared(), 0)
						.log_if_err();
				}
			}
			State::Transitioning => { }
		}
	}
	
	#[method]
	fn _fade_finished(&self, #[base] owner: &Control) {
		let scene_tree_option = owner.get_tree();
		let scene_tree = scene_tree_option.unwrap_manual();

		let gm_resource_option: Option<Ref<Resource>> = ResourceLoader::godot_singleton()
			.load("res://game_manager.tscn", "PackedScene", false);
		let gm_resource = gm_resource_option.unwrap_refcount();
		let gm_packed_scene = gm_resource
			.cast::<PackedScene>()
			.unwrap();

		scene_tree.change_scene_to(gm_packed_scene)
				.log_if_err();
	}
}
