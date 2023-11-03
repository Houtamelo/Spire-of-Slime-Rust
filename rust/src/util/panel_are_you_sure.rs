use gdnative::prelude::*;
use gdnative::api::object::ConnectFlags;
use crate::util::tref_acquirer;

pub static signal_yes: &str = "on_yes";

#[derive(NativeClass)]
#[register_with(Self::register)]
#[inherit(Control)]
pub struct PanelAreYouSure {
	#[property] button_yes_path: NodePath,
	button_yes: Option<Ref<Button>>,
	#[property] button_no_path: NodePath,
	button_no: Option<Ref<Button>>,
	#[property] label_path: NodePath,
	label: Option<Ref<Label>>,
}

#[methods]
impl PanelAreYouSure {
	fn new(_owner: &Control) -> Self {
		PanelAreYouSure {
			button_yes_path: NodePath::default(), button_yes: None,
			button_no_path : NodePath::default(), button_no : None,
			label_path     : NodePath::default(), label     : None,
		}
	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(signal_yes).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let Some(button_yes) = (unsafe { owner.get_node_as::<Button>(self.button_yes_path.new_ref()) })
				else { godot_error!("Failed to get button_yes"); return; };
		self.button_yes = Some( unsafe { button_yes.assume_shared() });

		let Some(button_no) = (unsafe { owner.get_node_as::<Button>(self.button_no_path.new_ref()) })
				else { godot_error!("Failed to get button_no"); return; };
		self.button_no = Some( unsafe { button_no.assume_shared() });

		let Some(label) = (unsafe { owner.get_node_as::<Label>(self.label_path.new_ref()) })
				else { godot_error!("Failed to get label"); return; };
		self.label = Some( unsafe { label.assume_shared() });

		let owner_ref = unsafe { owner.assume_shared() };
		let result = button_no.connect("pressed", owner_ref, "_button_no_pressed", VariantArray::new_shared(), ConnectFlags::DEFERRED.into());
		if let Err(error) = result {
			godot_error!("Failed to connect button_no to _button_no_pressed: {}", error);
		}

		let result = button_yes.connect("pressed", owner_ref, "_button_yes_pressed", VariantArray::new_shared(), ConnectFlags::DEFERRED.into());
		if let Err(error) = result {
			godot_error!("Failed to connect button_yes to _button_yes_pressed: {}", error);
		}
	}

	#[method]
	fn _button_yes_pressed(&self, #[base] owner: &Control) {
		owner.hide();
		owner.emit_signal("on_yes", &[]);
	}

	#[method]
	fn _button_no_pressed(&self, #[base] owner: &Control) {
		owner.hide();
	}

	pub fn set_text(&self, text: &str) {
		if let Some(label) = tref_acquirer::assert_tref_if_sane(&self.label) {
			label.set_text(text);
		}
	}
}