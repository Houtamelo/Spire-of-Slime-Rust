use anyhow::Result;
use comfy_bounded_ints::prelude::Bound_u8;
use gdnative::api::*;
use gdnative::prelude::*;
use houta_utils::full_fn_name;
use houta_utils_gdnative::prelude::GodotManualSomeInspector;

use crate::combat::entity::stat::CheckedRange;
use crate::combat::ui::get_ref_or_bail;

#[derive(PartialEq, Eq)]
enum State {
	Hidden,
	Displaying,
}

#[derive(NativeClass)]
#[no_constructor]
#[inherit(Reference)]
pub struct TargetingTooltip {
	owner_ref: Ref<Control>,
	hit_label: Ref<Label>,
	crit_label: Ref<Label>,
	dmg_label: Ref<Label>,
	effects_label: Ref<Label>,
	state: State,
}

fn show_label(label: Ref<Label>, text: String) {
	label.touch_assert_sane(|l| {
		l.set_text(text);
		l.show();
	});
}

fn hide_label(label: Ref<Label>) {
	label.touch_assert_sane(|l| 
		l.hide());
}

#[methods]
impl TargetingTooltip {
	pub fn build_in(owner: TRef<Control>) -> Result<()> {
		let hit_label = get_ref_or_bail!(owner, "hit", Label)?;
		let crit_label = get_ref_or_bail!(owner, "crit", Label)?;
		let dmg_label = get_ref_or_bail!(owner, "dmg", Label)?;
		let effects_label = get_ref_or_bail!(owner, "effects", Label)?;
		
		let owner_ref = unsafe { owner.assume_shared() };
		
		let this = TargetingTooltip {
			owner_ref,
			hit_label,
			crit_label,
			dmg_label,
			effects_label,
			state: State::Hidden,
		}.emplace();
		
		owner.set_script(this);
		owner.set_visible(false);
		
		return Ok(());
	}

	#[method]
	fn _notification(&mut self, what: i64) {
		if what == Node::NOTIFICATION_PROCESS
		&& self.state == State::Displaying {
			self.update_position();
		}
	}
	
	pub fn display(&mut self, 
	               hit_chance: Option<Bound_u8<0, 100>>, 
	               crit_chance: Option<Bound_u8<0, 100>>, 
	               dmg: Option<CheckedRange>, 
	               effects: Option<String>) {
		match hit_chance {
			Some(value) => 
				show_label(self.hit_label, format!("{}%", value.get())),
			None => 
				hide_label(self.hit_label),
		}

		match crit_chance {
			Some(value) => 
				show_label(self.crit_label, format!("{}%", value.get())),
			None => 
				hide_label(self.crit_label),
		}

		match dmg {
			Some(value) => 
				show_label(self.dmg_label, format!("{}~{}", value.bound_lower(), value.bound_upper())),
			None => 
				hide_label(self.dmg_label),
		}

		match effects {
			Some(value) => 
				show_label(self.effects_label, value),
			None => 
				hide_label(self.effects_label),
		}
		
		self.state = State::Displaying;
		self.update_position();
	}
	
	fn update_position(&self) {
		let Some(owner) = (unsafe { self.owner_ref.assume_safe_if_sane() })
			else { 
				godot_warn!("{}: owner is not sane", full_fn_name(&Self::update_position));
				return;
			};
		
		let Some(viewport_ref) = owner.get_viewport()
			else { 
				godot_warn!("{}: owner has no viewport", full_fn_name(&Self::update_position));
				return;
			};
		
		let Some(viewport) = (unsafe { viewport_ref.assume_safe_if_sane() })
			else { 
				godot_warn!("{}: viewport is not sane", full_fn_name(&Self::update_position));
				return;
			};
		
		let mouse_position = viewport.get_mouse_position();
		owner.set_global_position(mouse_position, false);
	}
	
	pub fn hide(&mut self) {
		if self.state == State::Displaying {
			self.state = State::Hidden;
			self.owner_ref.touch_assert_sane(|owner| 
				owner.hide());
		}
	}
}
