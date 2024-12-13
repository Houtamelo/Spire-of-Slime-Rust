use super::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct TargetingTooltip {
	base: Base<Control>,
	#[init(node = "panel-container/vertical-container/hit-chance")]
	hit_root: OnReady<Gd<Control>>,
	#[init(node = "horizontal-container/value")]
	hit_label: OnReady<Gd<Label>>,
	#[init(node = "panel-container/vertical-container/crit-chance")]
	crit_root: OnReady<Gd<Control>>,
	#[init(node = "horizontal-container/value")]
	crit_label: OnReady<Gd<Label>>,
	#[init(node = "panel-container/vertical-container/damage")]
	dmg_root: OnReady<Gd<Control>>,
	#[init(node = "horizontal-container/value")]
	dmg_label: OnReady<Gd<Label>>,
	#[init(node = "panel-container/vertical-container/effects")]
	effects_label: OnReady<Gd<Label>>,
}

#[godot_api]
impl IControl for TargetingTooltip {
	fn process(&mut self, delta: f64) {
		if self.base().is_visible() {
			self.update_position();
		}
	}
}

#[godot_api]
impl TargetingTooltip {
	pub fn display(
		&mut self,
		hit_chance: Option<Int>,
		crit_chance: Option<Int>,
		dmg: Option<SaneRange>,
		effects: Option<String>,
	) {
		set_label(&mut self.hit_label, hit_chance);
		set_label(&mut self.crit_label, crit_chance);
		set_label(&mut self.dmg_label, dmg);
		set_label(&mut self.effects_label, effects);

		self.update_position();
	}

	fn update_position(&mut self) {
		if let Some(viewport) = self.base().get_viewport() {
			let mouse_position = viewport.get_mouse_position();
			self.base_mut().set_global_position(mouse_position);
		} else {
			godot_warn!("ActorBase has no viewport");
		}
	}
}

fn set_label<T: Display>(label: &mut Gd<Label>, option: Option<T>) {
	if let Some(var) = option {
		label.set_text(&format!("{var}"));
		label.show();
	} else {
		label.hide();
	}
}
