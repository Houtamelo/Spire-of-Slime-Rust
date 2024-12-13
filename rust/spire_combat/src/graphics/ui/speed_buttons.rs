use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Speed {
	X1,
	X2,
	X3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedSetting {
	Paused { previous_speed: Speed },
	UnPaused { speed: Speed },
}

impl Default for SpeedSetting {
	fn default() -> Self { SpeedSetting::UnPaused { speed: Speed::X1 } }
}

impl SpeedSetting {
	pub fn speed(&self) -> Speed {
		match self {
			SpeedSetting::Paused { previous_speed } => *previous_speed,
			SpeedSetting::UnPaused { speed } => *speed,
		}
	}

	pub fn with_speed(self, speed: Speed) -> Self {
		match self {
			SpeedSetting::Paused { .. } => {
				SpeedSetting::Paused {
					previous_speed: speed,
				}
			}
			SpeedSetting::UnPaused { .. } => SpeedSetting::UnPaused { speed },
		}
	}

	pub fn speed_mut(&mut self) -> &mut Speed {
		match self {
			SpeedSetting::Paused { previous_speed } => previous_speed,
			SpeedSetting::UnPaused { speed } => speed,
		}
	}
}

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct SpeedButtons {
	base: Base<Control>,
	#[init(node = "pause")]
	pause_button: OnReady<Gd<Button>>,
	#[init(node = "pause/indicator")]
	pause_indicator: OnReady<Gd<Control>>,
	#[init(node = "speed_x1")]
	speed_x1_button: OnReady<Gd<Button>>,
	#[init(node = "speed_x1/indicator")]
	speed_x1_indicator: OnReady<Gd<Control>>,
	#[init(node = "speed_x2")]
	speed_x2_button: OnReady<Gd<Button>>,
	#[init(node = "speed_x2/indicator")]
	speed_x2_indicator: OnReady<Gd<Control>>,
	#[init(node = "speed_x3")]
	speed_x3_button: OnReady<Gd<Button>>,
	#[init(node = "speed_x3/indicator")]
	speed_x3_indicator: OnReady<Gd<Control>>,
	speed_setting: SpeedSetting,
}

#[godot_api]
impl IControl for SpeedButtons {
	fn ready(&mut self) {
		self.connect_with_deferred(&self.pause_button.clone(), "pressed", |this, _| {
			let speed = this.speed_setting.speed();
			this.set_speed_setting(SpeedSetting::Paused {
				previous_speed: speed,
			});
		});

		self.connect_with_deferred(&self.speed_x1_button.clone(), "pressed", |this, _| {
			this.set_speed(Speed::X1)
		});

		self.connect_with_deferred(&self.speed_x2_button.clone(), "pressed", |this, _| {
			this.set_speed(Speed::X2)
		});

		self.connect_with_deferred(&self.speed_x3_button.clone(), "pressed", |this, _| {
			this.set_speed(Speed::X3)
		});

		self.check_indicators();
	}
}

impl SpeedButtons {
	pub fn set_speed(&mut self, speed: Speed) {
		self.set_speed_setting(self.speed_setting.with_speed(speed));
	}

	pub fn set_speed_setting(&mut self, setting: SpeedSetting) {
		self.speed_setting = setting;
		self.check_indicators();
	}

	fn check_indicators(&mut self) {
		let speed_setting = self.speed_setting;
		let speed = speed_setting.speed();

		self.pause_indicator
			.set_visible(matches!(speed_setting, SpeedSetting::Paused { .. }));
		self.speed_x1_indicator.set_visible(speed == Speed::X1);
		self.speed_x2_indicator.set_visible(speed == Speed::X2);
		self.speed_x3_indicator.set_visible(speed == Speed::X3);
	}
}
