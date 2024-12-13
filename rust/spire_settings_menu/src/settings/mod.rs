use super::*;

mod language;
mod skill_overlay_mode;
mod trait_impls;

pub use language::*;
pub use skill_overlay_mode::*;
#[allow(unused_imports)]
pub use trait_impls::*;

declarative_type_state::type_table! {
	ENUM_OUT: {
		#[vars(derive(Debug, Clone, Copy))]
		#[derive(Clone, Copy)]
		pub enum SettingsEnum {
			WindowMaximized(bool),
			WindowSize(Vector2i),
			SkillOverlayModeSetting(SkillOverlayMode),
			LanguageSetting(Language),
			MaxFps(i32),
			DialogueTextSpeed(i32),
			Vsync(bool),
			MainVolume(i32),
			MusicVolume(i32),
			SfxVolume(i32),
			VoiceVolume(i32),
		}
	}

	TABLE: {
		pub struct SettingsTable;
	}

	DELEGATES: {
		impl trait Setting {
			[fn key(&self) -> GString]
			[fn apply(&self)]
		}

		impl {
			[pub fn to_variant(&self) -> Variant]
		}
	}
}

pub trait Setting {
	fn key(&self) -> GString;

	fn apply(&self) {}

	fn confirmed(&self) {}
}

impl Default for SettingsTable {
	fn default() -> Self {
		SettingsTable::new(
			WindowMaximized(false),
			WindowSize(Vector2i::new(1280, 720)),
			SkillOverlayModeSetting(SkillOverlayMode::Auto { delay_ms: 3000 }),
			LanguageSetting(Language::English),
			MaxFps(60),
			DialogueTextSpeed(100),
			Vsync(true),
			MainVolume(50),
			MusicVolume(50),
			SfxVolume(50),
			VoiceVolume(50),
		)
	}
}

impl SettingsEnum {
	pub fn get_saved<T: Setting + FromGodot>(key: &GString, default: T) -> T {
		let mut config = ConfigFile::new_gd();
		match config.load(shared::CONFIG_PATH) {
			godot::global::Error::OK => {}
			err => {
				godot_warn!(
					"Failed to load config file.\n\
				     Error: {err:?}"
				);
				return default;
			}
		}

		let result = config.get_value("player_prefs", key);
		match result.try_to::<T>() {
			Ok(ok) => ok,
			Err(error) => {
				let type_name = type_name::<T>();
				godot_warn!(
					"Failed converting: {result} into {type_name}, in setting: {key}, returning default.\n\
	                 Error: {error}"
				);

				default
			}
		}
	}
}
