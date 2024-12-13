use super::*;

impl Setting for WindowMaximized {
	fn key(&self) -> GString { "window_maximized".into() }

	fn apply(&self) {
		let mode = if self.0 {
			WindowMode::MAXIMIZED
		} else {
			WindowMode::WINDOWED
		};

		DisplayServer::singleton().call_deferred("window_set_mode", &[mode.to_variant()]);
	}
}

impl Setting for WindowSize {
	fn key(&self) -> GString { "resolution".into() }

	fn confirmed(&self) {
		DisplayServer::singleton().call_deferred("window_set_size", &[self.0.to_variant()]);
	}
}

impl Setting for SkillOverlayModeSetting {
	fn key(&self) -> GString { "skill_overlay_mode".into() }
}

impl Setting for LanguageSetting {
	fn key(&self) -> GString { "language".into() }
}

impl Setting for MaxFps {
	fn key(&self) -> GString { "max_fps".into() }

	fn confirmed(&self) { Engine::singleton().set_max_fps(self.0); }
}

impl Setting for DialogueTextSpeed {
	fn key(&self) -> GString { "dialogue_text_speed".into() }
}

impl Setting for Vsync {
	fn key(&self) -> GString { "vsync".into() }

	fn apply(&self) {
		let mode = if self.0 {
			VSyncMode::ADAPTIVE
		} else {
			VSyncMode::DISABLED
		};

		DisplayServer::singleton().call_deferred("window_set_vsync_mode", &[mode.to_variant()]);
	}
}

impl Setting for MainVolume {
	fn key(&self) -> GString { "main_volume".into() }
	fn apply(&self) { apply_volume_percentage(self.0, BUS_MAIN); }
}

impl Setting for MusicVolume {
	fn key(&self) -> GString { "music_volume".into() }
	fn apply(&self) { apply_volume_percentage(self.0, BUS_MUSIC); }
}

impl Setting for SfxVolume {
	fn key(&self) -> GString { "sfx_volume".into() }
	fn apply(&self) { apply_volume_percentage(self.0, BUS_SFX); }
}

impl Setting for VoiceVolume {
	fn key(&self) -> GString { "voice_volume".into() }
	fn apply(&self) { apply_volume_percentage(self.0, BUS_VOICE); }
}

fn apply_volume_percentage(base100_percentage: i32, bus_name: &str) {
	let volume_db = volume_percentage_to_db(base100_percentage);
	let audio_server = &mut AudioServer::singleton();
	let bus_index = audio_server.get_bus_index(bus_name);
	audio_server.set_bus_volume_db(bus_index, volume_db as f32);
}

fn volume_percentage_to_db(base100_percentage: i32) -> f64 {
	if base100_percentage <= 0 {
		-80.0
	} else {
		let base1_percentage = f64::clamp(base100_percentage as f64, 0.0, 100.0) / 100.0;
		20.0 * base1_percentage.log10()
	}
}

impl GodotConvert for LanguageSetting {
	type Via = <Language as GodotConvert>::Via;
}

impl ToGodot for LanguageSetting {
	type ToVia<'v> = Self::Via;

	fn to_godot(&self) -> Self::Via { self.0.to_godot() }
}

impl FromGodot for LanguageSetting {
	fn try_from_godot(v: Self::Via) -> Result<Self, ConvertError> {
		let v = Language::try_from_godot(v)?;
		Ok(LanguageSetting(v))
	}
}

impl GodotConvert for SkillOverlayModeSetting {
	type Via = <SkillOverlayMode as GodotConvert>::Via;
}

impl ToGodot for SkillOverlayModeSetting {
	type ToVia<'v> = Self::Via;

	fn to_godot(&self) -> Self::Via { self.0.to_godot() }
}

impl FromGodot for SkillOverlayModeSetting {
	fn try_from_godot(v: Self::Via) -> Result<Self, ConvertError> {
		let v = SkillOverlayMode::try_from_godot(v)?;
		Ok(SkillOverlayModeSetting(v))
	}
}

macro_rules! impl_godot_convert {
    (
	    $( $Setting: ident => $Via: ty ),*
	    $(,)?
    ) => {
	    $(
		    impl GodotConvert for $Setting {
				type Via = $Via;
			}

			impl ToGodot for $Setting {
			    type ToVia<'v> = Self::Via;

				fn to_godot(&self) -> Self::Via {
					self.0
				}
			}

			impl FromGodot for $Setting {
				fn try_from_godot(v: Self::Via) -> Result<Self, ConvertError> {
					Ok($Setting(v))
				}
			}
	    )*
    };
}

impl_godot_convert! {
	WindowMaximized => bool,
	WindowSize => Vector2i,
	MaxFps => i32,
	DialogueTextSpeed => i32,
	Vsync => bool,
	MainVolume => i32,
	MusicVolume => i32,
	SfxVolume => i32,
	VoiceVolume => i32,
}
