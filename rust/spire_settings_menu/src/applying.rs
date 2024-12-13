use super::*;

const CFG_SECTION: &str = "player_prefs";

pub fn write_settings_to_config(settings: &mut HashMap<GString, SettingsEnum>) {
	let mut config = ConfigFile::new_gd();
	config.load(shared::CONFIG_PATH);

	for (key, setting) in settings.iter() {
		config.set_value(CFG_SECTION, key, &setting.to_variant());
	}

	match config.save(shared::CONFIG_PATH) {
		godot::global::Error::OK => {
			settings.clear();
		}
		err => {
			godot_warn!("Failed to save config file. \nError: {err:?}");
		}
	}
}

pub fn replace_setting(
	settings: &mut HashMap<GString, SettingsEnum>,
	new_setting: impl Into<SettingsEnum>,
) {
	let new_setting = new_setting.into();
	let key = new_setting.key();
	settings.insert(key, new_setting);
}
