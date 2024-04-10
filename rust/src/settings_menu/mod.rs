#[allow(unused_imports)]
use crate::*;

use settings::*;
use crate::misc::panel_are_you_sure;
use crate::misc::panel_are_you_sure::PanelAreYouSure;

mod settings;

pub const SIGNAL_LANGUAGE_CHANGED: &str = "language_changed";
pub(super) const SIGNAL_PANEL_CLOSED: &str = "panel_closed";
pub(super) const CALL_OPEN_PANEL: &str = "_open_panel";

#[extends(CanvasLayer)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct SettingsMenuController {
    #[export_path] check_box_window_maximized: Option<Ref<CheckBox>>,
    #[export_path] spin_box_window_size_x    : Option<Ref<SpinBox>>,
    #[export_path] spin_box_window_size_y    : Option<Ref<SpinBox>>,
    #[export_path] option_button_skill_overlay_mode: Option<Ref<OptionButton>>,
    #[export_path] spin_box_skill_overlay_mode_auto_delay: Option<Ref<SpinBox>>,
    #[export_path] option_button_language      : Option<Ref<OptionButton>>,
    #[export_path] spin_box_target_framerate   : Option<Ref<SpinBox>>,
    #[export_path] h_slider_dialogue_text_speed: Option<Ref<HSlider>>,
    #[export_path] check_box_vsync: Option<Ref<CheckBox>>,
    #[export_path] h_slider_main_volume : Option<Ref<HSlider>>,
    #[export_path] h_slider_music_volume: Option<Ref<HSlider>>,
    #[export_path] h_slider_sfx_volume  : Option<Ref<HSlider>>,
    #[export_path] h_slider_voice_volume: Option<Ref<HSlider>>,

    #[export_path] button_confirm_changes: Option<Ref<Button>>,
    #[export_path] button_undo_changes: Option<Ref<Button>>,

    #[export_path] button_close_panel: Option<Ref<Button>>,
    #[export_path] panel_on_close_confirm_or_undo: Option<Ref<Control>>,
    #[export_path] button_on_close_confirm: Option<Ref<Button>>,
    #[export_path] button_on_close_undo: Option<Ref<Button>>,

    #[export_path] button_reset_settings : Option<Ref<Button>>,
    #[export_path] panel_are_you_sure_reset: Option<Instance<PanelAreYouSure>>,

    saved_settings  : HashMap<&'static str, GameSetting>,
    unsaved_settings: HashMap<&'static str, GameSetting>,
}

#[methods]
impl SettingsMenuController {
	fn register(builder: &ClassBuilder<Self>) {
        builder.signal(SIGNAL_LANGUAGE_CHANGED).with_param("language", VariantType::Object).done();
        builder.signal(SIGNAL_PANEL_CLOSED).done();
    }

	#[method]
	fn _ready(&mut self, #[base] _owner: &CanvasLayer) {
		self.grab_nodes_by_path(_owner);
        let owner_ref = unsafe { _owner.assume_shared() };

		for setting in DEFAULT_SETTINGS {
            let key = setting.key();

			match setting {
				GameSetting::WindowMaximized(_) => {
                    let GameSetting::WindowMaximized(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_maximized = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::WindowMaximized(saved_maximized));

                    let check_box = self.check_box_window_maximized.unwrap_manual();
                    check_box.set_pressed(saved_maximized);
                    check_box.connect("toggled", owner_ref, "_on_check_box_window_maximized", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

                    let os = OS::godot_singleton();
                    os.set_window_maximized(saved_maximized);
                },
				GameSetting::WindowSize(_, _) => {
                    let GameSetting::WindowSize(default_x, default_y) = GameSetting::default_value(setting) else { unreachable!() };
                    let (mut saved_size_x, mut saved_size_y) = GameSetting::get_saved(key, (default_x, default_y));
                    saved_size_x = i32::max(saved_size_x, 480);
                    saved_size_y = i32::max(saved_size_y, 270);
                    self.saved_settings.insert(key, GameSetting::WindowSize(saved_size_x, saved_size_y));

                    let spin_box_x = self.spin_box_window_size_x.unwrap_manual();
                    spin_box_x.set_value(saved_size_x as f64);
                    spin_box_x.connect("value_changed", owner_ref, "_on_spin_box_window_size_x", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                    let spin_box_y = self.spin_box_window_size_y.unwrap_manual();
                    spin_box_y.set_value(saved_size_y as f64);
                    spin_box_y.connect("value_changed", owner_ref, "_on_spin_box_window_size_y", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

                    let os = OS::godot_singleton();
                    os.set_window_size(Vector2 { x: saved_size_x as f32, y: saved_size_y as f32 });
                },
				GameSetting::SkillOverlayMode(_) => {
                    let GameSetting::SkillOverlayMode(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_mode = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::SkillOverlayMode(saved_mode.clone()));

                    let option_button = self.option_button_skill_overlay_mode.unwrap_manual();
                    for overlay_mode in settings::ALL_OVERLAY_MODES {
                        option_button.add_item(overlay_mode.display_name(), overlay_mode.index());
                    }

                    option_button.select(saved_mode.index());
                    option_button.connect("item_selected", owner_ref, "_on_option_button_skill_overlay_mode", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

                    let spin_box = self.spin_box_skill_overlay_mode_auto_delay.unwrap_manual();
                    match saved_mode {
                        SkillOverlayMode::Auto { delay_ms } => {
                            spin_box.set_visible(true);
                            spin_box.set_value(delay_ms as f64);
                        },
                        SkillOverlayMode::WaitForInput => {
                            spin_box.set_visible(false);
                        },
                    }
                    spin_box.connect("value_changed", owner_ref, "_on_spin_box_skill_overlay_mode_auto_delay", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::Language(_) => {
                    let GameSetting::Language(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_language = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::Language(saved_language));

                    let option_button = self.option_button_language.unwrap_manual();
                    for language in settings::ALL_LANGUAGES {
                        option_button.add_item(language.display_name(), language.index());
                    }

                    option_button.select(saved_language.index());
                    option_button.connect("item_selected", owner_ref, "_on_option_button_language", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::TargetFramerate(_) => {
                    let GameSetting::TargetFramerate(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_framerate = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::TargetFramerate(saved_framerate));

                    let spin_box = self.spin_box_target_framerate.unwrap_manual();
                    spin_box.set_value(saved_framerate as f64);
                    spin_box.connect("value_changed", owner_ref, "_on_spin_box_target_framerate", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::DialogueTextSpeed(_) => {
                    let GameSetting::DialogueTextSpeed(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_speed = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::DialogueTextSpeed(saved_speed));

                    let h_slider = self.h_slider_dialogue_text_speed.unwrap_manual();
                    h_slider.set_value(saved_speed as f64);
                    h_slider.connect("value_changed", owner_ref, "_on_h_slider_dialogue_text_speed", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::Vsync(_) => {
                    let GameSetting::Vsync(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_vsync = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::Vsync(saved_vsync));

                    let check_box = self.check_box_vsync.unwrap_manual();
                    check_box.set_pressed(saved_vsync);
                    check_box.connect("toggled", owner_ref, "_on_check_box_vsync", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

                    let os = OS::godot_singleton();
                    os.set_use_vsync(saved_vsync);
                },
				GameSetting::MainVolume(_) => {
                    let GameSetting::MainVolume(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_volume = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::MainVolume(saved_volume));

                    let h_slider = self.h_slider_main_volume.unwrap_manual();
                    h_slider.set_value(saved_volume as f64);
                    h_slider.connect("value_changed", owner_ref, "_on_h_slider_main_volume", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::MusicVolume(_) => {
                    let GameSetting::MusicVolume(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_volume = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::MusicVolume(saved_volume));

                    let h_slider = self.h_slider_music_volume.unwrap_manual();
                    h_slider.set_value(saved_volume as f64);
                    h_slider.connect("value_changed", owner_ref, "_on_h_slider_music_volume", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::SfxVolume(_) => {
                    let GameSetting::SfxVolume(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_volume = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::SfxVolume(saved_volume));

                    let h_slider = self.h_slider_sfx_volume.unwrap_manual();
                    h_slider.set_value(saved_volume as f64);
                    h_slider.connect("value_changed", owner_ref, "_on_h_slider_sfx_volume", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
				GameSetting::VoiceVolume(_) => {
                    let GameSetting::VoiceVolume(default) = GameSetting::default_value(setting) else { unreachable!() };
                    let saved_volume = GameSetting::get_saved(key, default);
                    self.saved_settings.insert(key, GameSetting::VoiceVolume(saved_volume));

                    let h_slider = self.h_slider_voice_volume.unwrap_manual();
                    h_slider.set_value(saved_volume as f64);
                    h_slider.connect("value_changed", owner_ref, "_on_h_slider_voice_volume", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
                },
			}
		}

        self.button_confirm_changes.unwrap_manual().connect("pressed", owner_ref, "_on_button_confirm_changes", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
        self.button_undo_changes   .unwrap_manual().connect("pressed", owner_ref, "_on_button_undo_changes"   , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

        self.button_close_panel     .unwrap_manual().connect("pressed", owner_ref, "_on_button_close_panel"        , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
        self.button_on_close_confirm.unwrap_manual().connect("pressed", owner_ref, "_on_button_close_panel_confirm", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
        self.button_on_close_undo   .unwrap_manual().connect("pressed", owner_ref, "_on_button_close_panel_undo"   , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

        self.button_reset_settings.unwrap_manual().connect("pressed", owner_ref, "_on_button_reset_settings", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
        self.panel_are_you_sure_reset.unwrap_inst().base().connect(panel_are_you_sure::SIGNAL_YES, owner_ref, "_on_panel_are_you_sure_reset_yes", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
    }
    
    #[method]
    pub fn _open_panel(&mut self, #[base] owner: &CanvasLayer) {
        self.update_screen_settings();
        owner.show();
    }

    #[method]
    fn _on_check_box_window_maximized(&mut self, button_pressed: bool) {
        let new_setting = GameSetting::WindowMaximized(button_pressed);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
        OS::godot_singleton().set_deferred("window_maximized", button_pressed);
    }

    #[method]
    fn _on_spin_box_window_size_x(&mut self, value: f64) {
        let new_setting = GameSetting::WindowSize(value as i32, self.spin_box_window_size_y.unwrap_manual().value() as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_spin_box_window_size_y(&mut self, value: f64) {
        let new_setting = GameSetting::WindowSize(self.spin_box_window_size_x.unwrap_manual().value() as i32, value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_option_button_skill_overlay_mode(&mut self, index: i64) {
        let new_setting;
        match index {
            0 => {
                let spin_box = self.spin_box_skill_overlay_mode_auto_delay.unwrap_manual();
                let delay_ms = i64::clamp(spin_box.value() as i64, 0, 10000);
                new_setting = GameSetting::SkillOverlayMode(SkillOverlayMode::Auto { delay_ms });

                spin_box.set_visible(true);
                spin_box.set_block_signals(true);
                spin_box.set_value(delay_ms as f64);
                spin_box.set_block_signals(false);
            },
            1 => {
                new_setting = GameSetting::SkillOverlayMode(SkillOverlayMode::WaitForInput);
                self.spin_box_skill_overlay_mode_auto_delay.unwrap_manual().set_visible(false);
            },
            _ => {
                godot_error!("Invalid index for skill overlay mode: {}", index);
                return;
            },
        }

        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_spin_box_skill_overlay_mode_auto_delay(&mut self, value: f64) {
        let new_setting = GameSetting::SkillOverlayMode(SkillOverlayMode::Auto { delay_ms: value as i64 });
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_option_button_language(&mut self, #[base] owner: &CanvasLayer, index: i64) {
        let language = settings::ALL_LANGUAGES[index as usize];
        let new_setting = GameSetting::Language(language.clone());
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);

        owner.emit_signal(SIGNAL_LANGUAGE_CHANGED, &[language.to_variant()]);
    }

    #[method]
    fn _on_spin_box_target_framerate(&mut self, value: f64) {
        let new_setting = GameSetting::TargetFramerate(value as i64);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_h_slider_dialogue_text_speed(&mut self, value: f64) {
        let new_setting = GameSetting::DialogueTextSpeed(value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
    }

    #[method]
    fn _on_check_box_vsync(&mut self, button_pressed: bool) {
        let new_setting = GameSetting::Vsync(button_pressed);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);
        OS::godot_singleton().set_deferred("use_vsync", button_pressed);
    }

    #[method]
    fn _on_h_slider_main_volume(&mut self, value: f64) {
        let new_setting = GameSetting::MainVolume(value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);

        SettingsMenuController::set_volume_percentage(value as i32, crate::bus::bus_name_main);
    }

    #[method]
    fn _on_h_slider_music_volume(&mut self, value: f64) {
        let new_setting = GameSetting::MusicVolume(value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);

        SettingsMenuController::set_volume_percentage(value as i32, crate::bus::bus_name_music);
    }

    #[method]
    fn _on_h_slider_sfx_volume(&mut self, value: f64) {
        let new_setting = GameSetting::SfxVolume(value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);

        SettingsMenuController::set_volume_percentage(value as i32, crate::bus::bus_name_sfx);
    }

    #[method]
    fn _on_h_slider_voice_volume(&mut self, value: f64) {
        let new_setting = GameSetting::VoiceVolume(value as i32);
        SettingsMenuController::replace_setting(&mut self.unsaved_settings, new_setting);
        self.enable_dirty_changes_buttons(true);

        SettingsMenuController::set_volume_percentage(value as i32, crate::bus::bus_name_voice);
    }

    #[method]
    fn _on_button_confirm_changes(&mut self) {
        for unsaved_setting in self.unsaved_settings.values() {
            match unsaved_setting {
                GameSetting::WindowMaximized(_) | GameSetting::SkillOverlayMode(_) | GameSetting::Language(_) | GameSetting::DialogueTextSpeed(_)
                | GameSetting::Vsync(_) | GameSetting::MainVolume(_) | GameSetting::MusicVolume(_) | GameSetting::SfxVolume(_) | GameSetting::VoiceVolume(_)
                => {},
                GameSetting::WindowSize(x, y) => {
                    let os = OS::godot_singleton();
                    os.set_window_size(Vector2 { x: *x as f32, y: *y as f32 });
                },
                GameSetting::TargetFramerate(rate) => {
                    let engine = Engine::godot_singleton();
                    engine.set_target_fps(*rate);
                },
            }

            self.saved_settings.insert(unsaved_setting.key(), unsaved_setting.clone());
        }

        SettingsMenuController::write_settings_to_config(&mut self.unsaved_settings);
        self.unsaved_settings.clear();
        self.enable_dirty_changes_buttons(false);
    }

    #[method]
    fn _on_button_undo_changes(&mut self, #[base] _owner: &CanvasLayer) {
        self.unsaved_settings.clear();
        self.enable_dirty_changes_buttons(false);

        self.apply_settings_no_signals(_owner, &self.saved_settings);

        self.update_screen_settings();
    }

    #[method]
    fn _on_button_close_panel(&mut self, #[base] _owner: &CanvasLayer) {
        if self.unsaved_settings.len() > 0 {
            self.panel_on_close_confirm_or_undo.unwrap_manual().show();
        } else {
            _owner.hide();
            _owner.emit_signal(SIGNAL_PANEL_CLOSED, &[]);
        }
    }

    #[method]
    fn _on_button_close_panel_confirm(&mut self, #[base] _owner: &CanvasLayer) {
        for unsaved_setting in self.unsaved_settings.values() {
            match unsaved_setting {
                GameSetting::WindowMaximized(_) | GameSetting::SkillOverlayMode(_) | GameSetting::Language(_) | GameSetting::DialogueTextSpeed(_)
                | GameSetting::Vsync(_) | GameSetting::MainVolume(_) | GameSetting::MusicVolume(_) | GameSetting::SfxVolume(_) | GameSetting::VoiceVolume(_)
                => {},
                GameSetting::WindowSize(x, y) => {
                    let os = OS::godot_singleton();
                    os.set_window_size(Vector2 { x: *x as f32, y: *y as f32 });
                },
                GameSetting::TargetFramerate(rate) => {
                    let engine = Engine::godot_singleton();
                    engine.set_target_fps(*rate);
                },
            }

            self.saved_settings.insert(unsaved_setting.key(), unsaved_setting.clone());
        }

        SettingsMenuController::write_settings_to_config(&mut self.unsaved_settings);
        self.unsaved_settings.clear();
        self.enable_dirty_changes_buttons(false);

        _owner.hide();
        _owner.emit_signal(SIGNAL_PANEL_CLOSED, &[]);
    }

    #[method]
    fn _on_button_close_panel_undo(&mut self, #[base] _owner: &CanvasLayer) {
        self.unsaved_settings.clear();
        self.enable_dirty_changes_buttons(false);

        self.apply_settings_no_signals(_owner, &self.saved_settings);

        self.update_screen_settings();

        _owner.hide();
        _owner.emit_signal(SIGNAL_PANEL_CLOSED, &[]);
    }

    #[method]
    fn _on_button_reset_settings(&mut self, #[base] _owner: &CanvasLayer) {
        self.panel_are_you_sure_reset.unwrap_inst().base().show();
    }

    #[method]
    fn _on_panel_are_you_sure_reset_yes(&mut self, #[base] _owner: &CanvasLayer) {
        self.unsaved_settings.clear();
        self.enable_dirty_changes_buttons(false);

        self.saved_settings.extend(settings::DEFAULT_SETTINGS.iter().map(|setting| (setting.key(), setting.clone())));
        self.apply_settings_no_signals(_owner, &self.saved_settings);

        self.update_screen_settings();
    }

    fn apply_settings_no_signals(&self, _owner: &CanvasLayer, settings: &HashMap<&'static str, GameSetting>) {
        for setting in settings.values() {
            match setting {
                GameSetting::WindowSize(x, y) => {
                    let spin_box_x = self.spin_box_window_size_x.unwrap_manual();
                    spin_box_x.set_block_signals(true);
                    spin_box_x.set_value(*x as f64);
                    spin_box_x.set_block_signals(false);

                    let spin_box_y = self.spin_box_window_size_y.unwrap_manual();
                    spin_box_y.set_block_signals(true);
                    spin_box_y.set_value(*y as f64);
                    spin_box_y.set_block_signals(false);

                    let os = OS::godot_singleton();
                    os.set_window_size(Vector2 { x: *x as f32, y: *y as f32 });
                },
                GameSetting::TargetFramerate(rate) => {
                    let spin_box = self.spin_box_target_framerate.unwrap_manual();
                    spin_box.set_block_signals(true);
                    spin_box.set_value(*rate as f64);
                    spin_box.set_block_signals(false);

                    let engine = Engine::godot_singleton();
                    engine.set_target_fps(*rate);
                },
                GameSetting::WindowMaximized(maximized) => {
                    let check_box = self.check_box_window_maximized.unwrap_manual();
                    check_box.set_block_signals(true);
                    check_box.set_pressed(*maximized);
                    check_box.set_block_signals(false);
                    
                    OS::godot_singleton().set_deferred("window_maximized", *maximized);
                },
                GameSetting::SkillOverlayMode(mode) => {
                    let option_button = self.option_button_skill_overlay_mode.unwrap_manual();
                    option_button.set_block_signals(true);
                    option_button.select(mode.index());
                    option_button.set_block_signals(false);

                    let spin_box = self.spin_box_skill_overlay_mode_auto_delay.unwrap_manual();
                    match mode {
                        SkillOverlayMode::Auto { delay_ms } => {
                            spin_box.set_visible(true);
                            spin_box.set_block_signals(true);
                            spin_box.set_value(*delay_ms as f64);
                            spin_box.set_block_signals(false);
                        },
                        SkillOverlayMode::WaitForInput => {
                            spin_box.set_visible(false);
                        },
                    }
                },
                GameSetting::Language(language) => {
                    let option_button = self.option_button_language.unwrap_manual();
                    option_button.set_block_signals(true);
                    option_button.select(language.index());
                    option_button.set_block_signals(false);

                    _owner.emit_signal(SIGNAL_LANGUAGE_CHANGED, &[language.to_variant()]);
                },
                GameSetting::DialogueTextSpeed(speed) => {
                    let h_slider = self.h_slider_dialogue_text_speed.unwrap_manual();
                    h_slider.set_block_signals(true);
                    h_slider.set_value(*speed as f64);
                    h_slider.set_block_signals(false);
                },
                GameSetting::Vsync(vsync) => {
                    let check_box = self.check_box_vsync.unwrap_manual();
                    check_box.set_block_signals(true);
                    check_box.set_pressed(*vsync);
                    check_box.set_block_signals(false);

                    OS::godot_singleton().set_deferred("use_vsync", *vsync);
                },
                GameSetting::MainVolume(volume) => {
                    let h_slider = self.h_slider_main_volume.unwrap_manual();
                    h_slider.set_block_signals(true);
                    h_slider.set_value(*volume as f64);
                    h_slider.set_block_signals(false);

                    SettingsMenuController::set_volume_percentage(*volume, crate::bus::bus_name_main);
                },
                GameSetting::MusicVolume(volume) => {
                    let h_slider = self.h_slider_music_volume.unwrap_manual();
                    h_slider.set_block_signals(true);
                    h_slider.set_value(*volume as f64);
                    h_slider.set_block_signals(false);

                    SettingsMenuController::set_volume_percentage(*volume, crate::bus::bus_name_music);
                },
                GameSetting::SfxVolume(volume) => {
                    let h_slider = self.h_slider_sfx_volume.unwrap_manual();
                    h_slider.set_block_signals(true);
                    h_slider.set_value(*volume as f64);
                    h_slider.set_block_signals(false);

                    SettingsMenuController::set_volume_percentage(*volume, crate::bus::bus_name_sfx);
                },
                GameSetting::VoiceVolume(volume) => {
                    let h_slider = self.h_slider_voice_volume.unwrap_manual();
                    h_slider.set_block_signals(true);
                    h_slider.set_value(*volume as f64);
                    h_slider.set_block_signals(false);

                    SettingsMenuController::set_volume_percentage(*volume, crate::bus::bus_name_voice);
                },
            }
        }
    }

    fn update_screen_settings(&mut self) {
        let os = OS::godot_singleton();

        {
            let window_maximized = os.is_window_maximized();

            let check_box = self.check_box_window_maximized.unwrap_manual();
            check_box.set_block_signals(true);
            check_box.set_pressed(window_maximized);
            check_box.set_block_signals(false);

            let new_setting = GameSetting::WindowMaximized(window_maximized);
            self.unsaved_settings.remove(new_setting.key());
            SettingsMenuController::replace_setting(&mut self.saved_settings, new_setting);
        }

        {
            let window_size = os.window_size();

            let spin_box_x = self.spin_box_window_size_x.unwrap_manual();
            spin_box_x.set_block_signals(true);
            spin_box_x.set_value(window_size.x as f64);
            spin_box_x.set_block_signals(false);

            let spin_box_y = self.spin_box_window_size_y.unwrap_manual();
            spin_box_y.set_block_signals(true);
            spin_box_y.set_value(window_size.y as f64);
            spin_box_y.set_block_signals(false);

            let new_setting = GameSetting::WindowSize(window_size.x as i32, window_size.y as i32);
            self.unsaved_settings.remove(new_setting.key());
            SettingsMenuController::replace_setting(&mut self.saved_settings, new_setting);
        }

        {
            let target_framerate = Engine::godot_singleton().target_fps();

            let spin_box = self.spin_box_target_framerate.unwrap_manual();
            spin_box.set_block_signals(true);
            spin_box.set_value(target_framerate as f64);
            spin_box.set_block_signals(false);

            let new_setting = GameSetting::TargetFramerate(target_framerate);
            self.unsaved_settings.remove(new_setting.key());
            SettingsMenuController::replace_setting(&mut self.saved_settings, new_setting);
        }

        self.enable_dirty_changes_buttons(self.unsaved_settings.len() > 0);
    }

    fn enable_dirty_changes_buttons(&self, enable: bool) {
        self.button_confirm_changes.unwrap_manual().set_disabled(!enable);
        self.button_undo_changes.unwrap_manual().set_disabled(!enable);
    }

    fn replace_setting(settings: &mut HashMap<&'static str, GameSetting>, new_setting: GameSetting) {
        let key = new_setting.key();
        settings.insert(key, new_setting);
    }

    fn write_settings_to_config(settings: &mut HashMap<&'static str, GameSetting>) {
        let config = ConfigFile::new();
        if let Err(error) = config.load(crate::CONFIG_PATH) {
            godot_warn!("Failed to load config file: {}", error);
        }

        for (key, setting) in settings.iter() {
            config.set_value("player_prefs", key, setting);
        }

        if let Err(error) = config.save(crate::CONFIG_PATH) {
            godot_warn!("Failed to load config file: {}", error);
        } else {
            settings.clear();
        }
    }

    fn volume_percentage_to_db(base100_percentage: i32) -> f64 {
        if base100_percentage <= 0 {
            return -80.0;
        }

        let base1_percentage = f64::clamp(base100_percentage as f64, 0.0, 100.0) / 100.0;
        return 20.0 * base1_percentage.log10();
    }

    fn set_volume_percentage(base100_percentage: i32, bus_name: &str) {
        let volume_db = SettingsMenuController::volume_percentage_to_db(base100_percentage);
        let audio_server = AudioServer::godot_singleton();
        let bus_index = audio_server.get_bus_index(bus_name);
        audio_server.set_bus_volume_db(bus_index, volume_db);
    }
}
