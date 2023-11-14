use gdrust_export_nodepath::extends;
use gdnative::api::*;
use gdnative::prelude::*;
use crate::save::file::*;
use crate::save::*;
use crate::util::panel_are_you_sure;
use crate::util::panel_are_you_sure::PanelAreYouSure;
use houta_utils::prelude::*;
use load_button::LoadButton;

mod easters_save_name;
mod easters_iron_gauntlet;
pub(crate) mod load_button;

#[extends(Control)]
pub struct MainMenu {
	#[property] background: Option<Ref<Control>>,

	// Panel - NEW GAME
	#[property] panel_new_game           : Option<Ref<Control >>,
	#[property] fake_toggle_iron_gauntlet: Option<Ref<Button  >>, iron_gauntlet_times_pressed: usize,
	#[property] label_easter_egg         : Option<Ref<Label   >>, tween_easter_egg_text      : Option<Ref<SceneTreeTween>>,
	#[property] button_new_game          : Option<Ref<Button  >>,
	#[property] line_edit_save_name      : Option<Ref<LineEdit>>,
	#[property] button_start_game        : Option<Ref<Button  >>,
	#[property] path_panel_are_you_sure_overwrite_save: NodePath, panel_are_you_sure_overwrite_save: Option<Instance<PanelAreYouSure>>,

	// Panel - LOAD GAME
	#[property] panel_load_game       : Option<Ref<Control    >>,
	#[property] container_load_buttons: Option<Ref<Control    >>,
	#[property] prefab_load_button    : Option<Ref<PackedScene>>,
	spawned_load_buttons: Vec<Instance<LoadButton>>,
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
		// Panel - NEW GAME
		{
			self.background       .unwrap_manual().connect("gui_input", owner_ref, "_on_background_gui_input"     , VariantArray::new_shared(), Object::CONNECT_DEFERRED);
			self.button_new_game  .unwrap_manual().connect("pressed"  , owner_ref, "_on_button_clicked_new_game"  , VariantArray::new_shared(), Object::CONNECT_DEFERRED);
			self.button_start_game.unwrap_manual().connect("pressed"  , owner_ref, "_on_button_clicked_start_game", VariantArray::new_shared(), Object::CONNECT_DEFERRED);

			let config = ConfigFile::new();
			config.load(crate::config_path)
			      .touch_if_err(|err| {
				      godot_error!("Failed to load config.cfg: {}", err);
				      config.set_value("iron_gauntlet", "times_pressed", 0);
				      config.save(crate::config_path).log_if_err();
			      });

			self.iron_gauntlet_times_pressed = config.get_value("iron_gauntlet", "times_pressed", 0).to().unwrap();
			self.fake_toggle_iron_gauntlet.unwrap_manual().connect("pressed", owner_ref, "_on_fake_toggle_iron_gauntlet_pressed", VariantArray::new_shared(), Object::CONNECT_DEFERRED);

			self.panel_are_you_sure_overwrite_save.touch_assert_safe(|panel, base| base.connect(panel_are_you_sure::signal_yes, owner_ref, "_on_panel_are_you_sure_overwrite_save_yes",
			VariantArray::new_shared(), Object::CONNECT_DEFERRED));
		}

		// Panel - LOAD GAME
		{
			if let Some(container) = self.container_load_buttons.assert_tref_if_sane()
					&& let Some(prefab) = self.prefab_load_button.assert_tref() {
						//todo!
			}
		}
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
	fn _on_button_clicked_new_game(&self) {
		self.panel_new_game.touch_assert_sane(|panel| panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _on_button_clicked_start_game(&mut self) {
		let save_name = self.typed_save_name();
		if save_name.len() == 0 {
			self.set_easter_egg_text("You need to enter a save name");
			return;
		}

		let save_file = SaveFile::new(save_name.clone());
		let saves_manager = SavesManager::godot_singleton();

		saves_manager.map_mut(|manager, _| {
			if manager.get_saves().get(&save_name).is_some() {
				self.panel_are_you_sure_overwrite_save.touch_assert_safe(|panel, base| {
					panel.set_text(format!("Are you sure you want to overwrite save: {}", save_name).as_str());
					base.set_visible(true);
				});

				self.line_edit_save_name.unwrap_manual().release_focus();  // to make sure the player can't edit the save name while the are you sure panel is open
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
}