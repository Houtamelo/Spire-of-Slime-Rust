use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils_gdnative::prelude::*;

pub static SIGNAL_YES: &str = "_on_yes";

#[derive(Debug)]
#[extends(Control)]
#[register_with(Self::register)]
pub struct PanelAreYouSure {
	#[export_path] button_yes: Option<Ref<Button>>,
	#[export_path] button_no: Option<Ref<Button>>,
	#[export_path] label: Option<Ref<Label>>,
}

#[methods]
impl PanelAreYouSure {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_YES).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		
		let owner_ref = unsafe { owner.assume_shared() };

		self.button_no.unwrap_manual()
			.connect("pressed", owner_ref, houta_utils::fn_name(&Self::_button_pressed_no), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		self.button_yes.unwrap_manual()
			.connect("pressed", owner_ref, houta_utils::fn_name(&Self::_button_pressed_yes), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}

	#[method]
	fn _button_pressed_yes(&self, #[base] owner: &Control) {
		owner.hide();
		owner.emit_signal(SIGNAL_YES, &[]);
	}

	#[method]
	fn _button_pressed_no(&self, #[base] owner: &Control) {
		owner.hide();
	}

	pub fn set_text(&self, text: &str) {
		self.label.unwrap_manual().set_text(text);
	}
}