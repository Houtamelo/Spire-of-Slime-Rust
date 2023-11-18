use gdnative::prelude::*;
use houta_utils::prelude::*;
use gdrust_export_nodepath::extends;

pub(super) static signal_load  : &str = "load_save";
pub(super) static signal_delete: &str = "delete_save_confirmed";

#[extends(Control)]
#[register_with(Self::register)]
pub struct LoadButton {
	#[export_path] button_load          : Option<Ref<Button>>,
	#[export_path] button_delete        : Option<Ref<Button>>,
	#[export_path] button_confirm_delete: Option<Ref<Button>>,
	#[export_path] label                : Option<Ref<Label >>,
}

#[methods]
impl LoadButton {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(signal_load  ).with_param("save_name", VariantType::GodotString).done();
		builder.signal(signal_delete).done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let owner_ref = unsafe { owner.assume_shared() };
		self.button_load          .unwrap_manual().connect("pressed", owner_ref, "_on_button_load_pressed"          , VariantArray::new_shared(), Object::CONNECT_DEFERRED);
		self.button_delete        .unwrap_manual().connect("pressed", owner_ref, "_on_button_delete_pressed"        , VariantArray::new_shared(), Object::CONNECT_DEFERRED);
		self.button_confirm_delete.unwrap_manual().connect("pressed", owner_ref, "_on_button_confirm_delete_pressed", VariantArray::new_shared(), Object::CONNECT_DEFERRED);
	}

	#[method]
	fn _on_button_load_pressed(&self, #[base] owner: &Control) {
		self.label.touch_assert_sane(|label| { owner.emit_signal(signal_load, &[label.text().to_variant()]); });
	}

	#[method]
	fn _on_button_delete_pressed(&self) {
		self.button_confirm_delete.unwrap_manual().show();
	}

	#[method]
	fn _on_button_confirm_delete_pressed(&self, #[base] owner: &Control) {
		let label = self.label.unwrap_manual();
		owner.hide();
		owner.emit_signal(signal_delete, &[label.text().to_variant()]);
	}

	pub fn set_save_name(&self, text: &str) {
		self.label.unwrap_manual().set_text(text);
	}
}