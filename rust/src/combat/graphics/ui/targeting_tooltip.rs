#[allow(unused_imports)]
use crate::*;

use crate::combat::entity::stat::CheckedRange;
use super::get_ref_or_bail;

#[derive(Debug, PartialEq, Eq)]
enum State {
	Hidden,
	Displaying,
}

#[derive(Debug, NativeClass)]
#[no_constructor]
#[inherit(Reference)]
#[user_data(GoodCellData<TargetingTooltip>)]
pub struct TargetingTooltip {
	owner_ref: Ref<Control>,
	hit_root: Ref<Control>,
	hit_label: Ref<Label>,
	crit_root: Ref<Control>,
	crit_label: Ref<Label>,
	dmg_root: Ref<Control>,
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
	pub fn build_in(owner: &Control) -> Result<Instance<Self>> {
		let hit_root = get_ref_or_bail!(owner, "panel-container/vertical-container/hit-chance", Control)?;
		let hit_label = get_ref_or_bail!(hit_root.assume_safe(), "horizontal-container/value", Label)?;
		
		let crit_root = get_ref_or_bail!(owner, "panel-container/vertical-container/crit-chance", Control)?;
		let crit_label = get_ref_or_bail!(crit_root.assume_safe(), "horizontal-container/value", Label)?;
		
		let dmg_root = get_ref_or_bail!(owner, "panel-container/vertical-container/damage", Control)?;
		let dmg_label = get_ref_or_bail!(dmg_root.assume_safe(), "horizontal-container/value", Label)?;
		
		let effects_label = get_ref_or_bail!(owner, "panel-container/vertical-container/effects", Label)?;
		
		let owner_ref = unsafe { owner.assume_shared() };
		
		let _self = TargetingTooltip {
			owner_ref,
			hit_root,
			hit_label,
			crit_root,
			crit_label,
			dmg_root,
			dmg_label,
			effects_label,
			state: State::Hidden,
		}.emplace();
		
		_self.map_mut(|inst, _| inst.hide())?;
		
		owner.set_script(_self);
		owner.set_visible(false);
		
		owner.get_script()
			 .ok_or_else(|| anyhow!("Failed to set `{}` script for {}", type_name::<Self>(), owner.name()))
			 .map(|script| { 
				 script.cast_instance()
					   .ok_or_else(|| anyhow!("Failed to cast `{}` script for {}", type_name::<Self>(), owner.name()))
			 }).flatten()
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
