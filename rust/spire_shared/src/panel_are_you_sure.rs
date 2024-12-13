use super::*;

#[derive(GodotClass)]
#[class(base = Control)]
pub struct PanelAreYouSure {
	base:  Base<Control>,
	label: Gd<Label>,
}

#[godot_api]
impl IControl for PanelAreYouSure {
	fn init(base: Base<Self::Base>) -> Self {
		let gd = base.to_gd();
		let label = gd.get_node_as::<Label>("label");
		PanelAreYouSure { base, label }
	}

	fn ready(&mut self) {
		self.connect_child("button_no", "pressed", |this, _| {
			this.base_mut().hide();
		})
		.log_if_err();

		self.connect_child("button_yes", "pressed", |this, _| {
			let mut base = this.base_mut();
			base.hide();
			base.emit_signal(Self::SIGNAL_YES, &[]);
		})
		.log_if_err();
	}
}

#[godot_api]
impl PanelAreYouSure {
	pub const SIGNAL_YES: &'static str = "on_yes";

	#[signal]
	fn on_yes() {}

	pub fn set_text(&mut self, text: &str) { self.label.set_text(text); }

	pub fn show(&mut self) { self.base_mut().show(); }
}
