use gdnative::prelude::*;
use houta_utils::prelude::*;
use gdrust_export_nodepath::extends;

pub static signal_yes: &str = "_on_yes";

#[derive(Debug)]
#[extends(Control)]
#[register_with(Self::register)]
pub struct PanelAreYouSure {
	#[property] button_yes: Option<Ref<Button>>,
	#[property] button_no : Option<Ref<Button>>,
	#[property] label     : Option<Ref<Label >>,
}

#[methods]
impl PanelAreYouSure {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(signal_yes).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let owner_ref = unsafe { owner.assume_shared() };
		self.button_no .unwrap_manual().connect("pressed", owner_ref, "_on_no_pressed" , VariantArray::new_shared(), Object::CONNECT_DEFERRED);
		self.button_yes.unwrap_manual().connect("pressed", owner_ref, "_on_yes_pressed", VariantArray::new_shared(), Object::CONNECT_DEFERRED);
	}

	#[method]
	fn _on_yes_pressed(&self, #[base] owner: &Control) {
		owner.hide();
		owner.emit_signal(signal_yes, &[]);
	}

	#[method]
	fn _on_no_pressed(&self, #[base] owner: &Control) {
		owner.hide();
	}

	pub fn set_text(&self, text: &str) {
		self.label.unwrap_manual().set_text(text);
	}
}