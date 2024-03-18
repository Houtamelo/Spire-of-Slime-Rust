use std::any::type_name;

use anyhow::Result;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils::fn_name;
use houta_utils_gdnative::prelude::{ErrInspector, GodotManualSomeInspector};

use crate::combat::ui::get_or_bail;

struct MappedButtons {
	pause_button: Ref<Button>,
	pause_indicator: Ref<Control>,
	speed_x1_button: Ref<Button>,
	speed_x1_indicator: Ref<Control>,
	speed_x2_button: Ref<Button>,
	speed_x2_indicator: Ref<Control>,
	speed_x3_button: Ref<Button>,
	speed_x3_indicator: Ref<Control>,
}

impl MappedButtons {
	pub fn new(root_node: &Control) -> Result<Self> {
		let pause_button = get_or_bail!(root_node, "pause", Button)?;
		let pause_indicator = get_or_bail!(root_node, "pause/indicator", Control)?;
		
		let speed_x1_button = get_or_bail!(root_node, "speed_x1", Button)?;
		let speed_x1_indicator = get_or_bail!(root_node, "speed_x1/indicator", Control)?;
		
		let speed_x2_button = get_or_bail!(root_node, "speed_x2", Button)?;
		let speed_x2_indicator = get_or_bail!(root_node, "speed_x2/indicator", Control)?;
		
		let speed_x3_button = get_or_bail!(root_node, "speed_x3", Button)?;
		let speed_x3_indicator = get_or_bail!(root_node, "speed_x3/indicator", Control)?;
		
		Ok(Self {
			pause_button,
			pause_indicator,
			speed_x1_button,
			speed_x1_indicator,
			speed_x2_button,
			speed_x2_indicator,
			speed_x3_button,
			speed_x3_indicator,
		})
	}
}

#[derive(Clone, Copy)]
pub enum Speed {
	X1, 
	X2, 
	X3
}

#[derive(Clone, Copy)]
pub enum SpeedSetting {
	Paused { previous_speed: Speed },
	UnPaused { speed: Speed },
}

impl Default for SpeedSetting {
	fn default() -> Self {
		Self::UnPaused { speed: Speed::X1 }
	}
}

#[extends(Control)]
pub struct SpeedButtons {
	mapped_buttons: Option<MappedButtons>,
	speed_setting: SpeedSetting,
}

macro_rules! set_indicator_visible {
    ($self: ident, $indicator: ident, $visible: expr) => {
	    $self.mapped_buttons.as_ref().unwrap()
			.$indicator
			.touch_assert_sane(|indicator| 
				indicator.set_visible($visible));
    };
}

#[methods] 
impl SpeedButtons {
	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let owner_ref = unsafe { owner.assume_shared() };
		
		let mapped_buttons = MappedButtons::new(owner)
			.expect(format!(
				"{}: Failed to map speed buttons. \n\
				 Owner: {}", type_name::<SpeedButtons>(), owner.name()).as_str());
		
		
		mapped_buttons
			.pause_button
			.touch_assert_sane(|button| {
				button.connect("pressed", owner_ref, fn_name(&Self::_pause_pressed), VariantArray::new_shared(), Object::CONNECT_DEFERRED)
					  .log_if_err();
			});
		
		mapped_buttons
			.speed_x1_button
			.touch_assert_sane(|button| {
				button.connect("pressed", owner_ref, fn_name(&Self::_speed_1x_pressed), VariantArray::new_shared(), Object::CONNECT_DEFERRED)
					  .log_if_err();
			});
		
		mapped_buttons
			.speed_x2_button
			.touch_assert_sane(|button| {
				button.connect("pressed", owner_ref, fn_name(&Self::_speed_2x_pressed), VariantArray::new_shared(), Object::CONNECT_DEFERRED)
					  .log_if_err();
			});
		
		mapped_buttons
			.speed_x3_button
			.touch_assert_sane(|button| {
				button.connect("pressed", owner_ref, fn_name(&Self::_speed_3x_pressed), VariantArray::new_shared(), Object::CONNECT_DEFERRED)
					  .log_if_err();
			});
		
		set_indicator_visible!(self, pause_indicator, false);
		set_indicator_visible!(self, speed_x1_indicator, true);
		set_indicator_visible!(self, speed_x2_indicator, false);
		set_indicator_visible!(self, speed_x3_indicator, false);
	}
	
	#[method]
	fn _pause_pressed(&mut self) {
		let is_indicator_visible =
			match self.speed_setting {
				SpeedSetting::Paused { previous_speed } => {
					self.speed_setting = SpeedSetting::UnPaused { speed: previous_speed };
					false
				}
				SpeedSetting::UnPaused { speed } => {
					self.speed_setting = SpeedSetting::Paused { previous_speed: speed };
					true
				}
			};
		
		set_indicator_visible!(self, pause_indicator, is_indicator_visible);
	}
	
	#[method]
	fn _speed_1x_pressed(&self) {
		if let SpeedSetting::UnPaused { speed: Speed::X1 } | SpeedSetting::Paused { previous_speed: Speed::X1 } = self.speed_setting {
			return;
		}
		
		set_indicator_visible!(self, speed_x1_indicator, true);
		set_indicator_visible!(self, speed_x2_indicator, false);
		set_indicator_visible!(self, speed_x3_indicator, false);
	}
	
	#[method]
	fn _speed_2x_pressed(&self) {
		if let SpeedSetting::UnPaused { speed: Speed::X2 } | SpeedSetting::Paused { previous_speed: Speed::X2 } = self.speed_setting {
			return;
		}
		
		set_indicator_visible!(self, speed_x1_indicator, false);
		set_indicator_visible!(self, speed_x2_indicator, true);
		set_indicator_visible!(self, speed_x3_indicator, false);
	}
	
	#[method]
	fn _speed_3x_pressed(&self) {
		if let SpeedSetting::UnPaused { speed: Speed::X3 } | SpeedSetting::Paused { previous_speed: Speed::X3 } = self.speed_setting {
			return;
		}
		
		set_indicator_visible!(self, speed_x1_indicator, false);
		set_indicator_visible!(self, speed_x2_indicator, false);
		set_indicator_visible!(self, speed_x3_indicator, true);
	}
	
	pub fn speed(&self) -> SpeedSetting {
		return self.speed_setting;
	}
}
