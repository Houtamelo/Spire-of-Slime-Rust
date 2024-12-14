use main_menu::prelude::MainMenuController;

use super::*;

#[allow(clippy::large_enum_variant)]
pub enum MainMenuState {
	Idle,
	LoadingSession(SaveFile),
	SettingsMenu,
}

impl Debug for MainMenuState {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			MainMenuState::Idle => write!(f, "Idle"),
			MainMenuState::LoadingSession(SaveFile { name, .. }) => {
				write!(f, "LoadingSession(save: {name})")
			}
			MainMenuState::SettingsMenu => write!(f, "SettingsMenu"),
		}
	}
}

impl GameManager {
	pub(super) fn load_main_menu(&mut self) {
		let resource = ResourceLoader::singleton()
			.load_ex("res://Core/Main Menu/scene_main-menu.tscn")
			.type_hint("PackedScene")
			.cache_mode(CacheMode::REUSE)
			.done()
			.expect("Failed to load scene_main-menu.tscn")
			.cast::<PackedScene>();

		let mut main_menu = resource.instantiate_as::<MainMenuController>();
		self.base_mut().add_child(&main_menu);

		main_menu
			.bind_mut()
			.create_and_assign_load_buttons(self.save_files.get_saves().keys().cloned());

		self.connect_with_deferred(
			&main_menu,
			MainMenuController::SIGNAL_NEW_GAME,
			|this, args| {
				let Some(save_name) = args.first().and_then(|arg| arg.try_to::<String>().ok())
				else {
					return godot_error!("`NEW_GAME` signal did not provide save name argument")
				};

				let GameState::MainMenu(main_menu, MainMenuState::Idle) = &this.state
				else {
					godot_warn!("Cannot start new game when state is: {:?}.", this.state);
					return;
				};

				let save = SaveFile::new(save_name);
				this.save_files.add_save(save.clone());
				this.state =
					GameState::MainMenu(main_menu.clone(), MainMenuState::LoadingSession(save));
				this.fade_then_load_session();
			},
		);

		self.connect_with_deferred(
			&main_menu,
			MainMenuController::SIGNAL_LOAD_GAME,
			|this, args| {
				let Some(save_name) = args.first().and_then(|arg| arg.try_to::<String>().ok())
				else {
					return godot_error!("`LOAD_GAME` signal did not provide save name argument")
				};

				let GameState::MainMenu(main_menu, MainMenuState::Idle) = &this.state
				else {
					godot_warn!(
						"Cannot load save with name `{save_name}` when state is: {:?}.",
						this.state
					);
					return;
				};

				if let Some(save) = this.save_files.get_save(&save_name) {
					this.state = GameState::MainMenu(
						main_menu.clone(),
						MainMenuState::LoadingSession(save.clone()),
					);
					this.fade_then_load_session();
				} else {
					godot_warn!(
						"Cannot load save with name `{save_name}` because it does not exist."
					);
					this.state = GameState::MainMenu(main_menu.clone(), MainMenuState::Idle);
				}
			},
		);

		self.connect_with_deferred(
			&main_menu,
			MainMenuController::SIGNAL_DELETE_SAVE,
			|this, args| {
				let Some(save_name) = args.first().and_then(|arg| arg.try_to::<String>().ok())
				else {
					return godot_error!("`DELETE_SAVE` signal did not provide save name argument")
				};

				match &mut this.state {
					GameState::MainMenu(main_menu, MainMenuState::Idle) => {
						this.save_files.delete_save(&save_name);
						main_menu.bind_mut().create_and_assign_load_buttons(
							this.save_files.get_saves().keys().cloned(),
						);
					}
					invalid => {
						godot_error!(
							"Cannot delete save with name `{save_name}` when state is: {invalid:?}"
						);
					}
				}
			},
		);

		self.connect_with_deferred(
			&main_menu,
			MainMenuController::SIGNAL_OVERWRITE_SAVE_AND_START,
			|this, args| {
				let Some(save_name) = args.first().and_then(|arg| arg.try_to::<String>().ok())
				else {
					return godot_error!(
						"`OVERWRITE_SAVE_AND_START` signal did not provide save name argument"
					)
				};

				let GameState::MainMenu(main_menu, MainMenuState::Idle) = &this.state
				else {
					godot_warn!(
						"Cannot overwrite save with name `{save_name}` when state is: {:?}.",
						this.state
					);
					return;
				};

				let save = SaveFile::new(save_name.to_string());
				this.save_files.overwrite_save(save.clone());
				this.state =
					GameState::MainMenu(main_menu.clone(), MainMenuState::LoadingSession(save));
				this.fade_then_load_session();
			},
		);

		self.connect_with_deferred(
			&main_menu,
			MainMenuController::SIGNAL_OPEN_SETTINGS_MENU,
			|this, _| {
				let GameState::MainMenu(main_menu, MainMenuState::Idle) = &this.state
				else {
					godot_warn!("Cannot open Settings Menu when state is: {:?}.", this.state);
					return;
				};

				this.state = GameState::MainMenu(main_menu.clone(), MainMenuState::SettingsMenu);
				this.open_settings_menu();
			},
		);

		self.state = GameState::MainMenu(main_menu, MainMenuState::Idle);
	}

	fn fade_then_load_session(&mut self) {
		self.session_load_sound.play();

		let mut fade_screen = self.fade_screen.clone();
		fade_screen.show();

		fade_screen
			.do_fade(1.0, 2.0)
			.starting_at(0.0)
			.on_finish(
				|| todo!(), /*
							{
							let mut self_gd = self.to_gd();
							move || {


								let this = &mut *self_gd.bind_mut();
								let GameState::MainMenu(mut main_menu, MainMenuState::LoadingSession(save)) = this.state
								else {
									godot_error!("Load session fade completed but MainMenu state is: {:?}.", this.state);
									return;
								};

								this.base_mut().remove_child(&main_menu);
								main_menu.queue_free();
							}
							}
							*/
			)
			.register();
	}
}
