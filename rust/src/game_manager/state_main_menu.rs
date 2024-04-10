#[allow(unused_imports)]
use crate::*;
use crate::main_menu::{
	SIGNAL_DELETE_SAVE,
	SIGNAL_LOAD_GAME,
	SIGNAL_NEW_GAME,
	SIGNAL_OPEN_SETTINGS_MENU,
	SIGNAL_OVERWRITE_SAVE_AND_START,
};
use crate::save::file::SaveFile;

use super::*;

#[derive(Debug)]
pub enum MainMenuState {
	Idle,
	LoadingSession { save: SaveFile },
	SettingsMenu,
}

#[methods(mixin = "GM", pub)]
impl GameManager {
	pub fn main_menu_register_signals(gm_owner_ref: Ref<Node>, main_menu_owner: TRef<Control>) {
		main_menu_owner.connect(SIGNAL_NEW_GAME, gm_owner_ref, fn_name(&Self::_main_menu_new_game), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		main_menu_owner.connect(SIGNAL_LOAD_GAME, gm_owner_ref, fn_name(&Self::_main_menu_load_game), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		main_menu_owner.connect(SIGNAL_DELETE_SAVE, gm_owner_ref, fn_name(&Self::_main_menu_delete_save), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		main_menu_owner.connect(SIGNAL_OVERWRITE_SAVE_AND_START, gm_owner_ref, fn_name(&Self::_main_menu_overwrite_save_and_start), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
		main_menu_owner.connect(SIGNAL_OPEN_SETTINGS_MENU, gm_owner_ref, fn_name(&Self::_main_menu_open_settings_menu), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}

	fn fade_then_load_session(&mut self, owner: &Node) {
		let owner_ref = unsafe { owner.assume_shared() };

		self.session_load_sound
		    .unwrap_manual()
		    .play(0.0);

		let fade_screen = self.fade_screen.unwrap_manual();
		fade_screen.set_modulate(Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 });
		fade_screen.show();

		let tween_ref = seek_tree_and_create_tween!(owner);
		let tween = unsafe { tween_ref.assume_safe() };
		tween.tween_property(fade_screen, "modulate",
			Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0 }, 2.0);

		tween.connect("finished", owner_ref, fn_name(&Self::_main_menu_load_session_fade_completed),
			VariantArray::new_shared(), Object::CONNECT_DEFERRED)
		     .log_if_err();
	}

	#[method]
	fn _main_menu_load_session_fade_completed(&mut self) {
		let (main_menu, _save) =
			match std::mem::take(&mut self.state) {
				GameState::MainMenu(main_menu,
					MainMenuState::LoadingSession { save }) => (main_menu, save),
				other_state => {
					godot_error!("{}():\n Error: Load session fade completed, but MainMenu state is: {other_state:?}.",
						fn_name(&Self::_main_menu_overwrite_save_and_start));
					self.state = other_state;
					return;
				}
			};

		let main_menu_inst = main_menu.unwrap_inst();
		let main_menu_base = main_menu_inst.base();
		let main_menu_parent_option = main_menu_base.get_parent();
		main_menu_parent_option
			.unwrap_manual()
			.remove_child(main_menu_base);

		main_menu_base.queue_free();
		drop(main_menu_inst);
		// todo! set new state, start session
	}

	#[method]
	fn _main_menu_new_game(&mut self, #[base] owner: &Node, save_name: GodotString) {
		let main_menu =
			match std::mem::take(&mut self.state) {
				GameState::MainMenu(inner_main_menu, MainMenuState::Idle) => inner_main_menu,
				other_state => {
					godot_warn!("{}():\n Cannot start new game from MainMenu when state is: {other_state:?}.",
						fn_name(&Self::_main_menu_new_game));
					self.state = other_state;
					return;
				}
			};
		
		let save = SaveFile::new(save_name.to_string());
		self.save_files.add_save(save.clone());
		
		self.state = GameState::MainMenu(main_menu, MainMenuState::LoadingSession { save });
		self.fade_then_load_session(owner);
	}

	#[method]
	fn _main_menu_overwrite_save_and_start(&mut self, #[base] owner: &Node, save_name: GodotString) {
		let main_menu =
			match std::mem::take(&mut self.state) {
				GameState::MainMenu(inner_main_menu, MainMenuState::Idle) => inner_main_menu,
				other_state => {
					godot_warn!("{}():\n Cannot overwrite save with name `{save_name}` from MainMenu when state is: {other_state:?}.",
						fn_name(&Self::_main_menu_overwrite_save_and_start));
					self.state = other_state;
					return;
				}
			};

		let save = SaveFile::new(save_name.to_string());
		self.save_files.overwrite_save(save.clone());
		
		self.state = GameState::MainMenu(main_menu, MainMenuState::LoadingSession { save });
		self.fade_then_load_session(owner);
	}

	#[method]
	fn _main_menu_load_game(&mut self, #[base] owner: &Node, save_name: GodotString) {
		let main_menu =
			match std::mem::take(&mut self.state) {
				GameState::MainMenu(inner_main_menu, MainMenuState::Idle) => inner_main_menu,
				other_state => {
					godot_warn!("{}():\n Cannot load save with name `{save_name}` from MainMenu when state is: {other_state:?}.",
						fn_name(&Self::_main_menu_overwrite_save_and_start));
					self.state = other_state;
					return;
				}
			};
		
		let Some(save) = self.save_files.get_save(save_name.to_string().as_str()).cloned()
			else {
				godot_warn!("{}():\n Cannot load save with name `{save_name}` from MainMenu because it does not exist.",
					full_fn_name(&Self::_main_menu_load_game));
				self.state = GameState::MainMenu(main_menu, MainMenuState::Idle);
				return;
			};
		
		self.state = GameState::MainMenu(main_menu, MainMenuState::LoadingSession { save });
		self.fade_then_load_session(owner);
	}

	#[method]
	fn _main_menu_delete_save(&mut self, save_name: GodotString) {
		if let GameState::MainMenu(instance, MainMenuState::Idle) = &self.state {
			self.save_files.delete_save(save_name.to_string().as_str());
			instance.touch_assert_safe_mut(|main_menu, main_menu_owner| {
				main_menu.create_and_assign_load_buttons(main_menu_owner, self.save_files.get_saves());
			});
		} else {
			godot_error!("{}():\n Cannot delete save with name `{save_name}` from MainMenu when state is: {:?}", 
				fn_name(&Self::_main_menu_delete_save), self.state);
		};
	}

	#[method]
	fn _main_menu_open_settings_menu(&mut self) {
		let main_menu =
			match std::mem::take(&mut self.state) {
				GameState::MainMenu(inner_main_menu, MainMenuState::Idle) => inner_main_menu,
				other_state => {
					godot_warn!("{}():\n\
					Cannot open Settings Menu from MainMenu when state is: {other_state:?}.", 
					fn_name(&Self::_main_menu_open_settings_menu));
					self.state = other_state;
					return;
				}
			};
		
		self.state = GameState::MainMenu(main_menu, MainMenuState::SettingsMenu);
		self.open_settings_menu();
	}
}