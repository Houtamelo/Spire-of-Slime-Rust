use crate::internal_prelude::*;

pub(super) const SIGNAL_LOAD: &str = "load_save";
pub(super) const SIGNAL_DELETE: &str = "delete_save_confirmed";

#[extends(Control)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct LoadButton {
	#[export_path] button_load  : Option<Ref<Button>>,
	#[export_path] button_delete: Option<Ref<Button>>,
	#[export_path] button_confirm_delete: Option<Ref<Button>>,
	#[export_path] label: Option<Ref<Label>>,
	pub(super) save_name: Option<String>,
}

#[methods]
impl LoadButton {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_LOAD)
			.with_param("save_name", VariantType::GodotString)
			.done();
		builder.signal(SIGNAL_DELETE)
			.with_param("save_name", VariantType::GodotString)
			.done();
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		
		let owner_ref = unsafe { owner.assume_shared() };
		self.button_load.unwrap_manual()
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_load), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		self.button_delete.unwrap_manual()
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_delete), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		self.button_confirm_delete.unwrap_manual()
			.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_confirm_delete), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}

	#[method]
	fn _button_pressed_load(&self, #[base] owner: &Control) {
		if let Some(save_name) = &self.save_name {
			owner.emit_signal(SIGNAL_LOAD, &[save_name.to_variant()]);
		} else {
			godot_warn!("{}():\n\
			 LoadButton `{}` was pressed, but no save was assigned to it.", 
				fn_name(&Self::_button_pressed_load), owner.name());
		};
	}

	#[method]
	fn _button_pressed_delete(&self) {
		self.button_confirm_delete
			.unwrap_manual()
			.show();
	}

	#[method]
	fn _button_pressed_confirm_delete(&self, #[base] owner: &Control) {
		owner.hide();
		
		if let Some(save_name) = &self.save_name {
			owner.emit_signal(SIGNAL_DELETE, &[save_name.to_variant()]);
		} else {
			godot_warn!("{}():\n\
				DeleteSaveButton `{}` was pressed, but no save was assigned to it.", 
				fn_name(&Self::_button_pressed_confirm_delete), owner.name());
		}
	}
}