use super::*;

#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct MainMenuController {
	base: Base<Control>,

	#[init(node = "panel_new-game")]
	panel_new_game:  OnReady<Gd<Control>>,
	#[init(node = "panel_load_game")]
	panel_load_game: OnReady<Gd<Control>>,
	#[init(node = "panel_credits")]
	panel_credits:   OnReady<Gd<Control>>,

	// Panel - NEW GAME
	#[init(node = "panel_new-game/background_easter-egg/label")]
	label_easter_egg: OnReady<Gd<Label>>,
	#[init(node = "panel_new-game/line-edit_save-name")]
	line_edit_save_name: OnReady<Gd<LineEdit>>,
	#[init(node = "panel_new-game/panel_are-you-sure-overwrite-save")]
	panel_are_you_sure_overwrite_save: OnReady<Gd<PanelAreYouSure>>,

	// Panel - LOAD GAME
	#[init(node = "panel_load-game/scroll-container_load-game/v_box_container")]
	container_load_buttons: OnReady<Gd<Control>>,
	// Todo: add path to load button prefab
	#[init(val = OnReady::new(|| ResourceLoader::singleton().load("").unwrap().cast()))] //todo!
	prefab_load_button: OnReady<Gd<PackedScene>>,
	iron_gauntlet_times_pressed: u64,
	tween_easter_egg_text: Option<SpireHandle<Property<f64>>>,
	spawned_load_buttons: Vec<Gd<LoadButton>>,
}

#[godot_api]
impl IControl for MainMenuController {
	fn ready(&mut self) {
		self.connect_child("background", "gui_input", |this, args| {
			let Some(event) = args
				.first()
				.and_then(|arg| arg.try_to::<Gd<InputEvent>>().ok())
			else {
				return godot_error!("GuiInput signal did not provide InputEvent argument")
			};

			let Some(mouse_event) = event.try_cast::<InputEventMouseButton>().ok()
			else { return };

			if !mouse_event.is_echo()
				&& mouse_event.is_pressed()
				&& mouse_event.get_button_index() == godot::global::MouseButton::LEFT
			{
				this.close_all_panels();
			}
		})
		.log_if_err();

		self.connect_child("panel_credits/button_return", "pressed", |this, _| {
			this.panel_credits.hide();
		})
		.log_if_err();

		self.connect_child("button_discord", "pressed", |_, _| {
			Os::singleton().shell_open("https://discord.gg/Cacam7yuqR");
		})
		.log_if_err();

		self.connect_child("button_subscribe-star", "pressed", |_, _| {
			Os::singleton().shell_open("https://subscribestar.adult/spire-of-slime-yonder");
		})
		.log_if_err();

		// Main Buttons
		{
			self.connect_child("main-buttons/new-game", "pressed", |this, _| {
				let visible = !this.panel_new_game.is_visible();
				this.panel_new_game.set_visible(visible);
			})
			.log_if_err();

			self.connect_child("main-buttons/load-game", "pressed", |this, _| {
				let visible = !this.panel_load_game.is_visible();
				this.panel_load_game.set_visible(visible)
			})
			.log_if_err();

			self.connect_child("main-buttons/settings", "pressed", |this, _| {
				this.base_mut()
					.emit_signal(Self::SIGNAL_OPEN_SETTINGS_MENU, &[]);
			})
			.log_if_err();

			self.connect_child("main-buttons/credits", "pressed", |this, _| {
				let visible = !this.panel_credits.is_visible();
				this.panel_credits.set_visible(visible);
			})
			.log_if_err();

			self.connect_child("main-buttons/quit", "pressed", |this, _| {
				this.base().get_tree().unwrap().quit();
			})
			.log_if_err();
		}

		// Panel - NEW GAME
		{
			self.connect_child("panel_new-game/button_start-game", "pressed", |this, _| {
				let save_name = this.typed_save_name();
				if save_name.is_empty() {
					this.set_easter_egg_text("You need to enter a save name");
					return;
				}

				let save_already_exists = this
					.spawned_load_buttons
					.iter()
					.any(|bttn| bttn.bind().get_save_name() == save_name);

				if !save_already_exists {
					this.base_mut()
						.emit_signal(Self::SIGNAL_NEW_GAME, &[save_name.to_variant()]);
				} else {
					let mut panel = this.panel_are_you_sure_overwrite_save.bind_mut();
					panel.set_text("Are you sure you want to overwrite?");
					panel.base_mut().set_visible(true);
					this.line_edit_save_name.release_focus(); // to make sure the player can't edit the save name while the `are you sure panel` is open
				}
			})
			.log_if_err();

			let mut config = ConfigFile::new_gd();

			let maybe_err = config.load(CONFIG_PATH);
			if maybe_err != Error::OK {
				config.set_value("iron_gauntlet", "times_pressed", &0.to_variant());
				config.save(CONFIG_PATH).log_if_err();
			}

			config.load(CONFIG_PATH);

			self.iron_gauntlet_times_pressed = config
				.get_value("iron_gauntlet", "times_pressed")
				.try_to::<u64>()
				.ok()
				.unwrap_or(0);

			self.connect_child("panel_new-game/fake-toggle_iron-gauntlet", "pressed", |this, _| {
				this.iron_gauntlet_times_pressed += 1;

				let mut config = ConfigFile::new_gd();

				config.load(CONFIG_PATH).log_if_err();

				config.set_value(
					"iron_gauntlet",
					"times_pressed",
					&this.iron_gauntlet_times_pressed.to_variant(),
				);

				config.save(CONFIG_PATH).log_if_err();

				if this.iron_gauntlet_times_pressed >= easters_iron_gauntlet::MAX_PRESSES {
					this.base().get_tree().unwrap().quit();
				} else {
					let easter =
						easters_iron_gauntlet::get_easter(this.iron_gauntlet_times_pressed);
					this.set_easter_egg_text(easter);
				}
			})
			.log_if_err();

			self.connect_child(
				"panel_new-game/line-edit_save-name",
				"text_changed",
				|this, args| {
					let Some(new_text) = args.first().and_then(|s| s.try_to::<GString>().ok())
					else {
						return godot_error!(
							"LineEdit's text_changed signal did not provide string argument"
						)
					};

					let cleaned_text = {
						let mut temp = new_text.to_string();
						temp.retain(|char| char.is_ascii_alphanumeric());
						temp.chars()
							.map(|char| char.to_ascii_lowercase())
							.collect::<String>()
					};

					if cleaned_text == "alexa" {
						this.base().get_tree().unwrap().quit();
					} else {
						let easter_text = easters_save_name::get_name_easter(cleaned_text.as_str())
							.unwrap_or_default();

						this.set_easter_egg_text(easter_text);
					}
				},
			)
			.log_if_err();

			self.connect_child(
				"panel_are_you_sure_overwrite_save",
				PanelAreYouSure::SIGNAL_YES,
				|this, _| {
					let save_name = this.typed_save_name();
					this.base_mut().emit_signal(
						Self::SIGNAL_OVERWRITE_SAVE_AND_START,
						&[save_name.to_variant()],
					);
				},
			)
			.log_if_err();
		}
	}

	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		if any_cancel_input(&event) {
			self.close_all_panels();
		}
	}
}

#[godot_api]
impl MainMenuController {
	pub const SIGNAL_NEW_GAME: &'static str = "new_game";
	pub const SIGNAL_LOAD_GAME: &'static str = "load_game";
	pub const SIGNAL_DELETE_SAVE: &'static str = "delete_save";
	pub const SIGNAL_OVERWRITE_SAVE_AND_START: &'static str = "overwrite_save_and_start";
	pub const SIGNAL_OPEN_SETTINGS_MENU: &'static str = "open_settings_menu";

	#[signal]
	fn new_game(save_name: GString) {}

	#[signal]
	fn load_game(save_name: GString) {}

	#[signal]
	fn delete_save(save_name: GString) {}

	#[signal]
	fn overwrite_save_and_start(save_name: GString) {}

	#[signal]
	fn open_settings_menu() {}

	fn spawn_load_button(&mut self, save_name: impl Into<String>) {
		let mut button = self.prefab_load_button.instantiate_as::<LoadButton>();

		button.bind_mut().set_save_name(save_name);

		self.connect_with_deferred(&button, LoadButton::SIGNAL_LOAD, |this, args: &[&Variant]| {
			let Some(save_name) = args.first().and_then(|arg| arg.try_to::<GString>().ok())
			else { return godot_error!("Load button signal did not provide string argument") };

			this.base_mut()
				.emit_signal(Self::SIGNAL_LOAD_GAME, &[save_name.to_variant()]);
		});

		self.connect_with_deferred(
			&button,
			LoadButton::SIGNAL_DELETE,
			|this, args: &[&Variant]| {
				let Some(save_name) = args.first().and_then(|arg| arg.try_to::<GString>().ok())
				else {
					return godot_error!("Load button signal did not provide string argument")
				};

				this.base_mut()
					.emit_signal(Self::SIGNAL_DELETE_SAVE, &[save_name.to_variant()]);
			},
		);

		self.container_load_buttons.add_child(&button);
	}

	pub fn create_and_assign_load_buttons(&mut self, saves: impl IntoIterator<Item = String>) {
		saves.into_iter().enumerate().for_each(|(idx, save)| {
			if let Some(bttn) = self.spawned_load_buttons.get_mut(idx) {
				bttn.bind_mut().set_save_name(save.clone());
			} else {
				self.spawn_load_button(save);
			}
		});
	}

	fn close_all_panels(&mut self) {
		self.panel_new_game.hide();
		self.panel_load_game.hide();
		self.panel_are_you_sure_overwrite_save
			.bind_mut()
			.base_mut()
			.hide();
	}

	fn typed_save_name(&self) -> String { self.line_edit_save_name.get_text().to_string() }

	fn set_easter_egg_text(&mut self, text: &str) {
		if let Some(tween) = self.tween_easter_egg_text.take() {
			tween.kill();
		}

		self.label_easter_egg.set_text(text);
		self.label_easter_egg.set_visible_ratio(0.0);

		self.tween_easter_egg_text = Some(
			self.label_easter_egg
				.do_visible_ratio(1.0, text.len() as f64 * 0.015)
				.register(),
		);
	}
}
