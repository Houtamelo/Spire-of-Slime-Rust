use crate::internal_prelude::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct LoadButton {
	base: Base<Control>,
	#[init(node = "button_confirm_delete")]
	button_confirm_delete: OnReady<Gd<Button>>,
	#[init(node = "label")]
	label: OnReady<Gd<Label>>,
	save_name: String,
}

impl LoadButton {
	pub fn get_save_name(&self) -> &str { &self.save_name }

	pub fn set_save_name(&mut self, save_name: impl Into<String>) {
		self.save_name = save_name.into();
		self.label.set_text(&self.save_name);
	}
}

#[godot_api]
impl IControl for LoadButton {
	fn ready(&mut self) {
		self.connect_child("button_load", "pressed", |this, _| {
			let save = this.save_name.to_variant();
			this.base_mut()
				.emit_signal(Self::SIGNAL_LOAD, &[save])
				.log_if_err();
		})
		.log_if_err();

		self.connect_child("button_delete", "pressed", |this, _| {
			this.button_confirm_delete.show();
		})
		.log_if_err();

		self.connect_child("button_confirm_delete", "pressed", |this, _| {
			this.button_confirm_delete.hide();
			let save = this.save_name.to_variant();
			this.base_mut()
				.emit_signal(Self::SIGNAL_DELETE, &[save])
				.log_if_err();
		})
		.log_if_err();
	}
}

#[godot_api]
impl LoadButton {
	pub const SIGNAL_LOAD: &'static str = "load_save";
	pub const SIGNAL_DELETE: &'static str = "delete_save_confirmed";

	#[signal]
	fn load(save_name: GString) {}

	#[signal]
	fn delete(save_name: GString) {}
}
