use gdnative::prelude::*;
use gdnative::api::object::ConnectFlags;
use crate::{ErrInspector, GodotManualSomeInspector};

pub static signal_yes: &str = "_on_yes";

#[derive(NativeClass, Default)]
#[register_with(Self::register)]
#[inherit(Control)]
pub struct PanelAreYouSure {
	#[property] button_yes: Option<Ref<Button>>,
	#[property] button_no : Option<Ref<Button>>,
	#[property] label     : Option<Ref<Label >>,
}

#[methods]
impl PanelAreYouSure {
	fn new(_owner: &Control) -> Self { PanelAreYouSure::default() }

	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(signal_yes).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		assert!(self.button_yes.is_some());
		assert!(self.button_no .is_some());
		assert!(self.label     .is_some());

		let owner_ref = unsafe { owner.assume_shared() };
		self.button_no .on_sane(|b|b.connect("pressed", owner_ref, "_on_no_pressed",
		                                     VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err());
		self.button_yes.on_sane(|b|b.connect("pressed", owner_ref, "_on_yes_pressed",
		                                     VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err());
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
		self.label.on_sane(|label| label.set_text(text));
	}
}