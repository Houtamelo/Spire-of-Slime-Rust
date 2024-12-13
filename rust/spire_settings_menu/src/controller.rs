use super::*;

#[derive(GodotClass)]
#[class(init, base = CanvasLayer)]
pub struct SettingsMenuController {
	base: Base<CanvasLayer>,

	#[init(node = "")]
	check_box_window_maximized: OnReady<Gd<CheckBox>>,
	#[init(node = "")]
	spin_box_window_size_x: OnReady<Gd<SpinBox>>,
	#[init(node = "")]
	spin_box_window_size_y: OnReady<Gd<SpinBox>>,
	#[init(node = "")]
	option_button_skill_overlay_mode: OnReady<Gd<OptionButton>>,
	#[init(node = "")]
	spin_box_skill_overlay_mode_auto_delay: OnReady<Gd<SpinBox>>,
	#[init(node = "")]
	option_button_language: OnReady<Gd<OptionButton>>,
	#[init(node = "")]
	spin_box_target_framerate: OnReady<Gd<SpinBox>>,
	#[init(node = "")]
	h_slider_dialogue_text_speed: OnReady<Gd<HSlider>>,
	#[init(node = "")]
	check_box_vsync: OnReady<Gd<CheckBox>>,
	#[init(node = "")]
	h_slider_main_volume: OnReady<Gd<HSlider>>,
	#[init(node = "")]
	h_slider_music_volume: OnReady<Gd<HSlider>>,
	#[init(node = "")]
	h_slider_sfx_volume: OnReady<Gd<HSlider>>,
	#[init(node = "")]
	h_slider_voice_volume: OnReady<Gd<HSlider>>,

	#[init(node = "")]
	button_confirm_changes: OnReady<Gd<Button>>,
	#[init(node = "")]
	button_undo_changes: OnReady<Gd<Button>>,

	#[init(node = "")]
	button_close_panel: OnReady<Gd<Button>>,
	#[init(node = "")]
	panel_on_close_confirm_or_undo: OnReady<Gd<Control>>,
	#[init(node = "")]
	button_on_close_confirm: OnReady<Gd<Button>>,
	#[init(node = "")]
	button_on_close_undo: OnReady<Gd<Button>>,

	#[init(node = "")]
	button_reset_settings: OnReady<Gd<Button>>,
	#[init(node = "")]
	panel_are_you_sure_reset: OnReady<Gd<PanelAreYouSure>>,

	saved_settings:   HashMap<GString, SettingsEnum>,
	unsaved_settings: HashMap<GString, SettingsEnum>,
}

#[godot_api]
impl ICanvasLayer for SettingsMenuController {
	fn ready(&mut self) {
		for setting in SettingsTable::default() {
			let key = setting.key();

			match setting {
				SettingsEnum::WindowMaximized(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut check_box = self.check_box_window_maximized.clone();
					check_box.set_pressed_no_signal(saved.0);

					self.connect_with_deferred(&check_box, "toggled", |this, args| {
						let Some(pressed) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Toggled signal did not provide bool argument")
						};

						let new_setting = WindowMaximized(pressed);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::WindowSize(default_size) => {
					let saved = {
						let mut temp = SettingsEnum::get_saved(&key, default_size);
						temp.0.x = i32::max(temp.0.x, 480);
						temp.0.y = i32::max(temp.0.y, 270);
						temp
					};

					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut spin_box_x = self.spin_box_window_size_x.clone();
					spin_box_x.set_value_no_signal(saved.0.x as f64);

					self.connect_with_deferred(&spin_box_x, "value_changed", |this, args| {
						let Some(new_x) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Spin box signal did not provide float argument")
						};

						let same_y = this.spin_box_window_size_y.get_value().round() as i32;
						let new_size = Vector2i::new(new_x, same_y);
						let new_setting = WindowSize(new_size);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});

					let mut spin_box_y = self.spin_box_window_size_y.clone();
					spin_box_y.set_value_no_signal(saved.0.y as f64);

					self.connect_with_deferred(&spin_box_y, "value_changed", |this, args| {
						let Some(new_y) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Spin box signal did not provide float argument")
						};

						let same_x = this.spin_box_window_size_x.get_value().round() as i32;
						let new_size = Vector2i::new(same_x, new_y);
						let new_setting = WindowSize(new_size);

						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::SkillOverlayModeSetting(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut options = self.option_button_skill_overlay_mode.clone();
					for overlay_mode in ALL_OVERLAY_MODES {
						options.add_item(overlay_mode.display_name());
					}

					options.select(saved.0.index());

					self.connect_with_deferred(&options, "item_selected", |this, args| {
						let Some(idx) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Option button signal did not provide int argument")
						};

						let overlay_mode = match idx {
							0 => {
								let delay_f64 =
									this.spin_box_skill_overlay_mode_auto_delay.get_value();
								let delay_ms = i64::clamp(delay_f64.round() as i64, 0, 10000);
								SkillOverlayMode::Auto { delay_ms }
							}
							1 => {
								this.spin_box_skill_overlay_mode_auto_delay
									.set_visible(false);
								SkillOverlayMode::WaitForInput
							}
							_ => {
								godot_error!("Invalid index for skill overlay mode: {idx}");
								return;
							}
						};

						let new_setting = SkillOverlayModeSetting(overlay_mode);

						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});

					let mut spin_box = self.spin_box_skill_overlay_mode_auto_delay.clone();
					match saved.0 {
						SkillOverlayMode::Auto { delay_ms } => {
							spin_box.set_visible(true);
							spin_box.set_value_no_signal(delay_ms as f64);
						}
						SkillOverlayMode::WaitForInput => {
							spin_box.set_visible(false);
						}
					}

					self.connect_with_deferred(&spin_box, "value_changed", |this, args| {
						let Some(delay_ms) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Spin box signal did not provide float argument")
						};

						let overlay_mode = SkillOverlayMode::Auto { delay_ms };
						let new_setting = SkillOverlayModeSetting(overlay_mode);

						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::LanguageSetting(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();
					self.apply_language(saved.0);

					let mut options = self.option_button_language.clone();
					for language in Language::iter() {
						options.add_item(&language.display_name());
					}

					options.set_block_signals(true);
					options.select(saved.0.index());
					options.set_block_signals(false);

					self.connect_with_deferred(&options, "item_selected", |this, args| {
						let Some(idx) = args.first().and_then(|arg| arg.try_to::<i32>().ok())
						else {
							return godot_error!("Option button signal did not provide int argument")
						};

						let Some(language) = Language::from_repr(idx)
						else { return godot_error!("Invalid index for language: {idx}") };

						let new_setting = LanguageSetting(language);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
						this.apply_language(language);
					});
				}
				SettingsEnum::MaxFps(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut spin_box = self.spin_box_target_framerate.clone();
					spin_box.set_value_no_signal(saved.0 as f64);

					self.connect_with_deferred(&spin_box, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Spin box signal did not provide float argument")
						};

						let new_setting = MaxFps(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::DialogueTextSpeed(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut h_slider = self.h_slider_dialogue_text_speed.clone();
					h_slider.set_value_no_signal(saved.0 as f64);

					self.connect_with_deferred(&h_slider, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("H slider signal did not provide float argument")
						};

						let new_setting = DialogueTextSpeed(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::Vsync(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut check_box = self.check_box_vsync.clone();
					check_box.set_pressed_no_signal(saved.0);

					self.connect_with_deferred(&check_box, "toggled", |this, args| {
						let Some(pressed) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("Toggled signal did not provide bool argument")
						};

						let new_setting = Vsync(pressed);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::MainVolume(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut h_slider = self.h_slider_main_volume.clone();
					h_slider.set_value(saved.0 as f64);

					self.connect_with_deferred(&h_slider, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("H slider signal did not provide float argument")
						};

						let new_setting = MainVolume(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::MusicVolume(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut h_slider = self.h_slider_music_volume.clone();
					h_slider.set_value(saved.0 as f64);

					self.connect_with_deferred(&h_slider, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("H slider signal did not provide float argument")
						};

						let new_setting = MusicVolume(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::SfxVolume(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut h_slider = self.h_slider_sfx_volume.clone();
					h_slider.set_value(saved.0 as f64);

					self.connect_with_deferred(&h_slider, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("H slider signal did not provide float argument")
						};

						let new_setting = SfxVolume(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
				SettingsEnum::VoiceVolume(default) => {
					let saved = SettingsEnum::get_saved(&key, default);
					self.saved_settings.insert(key, saved.into());
					saved.apply();

					let mut h_slider = self.h_slider_voice_volume.clone();
					h_slider.set_value(saved.0 as f64);

					self.connect_with_deferred(&h_slider, "value_changed", |this, args| {
						let Some(value) = args.first().and_then(|arg| arg.try_to().ok())
						else {
							return godot_error!("H slider signal did not provide float argument")
						};

						let new_setting = VoiceVolume(value);
						replace_setting(&mut this.unsaved_settings, new_setting);
						this.enable_dirty_changes_buttons(true);

						new_setting.apply();
					});
				}
			}
		}

		{
			let button = self.button_confirm_changes.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				this.confirm_changes();
			});
		}

		{
			let button = self.button_undo_changes.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				this.unsaved_settings.clear();
				this.enable_dirty_changes_buttons(false);
				this.apply_settings_no_signals();
				this.update_screen_settings();
			});
		}

		{
			let button = self.button_close_panel.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				if !this.unsaved_settings.is_empty() {
					this.panel_on_close_confirm_or_undo.show();
				} else {
					let mut base = this.base_mut();
					base.hide();
					base.emit_signal(Self::SIGNAL_PANEL_CLOSED, &[]);
				}
			});
		}

		{
			let button = self.button_on_close_confirm.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				this.confirm_changes();
				let mut base = this.base_mut();
				base.hide();
				base.emit_signal(Self::SIGNAL_PANEL_CLOSED, &[]);
			});
		}

		{
			let button = self.button_on_close_undo.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				this.unsaved_settings.clear();
				this.enable_dirty_changes_buttons(false);
				this.apply_settings_no_signals();
				this.update_screen_settings();

				let mut base = this.base_mut();
				base.hide();
				base.emit_signal(Self::SIGNAL_PANEL_CLOSED, &[]);
			});
		}

		{
			let button = self.button_reset_settings.clone();
			self.connect_with_deferred(&button, "pressed", |this, _| {
				this.panel_are_you_sure_reset.bind_mut().show();
			});
		}

		{
			let panel = self.panel_are_you_sure_reset.clone();
			self.connect_with_deferred(&panel, PanelAreYouSure::SIGNAL_YES, |this, _| {
				this.unsaved_settings.clear();
				this.enable_dirty_changes_buttons(false);

				let default_settings = SettingsTable::default().into_iter().map(|s| (s.key(), s));

				this.saved_settings.extend(default_settings);
				this.apply_settings_no_signals();
				this.update_screen_settings();
			});
		}
	}
}

#[godot_api]
impl SettingsMenuController {
	const SIGNAL_LANGUAGE_CHANGED: &'static str = "language_changed";
	const SIGNAL_PANEL_CLOSED: &'static str = "panel_closed";

	#[signal]
	pub fn language_changed(language: Language) {}

	#[signal]
	pub fn panel_closed() {}
}

impl SettingsMenuController {
	pub fn open_panel(&mut self) {
		self.update_screen_settings();
		self.base_mut().show();
	}

	fn apply_settings_no_signals(&mut self) {
		for &setting in self.saved_settings.values() {
			match setting {
				SettingsEnum::WindowSize(size) => {
					self.spin_box_window_size_x
						.set_value_no_signal(size.0.x as f64);
					self.spin_box_window_size_y
						.set_value_no_signal(size.0.y as f64);
				}
				SettingsEnum::MaxFps(rate) => {
					self.spin_box_target_framerate
						.set_value_no_signal(rate.0 as f64);
				}
				SettingsEnum::WindowMaximized(maximized) => {
					self.check_box_window_maximized
						.set_pressed_no_signal(maximized.0);
				}
				SettingsEnum::SkillOverlayModeSetting(mode) => {
					let option_button = &mut self.option_button_skill_overlay_mode;
					option_button.set_block_signals(true);
					option_button.select(mode.0.index());
					option_button.set_block_signals(false);

					let spin_box = &mut self.spin_box_skill_overlay_mode_auto_delay;
					match mode.0 {
						SkillOverlayMode::Auto { delay_ms } => {
							spin_box.set_visible(true);
							spin_box.set_value_no_signal(delay_ms as f64);
						}
						SkillOverlayMode::WaitForInput => {
							spin_box.set_visible(false);
						}
					}
				}
				SettingsEnum::LanguageSetting(language) => {
					let option_button = &mut self.option_button_language;
					option_button.set_block_signals(true);
					option_button.select(language.0.index());
					option_button.set_block_signals(false);

					self.base().clone().call_deferred(
						"emit_signal",
						&[
							Self::SIGNAL_LANGUAGE_CHANGED.to_variant(),
							language.to_variant(),
						],
					);
				}
				SettingsEnum::DialogueTextSpeed(speed) => {
					self.h_slider_dialogue_text_speed
						.set_value_no_signal(speed.0 as f64);
				}
				SettingsEnum::Vsync(vsync) => {
					self.check_box_vsync.set_pressed_no_signal(vsync.0);
				}
				SettingsEnum::MainVolume(main) => {
					self.h_slider_main_volume.set_value_no_signal(main.0 as f64);
				}
				SettingsEnum::MusicVolume(music) => {
					self.h_slider_music_volume
						.set_value_no_signal(music.0 as f64);
				}
				SettingsEnum::SfxVolume(sfx) => {
					self.h_slider_sfx_volume.set_value_no_signal(sfx.0 as f64);
				}
				SettingsEnum::VoiceVolume(voice) => {
					self.h_slider_voice_volume
						.set_value_no_signal(voice.0 as f64);
				}
			}

			setting.apply();
		}
	}

	fn update_screen_settings(&mut self) {
		{
			let is_maximized =
				DisplayServer::singleton().window_get_mode() == WindowMode::MAXIMIZED;

			self.check_box_window_maximized
				.set_pressed_no_signal(is_maximized);

			let new_setting = WindowMaximized(is_maximized);
			self.unsaved_settings.remove(&new_setting.key());
			replace_setting(&mut self.saved_settings, new_setting);
		}

		{
			let size = DisplayServer::singleton().window_get_size();

			self.spin_box_window_size_x
				.set_value_no_signal(size.x as f64);
			self.spin_box_window_size_y
				.set_value_no_signal(size.y as f64);

			let new_setting = WindowSize(size);
			self.unsaved_settings.remove(&new_setting.key());
			replace_setting(&mut self.saved_settings, new_setting);
		}

		{
			let max_fps = Engine::singleton().get_max_fps();

			self.spin_box_target_framerate
				.set_value_no_signal(max_fps as f64);

			let new_setting = MaxFps(max_fps);
			self.unsaved_settings.remove(&new_setting.key());
			replace_setting(&mut self.saved_settings, new_setting);
		}

		self.enable_dirty_changes_buttons(!self.unsaved_settings.is_empty());
	}

	fn enable_dirty_changes_buttons(&mut self, enable: bool) {
		self.button_confirm_changes.set_disabled(!enable);
		self.button_undo_changes.set_disabled(!enable);
	}

	fn apply_language(&mut self, language: Language) {
		self.base_mut().call_deferred(
			"emit_signal",
			&[
				Self::SIGNAL_LANGUAGE_CHANGED.to_variant(),
				language.to_variant(),
			],
		);
	}

	fn confirm_changes(&mut self) {
		for (key, unsaved_setting) in self.unsaved_settings.drain() {
			unsaved_setting.confirmed();
			self.saved_settings.insert(key, unsaved_setting);
		}

		write_settings_to_config(&mut self.unsaved_settings);
		self.enable_dirty_changes_buttons(false);
	}
}
