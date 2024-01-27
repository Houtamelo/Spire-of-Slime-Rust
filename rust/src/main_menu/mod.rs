mod easters_save_name;
mod easters_iron_gauntlet;
mod load_button;

use gdnative_export_node_as_path::extends;
use gdnative::api::*;
use gdnative::prelude::*;
use crate::{saves::file::*, GameManager};
use crate::saves::*;
use crate::util::panel_are_you_sure;
use crate::util::panel_are_you_sure::PanelAreYouSure;
use houta_utils_gdnative::prelude::*;

pub(super) use load_button::LoadButton;

pub(super) const CALL_MAIN_MENU_TO_SETTINGS: &str = "main_menu_to_settings";
pub(super) const CALL_MAIN_MENU_LOAD_GAME  : &str = "main_menu_load_game";

#[extends(Control)]
#[derive(Debug)]
pub(super) struct MainMenu {
	#[export_path] background      : Option<Ref<Control>>,
	#[export_path] button_new_game : Option<Ref<Button>>,
	#[export_path] button_load_game: Option<Ref<Button>>,
	#[export_path] button_settings : Option<Ref<Button>>,
	#[export_path] button_credit   : Option<Ref<Button>>,
	#[export_path] button_exit     : Option<Ref<Button>>,

	// Panel - NEW GAME
	#[export_path] fake_toggle_iron_gauntlet: Option<Ref<Button  >>, iron_gauntlet_times_pressed: usize,
	#[export_path] panel_new_game     : Option<Ref<Control >>,
	#[export_path] label_easter_egg   : Option<Ref<Label   >>, tween_easter_egg_text: Option<Ref<SceneTreeTween>>,
	#[export_path] line_edit_save_name: Option<Ref<LineEdit>>,
	#[export_path] button_start_game  : Option<Ref<Button  >>,
	#[export_path] panel_are_you_sure_overwrite_save: Option<Instance<PanelAreYouSure>>,

	// Panel - LOAD GAME
	#[export_path] panel_load_game: Option<Ref<Control>>,
	#[export_path] container_load_buttons: Option<Ref<Control>>,
	#[property] prefab_load_button: Option<Ref<PackedScene>>,
	spawned_load_buttons: Vec<Instance<LoadButton>>,

	// Panel - CREDITS
	#[export_path] panel_credits: Option<Ref<Control>>,
}

fn crash_game() {
	unsafe { Engine::godot_singleton().get_main_loop().unwrap().assume_safe().cast::<SceneTree>().unwrap().quit(0) };
}

#[methods]
impl MainMenu {
	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		let owner_ref = unsafe { owner.assume_shared() };

		self.background.unwrap_manual().connect("gui_input", owner_ref, "_on_background_gui_input", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

		// Menu Buttons
		{
			self.button_new_game .unwrap_manual().connect("pressed", owner_ref, "_open_panel_new_game" , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
			self.button_load_game.unwrap_manual().connect("pressed", owner_ref, "_open_panel_load_game", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
			self.button_settings .unwrap_manual().connect("pressed", owner_ref, "_open_settings_menu"  , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
			self.button_credit   .unwrap_manual().connect("pressed", owner_ref, "_open_panel_credits"  , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
			self.button_exit     .unwrap_manual().connect("pressed", owner_ref, "_exit_game"           , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
		}

		// Panel - NEW GAME
		{
			self.button_start_game.unwrap_manual().connect("pressed", owner_ref, "_on_button_clicked_start_game", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

			let config = ConfigFile::new();
			config.load(crate::config_path)
			.touch_if_err(|err| {
				godot_error!("Failed to load config.cfg: {}", err);
				config.set_value("iron_gauntlet", "times_pressed", 0);
				config.save(crate::config_path).log_if_err();
			});

			self.iron_gauntlet_times_pressed = config.get_value("iron_gauntlet", "times_pressed", 0).to().unwrap();
			self.fake_toggle_iron_gauntlet.unwrap_manual().connect("pressed", owner_ref, "_on_fake_toggle_iron_gauntlet_pressed", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();

			self.panel_are_you_sure_overwrite_save.touch_assert_safe(|_, base| base.connect(panel_are_you_sure::signal_yes, owner_ref, "_on_panel_are_you_sure_overwrite_save_yes", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err());
		}

		// Panel - LOAD GAME
		{
			self.create_and_assign_load_buttons(&owner_ref);
		}
	}

	fn create_and_assign_load_buttons(&mut self, owner_ref: &Ref<Control>) {
		let container= self.container_load_buttons.unwrap_manual();
		let prefab= self.prefab_load_button.unwrap_refcount();
		let saves_manager = SavesManager::godot_singleton();

		saves_manager.map(|sm: &SavesManager, _| {
			let saves = sm.get_saves();
			let saves_count = saves.len();
			let mut spawned_count = self.spawned_load_buttons.len();
			while spawned_count <= saves_count {
				let node = unsafe {
					prefab.instance(PackedScene::GEN_EDIT_STATE_DISABLED).expect("Failed to create LoadButton node from packed scene")
						  .assume_safe_if_sane().expect("Failed to assume safe and sane LoadButton node from packed scene")
				};
				
				node.connect(load_button::SIGNAL_LOAD  , owner_ref, "_on_load_button_clicked"  , VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
				node.connect(load_button::SIGNAL_DELETE, owner_ref, "_on_delete_save_confirmed", VariantArray::new_shared(), Object::CONNECT_DEFERRED).log_if_err();
				container.add_child(node, false);
				let control = node.cast::<Control>().expect("Failed to cast LoadButton node to Control");
				let instance = control.cast_instance::<LoadButton>().expect("Failed to cast LoadButton control to LoadButton");
				self.spawned_load_buttons.push(instance.claim());
				spawned_count += 1;
			}

			let mut index = 0;
			for save_name in saves.keys() {
				self.spawned_load_buttons.get(index)
					.touch_assert_safe(|button, base| {
						button.set_save_name(save_name);
						base.set_visible(true);
					});

				index += 1;
			}

			self.spawned_load_buttons.iter().skip(index).for_each(|instance| {
				instance.touch_assert_safe(|button, base| {
					button.set_save_name("");
					base.set_visible(false);
				});
			});
		}).log_if_err();
	}
	
	#[method]
	fn _on_load_button_clicked(&self, save_name: GodotString) {
		unsafe { GameManager::godot_singleton().base().call_deferred(CALL_MAIN_MENU_LOAD_GAME, &[save_name.to_variant()]) };
	}
	
	#[method]
	fn _on_delete_save_confirmed(&mut self, #[base] owner: &Control, save_name: GodotString) {
		let saves_manager = SavesManager::godot_singleton();
		saves_manager.map_mut(|manager, _| manager.delete_save(save_name.to_string().as_str())).log_if_err();
		let owner_ref = unsafe { owner.assume_shared() };
		self.create_and_assign_load_buttons(&owner_ref);
	}

	#[method]
	fn _process(&self) {
		if Input::godot_singleton().is_action_just_pressed("ui_cancel", true) {
			self.close_all_panels();
		}
	}

	#[method]
	fn _on_background_gui_input(&self) {
		self.close_all_panels();
	}

	fn close_all_panels(&self) {
		self.panel_new_game.unwrap_manual().set_visible(false);
		self.panel_are_you_sure_overwrite_save.touch_assert_safe(|_, base| base.set_visible(false));
		self.panel_load_game.unwrap_manual().set_visible(false);
	}

	#[method]
	fn _on_fake_toggle_iron_gauntlet_pressed(&mut self) {
		self.iron_gauntlet_times_pressed = usize::clamp(self.iron_gauntlet_times_pressed + 1, 0, easters_iron_gauntlet::max_presses);

		let config = ConfigFile::new();
		config.load(crate::config_path).log_if_err();
		config.set_value("iron_gauntlet", "times_pressed", self.iron_gauntlet_times_pressed);
		config.save(crate::config_path).log_if_err();

		if self.iron_gauntlet_times_pressed >= easters_iron_gauntlet::max_presses {
			crash_game();
			return;
		}

		config.save(crate::config_path).log_if_err();

		let easter = easters_iron_gauntlet::get_easter(self.iron_gauntlet_times_pressed);
		self.set_easter_egg_text(easter);
	}

	#[method]
	fn _on_button_clicked_start_game(&mut self) {
		let save_name = self.typed_save_name();
		if save_name.len() == 0 {
			self.set_easter_egg_text("You need to enter a saves name");
			return;
		}

		let save_file = SaveFile::new(save_name.clone());
		let saves_manager = SavesManager::godot_singleton();

		saves_manager.map_mut(|manager, _| {
			if manager.get_saves().get(&save_name).is_some() {
				self.panel_are_you_sure_overwrite_save.touch_assert_safe(|panel, base| {
					panel.set_text(format!("Are you sure you want to overwrite saves: {}", save_name).as_str());
					base.set_visible(true);
				});

				self.line_edit_save_name.unwrap_manual().release_focus();  // to make sure the player can't edit the saves name while the are you sure panel is open
			}
			else {
				manager.add_save(save_file.clone());
				self.start_game(save_file);
			}
		}).log_if_err();
	}

	#[method]
	fn _on_line_edit_changed_save_name(&mut self, new_text: GodotString) {
		let mut cleaned_text = new_text.to_string();
		cleaned_text.retain(|c| c.is_ascii_alphanumeric());

		if cleaned_text == "alexa" {
			crash_game();
			return;
		}

		self.set_easter_egg_text(easters_save_name::get_name_easter(cleaned_text.as_str()).unwrap_or_default());
		self.line_edit_save_name.unwrap_manual().set_text(cleaned_text);
	}

	fn typed_save_name(&self) -> String {
		return self.line_edit_save_name.unwrap_manual().text().to_string();
	}

	#[method]
	fn _on_panel_are_you_sure_overwrite_save_yes(&self) {
		let save_name = self.typed_save_name();
		let save = SaveFile::new(save_name.to_string());
		SavesManager::godot_singleton().map_mut(|saves_manager, _| saves_manager.add_save(save.clone())).log_if_err();
		self.start_game(save);
	}

	fn start_game(&self, _save: SaveFile) {
		//todo!
	}

	fn set_easter_egg_text(&mut self, text: &str) {
		self.tween_easter_egg_text.kill_if_some();

		let label = self.label_easter_egg.unwrap_manual();
		label.set_text(text);
		label.set_percent_visible(0.0);
		match label.create_tween() {
			Some(tween_ref) => {
				let tween = unsafe { tween_ref.assume_safe() };
				tween.tween_property(label, "percent_visible", 1.0, text.len() as f64 * 0.015);
				self.tween_easter_egg_text = Some(tween_ref);
			},
			None => {
				godot_error!("Failed to create tween for label_easter_egg");
			}
		}
	}

	#[method]
	fn _open_panel_new_game(&self) {
		self.panel_new_game.touch_assert_sane(|panel| panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _open_panel_load_game(&self) {
		self.panel_load_game.touch_assert_sane(|panel| panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _open_settings_menu(&self) {
		unsafe { GameManager::godot_singleton().base().call_deferred(CALL_MAIN_MENU_TO_SETTINGS, &[]) };
	}

	#[method]
	fn _open_panel_credits(&self) {
		self.panel_credits.touch_assert_sane(|panel| panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _exit_game(&self) {
		self.background.unwrap_manual().get_tree().unwrap_manual().quit(0);
	}
}
