#[allow(unused_imports)]
use crate::*;
use crate::combat::ui::get_tref_or_bail;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
	X1,
	X2,
	X3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedSetting {
	Paused { previous_speed: Speed },
	UnPaused { speed: Speed },
}

impl SpeedSetting {
	pub fn speed(&self) -> Speed {
		match self {
			SpeedSetting::Paused { previous_speed } => *previous_speed,
			SpeedSetting::UnPaused { speed } => *speed,
		}
	}
}

#[derive(NativeClass)]
#[no_constructor]
#[inherit(Reference)]
pub struct SpeedButtons {
	speed_setting: SpeedSetting,
	pause_button: Ref<Button>,
	pause_indicator: Ref<Control>,
	speed_x1_button: Ref<Button>,
	speed_x1_indicator: Ref<Control>,
	speed_x2_button: Ref<Button>,
	speed_x2_indicator: Ref<Control>,
	speed_x3_button: Ref<Button>,
	speed_x3_indicator: Ref<Control>,
}

fn check_indicators(pause_indicator: TRef<Control>,
                    speed_x1_indicator: TRef<Control>,
                    speed_x2_indicator: TRef<Control>,
                    speed_x3_indicator: TRef<Control>,
                    speed_setting: SpeedSetting) {
	pause_indicator.set_visible(matches!(speed_setting, SpeedSetting::Paused { .. }));

	let speed = speed_setting.speed();
	speed_x1_indicator.set_visible(speed == Speed::X1);
	speed_x2_indicator.set_visible(speed == Speed::X2);
	speed_x3_indicator.set_visible(speed == Speed::X3);
}

macro_rules! set_indicator_visible {
    ($self: ident, $indicator: ident, $visible: expr) => {
	    $self.$indicator
			 .touch_assert_sane(|indicator| 
				 indicator.set_visible($visible));
    };
}

#[methods]
impl SpeedButtons {
	pub fn build_in(owner: TRef<Control>, speed_setting: SpeedSetting) -> Result<()> {
		let owner_ref = unsafe { owner.assume_shared() };

		let pause_indicator = get_tref_or_bail!(owner, "pause/indicator", Control)?;
		let speed_x1_indicator = get_tref_or_bail!(owner, "speed_x1/indicator", Control)?;
		let speed_x2_indicator = get_tref_or_bail!(owner, "speed_x2/indicator", Control)?;
		let speed_x3_indicator = get_tref_or_bail!(owner, "speed_x3/indicator", Control)?;
		check_indicators(pause_indicator, speed_x1_indicator, speed_x2_indicator, speed_x3_indicator, speed_setting);
		
		let pause_button = get_tref_or_bail!(owner, "pause", Button)?;
		let speed_x1_button = get_tref_or_bail!(owner, "speed_x1", Button)?;
		let speed_x2_button = get_tref_or_bail!(owner, "speed_x2", Button)?;
		let speed_x3_button = get_tref_or_bail!(owner, "speed_x3", Button)?;
		
		let this = Self {
			speed_setting,
			pause_button: unsafe { pause_button.assume_shared() },
			pause_indicator: unsafe { pause_indicator.assume_shared() },
			speed_x1_button: unsafe { speed_x1_button.assume_shared() },
			speed_x1_indicator: unsafe { speed_x1_indicator.assume_shared() },
			speed_x2_button: unsafe { speed_x2_button.assume_shared() },
			speed_x2_indicator: unsafe { speed_x2_indicator.assume_shared() },
			speed_x3_button: unsafe { speed_x3_button.assume_shared() },
			speed_x3_indicator: unsafe { speed_x3_indicator.assume_shared() },
		}.emplace();
		
		owner.set_script(this);

		pause_button
			.connect("pressed", owner_ref, fn_name(&Self::_pause_pressed), 
			         VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();

		speed_x1_button
			.connect("pressed", owner_ref, fn_name(&Self::_speed_1x_pressed),
			         VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		
		speed_x2_button
			.connect("pressed", owner_ref, fn_name(&Self::_speed_2x_pressed), 
			         VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();

		speed_x3_button
			.connect("pressed", owner_ref, fn_name(&Self::_speed_3x_pressed),
			         VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();

		return Ok(());
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
