use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils_gdnative::prelude::*;

pub(super) const SIGNAL_LOAD: &str = "load_save";
pub(super) const SIGNAL_DELETE: &str = "delete_save_confirmed";

#[extends(Control)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct LoadButton {
	#[export_path] button_load          : Option<Ref<Button>>,
	#[export_path] button_delete        : Option<Ref<Button>>,
	#[export_path] button_confirm_delete: Option<Ref<Button>>,
	#[export_path] label                : Option<Ref<Label >>,
}

#[methods]
impl LoadButton {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_LOAD).with_param("save_name", VariantType::GodotString).done();
		builder.signal(SIGNAL_DELETE).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let owner_ref = unsafe { owner.assume_shared() };
		self.button_load          .unwrap_manual().connect("pressed", owner_ref, "_on_button_load_pressed"          , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
		self.button_delete        .unwrap_manual().connect("pressed", owner_ref, "_on_button_delete_pressed"        , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
		self.button_confirm_delete.unwrap_manual().connect("pressed", owner_ref, "_on_button_confirm_delete_pressed", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
	}

	#[method]
	fn _on_button_load_pressed(&self, #[base] owner: &Control) {
		self.label.touch_assert_sane(|label| { owner.emit_signal(SIGNAL_LOAD, &[label.text().to_variant()]); });
	}

	#[method]
	fn _on_button_delete_pressed(&self) {
		self.button_confirm_delete.unwrap_manual().show();
	}

	#[method]
	fn _on_button_confirm_delete_pressed(&self, #[base] owner: &Control) {
		let label = self.label.unwrap_manual();
		owner.hide();
		owner.emit_signal(SIGNAL_DELETE, &[label.text().to_variant()]);
	}

	pub fn set_save_name(&self, text: &str) {
		self.label.unwrap_manual().set_text(text);
	}
}