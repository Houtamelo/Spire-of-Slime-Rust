mod easters_save_name;
mod easters_iron_gauntlet;

use std::ops::Not;
use gdnative::api::*;
use gdnative::api::object::ConnectFlags;
use gdnative::prelude::*;
use crate::*;
use crate::save::file::*;
use crate::save::*;
use crate::util::{panel_are_you_sure};
use crate::util::panel_are_you_sure::PanelAreYouSure;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct MainMenu {
	#[property] path_background: NodePath,
	background: Option<Ref<Control>>,

	// Panel - NEW GAME
	#[property] path_panel_new_game: NodePath,
	panel_new_game: Option<Ref<Control>>,
	#[property] path_fake_toggle_iron_gauntlet: NodePath,
	fake_toggle_iron_gauntlet: Option<Ref<Button>>,
	iron_gauntlet_times_pressed: usize,
	#[property] path_label_easter_egg: NodePath,
	label_easter_egg: Option<Ref<Label>>,
	tween_easter_egg_text: Option<Ref<SceneTreeTween>>,
	#[property] path_button_new_game: NodePath,
	button_new_game: Option<Ref<Button>>,
	#[property] path_line_edit_save_name: NodePath,
	line_edit_save_name: Option<Ref<LineEdit>>,
	#[property] path_button_start_game: NodePath,
	button_start_game: Option<Ref<Button>>,
	#[property] path_panel_are_you_sure_overwrite_save: NodePath,
	panel_are_you_sure_overwrite_save: Option<Instance<PanelAreYouSure>>,
	// Panel - NEW GAME

}

fn crash_game() {
	unsafe { Engine::godot_singleton().get_main_loop().unwrap().assume_safe().cast::<SceneTree>().unwrap().quit(0) };
}

#[methods]
impl MainMenu {
	fn new(_owner: &Control) -> Self {
		MainMenu {
			path_background               : NodePath::default(), background               : None,
			path_panel_new_game           : NodePath::default(), panel_new_game           : None,
			path_fake_toggle_iron_gauntlet: NodePath::default(), fake_toggle_iron_gauntlet: None, iron_gauntlet_times_pressed: 0,
			path_label_easter_egg         : NodePath::default(), label_easter_egg         : None, tween_easter_egg_text: None,
			path_button_new_game          : NodePath::default(), button_new_game: None,
			path_line_edit_save_name      : NodePath::default(), line_edit_save_name      : None,
			path_button_start_game        : NodePath::default(), button_start_game: None,
			path_panel_are_you_sure_overwrite_save: NodePath::default(), panel_are_you_sure_overwrite_save: None,
		}
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		let owner_ref = unsafe { owner.assume_shared() };

		{
			let Some(background) = (unsafe { owner.get_node_as::<Control>(self.path_background.new_ref()) })
					else { godot_error!("Failed to get background");return; };
			self.background = Some(unsafe { background.assume_shared() });

			background.connect("gui_input ", owner_ref, "_on_background_gui_input",
			                   VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err();
		}

		let Some(panel_new_game) = (unsafe { owner.get_node_as::<Control>(self.path_panel_new_game.new_ref()) })
				else { godot_error!("Failed to get panel_new_game"); return; };
		self.panel_new_game = Some( unsafe { panel_new_game.assume_shared() });

		{
			let Some(fake_toggle_iron_gauntlet) = (unsafe { owner.get_node_as::<Button>(self.path_fake_toggle_iron_gauntlet.new_ref()) })
					else { godot_error!("Failed to get fake_toggle_iron_gauntlet"); return; };
			self.fake_toggle_iron_gauntlet = Some(unsafe { fake_toggle_iron_gauntlet.assume_shared() });

			let config = ConfigFile::new();
			config.load(crate::config_path)
					.on_err(|error| {
						godot_error!("Failed to load config.cfg: {}", error);
						config.set_value("iron_gauntlet", "times_pressed", 0);
						config.save(crate::config_path).report_on_err();
					});

			self.iron_gauntlet_times_pressed = config.get_value("iron_gauntlet", "times_pressed", 0).to().unwrap();

			fake_toggle_iron_gauntlet.connect("pressed", owner_ref, "_on_fake_toggle_iron_gauntlet_pressed",
			                                  VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err();
		}

		let Some(label_easter_egg) = (unsafe { owner.get_node_as::<Label>(self.path_label_easter_egg.new_ref()) })
				else { godot_error!("Failed to get label_easter_egg"); return; };
		self.label_easter_egg = Some( unsafe { label_easter_egg.assume_shared() });

		{
			let Some(button_new_game) = (unsafe { owner.get_node_as::<Button>(self.path_button_new_game.new_ref()) })
					else { godot_error!("Failed to get buton_new_game"); return; };
			self.button_new_game = Some(unsafe { button_new_game.assume_shared() });

			button_new_game.connect("pressed", owner_ref, "_on_button_clicked_new_game",
			                        VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err();
		}

		let Some(line_edit_save_name) = (unsafe { owner.get_node_as::<LineEdit>(self.path_line_edit_save_name.new_ref()) })
				else { godot_error!("Failed to get line_edit_save_name"); return; };
		self.line_edit_save_name = Some( unsafe { line_edit_save_name.assume_shared() });

		{
			let Some(buton_start_game) = (unsafe { owner.get_node_as::<Button>(self.path_button_start_game.new_ref()) })
					else { godot_error!("Failed to get buton_start_game"); return; };
			self.button_start_game = Some(unsafe { buton_start_game.assume_shared() });

			buton_start_game.connect("pressed", owner_ref, "_on_button_clicked_start_game",
			                         VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err();
		}

		{
			let Some(are_you_sure) = (unsafe { owner.get_node_as_instance::<PanelAreYouSure>(self.path_panel_are_you_sure_overwrite_save.new_ref()) })
					else { godot_error!("Failed to get panel_are_you_sure"); return; };
			self.panel_are_you_sure_overwrite_save = Some(are_you_sure.clone().claim());

			are_you_sure.base().connect(panel_are_you_sure::signal_yes, owner_ref, "_on_panel_are_you_sure_overwrite_save_yes",
			                            VariantArray::new_shared(), ConnectFlags::DEFERRED.into()).report_on_err();
		}
	}

	#[method]
	fn _process(&self) {
		if Input::godot_singleton().is_action_just_pressed("ui_cancel", true) {
			self.panel_new_game.on_sane(|panel|panel.set_visible(false));
			self.panel_are_you_sure_overwrite_save.on_safe(|_, base| base.set_visible(false));
			//todo! also close load panel
		}
	}

	#[method]
	fn _on_background_gui_input(&self) {
		self.panel_new_game.on_sane(|panel|panel.set_visible(false));
		//todo! also close load panel
	}

	#[method]
	fn _on_fake_toggle_iron_gauntlet_pressed(&mut self) {
		self.iron_gauntlet_times_pressed = usize::clamp(self.iron_gauntlet_times_pressed + 1, 0, easters_iron_gauntlet::max_presses);

		let config = ConfigFile::new();
		config.load(crate::config_path).report_on_err();
		config.set_value("iron_gauntlet", "times_pressed", self.iron_gauntlet_times_pressed);
		config.save(crate::config_path).report_on_err();

		if self.iron_gauntlet_times_pressed >= easters_iron_gauntlet::max_presses {
			crash_game();
			return;
		}

		config.save(crate::config_path).report_on_err();

		let easter = easters_iron_gauntlet::get_easter(self.iron_gauntlet_times_pressed);
		self.set_easter_egg_text(easter);
	}

	#[method]
	fn _on_button_clicked_new_game(&self) {
		self.panel_new_game.on_sane(|panel| panel.set_visible(panel.is_visible().not()));
	}

	#[method]
	fn _on_button_clicked_start_game(&mut self) {
		let Some(save_name) = self.typed_save_name() else { return; };
		if save_name.len() == 0 {
			self.set_easter_egg_text("You need to enter a save name");
			return;
		}

		let save_file = SaveFile::new(save_name.clone());
		let saves_manager = SavesManager::godot_singleton();

		saves_manager.map_mut(|manager, _| {
			if manager.get_saves().get(&save_name).is_some() {
				self.panel_are_you_sure_overwrite_save.on_safe(|panel, base| {
					panel.set_text(format!("Are you sure you want to overwrite save: {}", save_name).as_str());
					base.set_visible(true);
				});
				self.line_edit_save_name.on_sane(|line_edit| line_edit.release_focus()); // to make sure the player can't edit the save name while the panel is open
			} else {
				manager.add_save(save_file.clone());
				self.start_game(save_file);
			}
		}).report_on_err();
	}

	#[method]
	fn _on_line_edit_changed_save_name(&mut self, new_text: GodotString) {
		let mut cleaned_text = new_text.to_string();
		cleaned_text.retain(|c| c.is_ascii_alphanumeric());

		if cleaned_text == "alexa" {
			crash_game();
			return;
		}

		easters_save_name::get_name_easter(cleaned_text.as_str())
				.on_some(|easter| self.set_easter_egg_text(easter));

		self.line_edit_save_name.on_sane(|line_edit| line_edit.set_text(cleaned_text));
	}

	fn typed_save_name(&self) -> Option<String> {
		return self.line_edit_save_name.map_on_sane(|line_edit| line_edit.text().to_string());
	}

	#[method]
	fn _on_panel_are_you_sure_overwrite_save_yes(&self) {
		let Some(save_name) = self.typed_save_name() else { return; };
		let save = SaveFile::new(save_name.to_string());
		SavesManager::godot_singleton().map_mut(|saves_manager, _| saves_manager.add_save(save.clone())).report_on_err();
		self.start_game(save);
	}

	fn start_game(&self, _save: SaveFile) {

	}

	fn set_easter_egg_text(&mut self, text: &str) {
		self.tween_easter_egg_text.on_safe(|tween| { if tween.is_valid() { tween.kill(); } });
		self.tween_easter_egg_text = None;

		self.label_easter_egg.on_sane(|label| {
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
			};
		});
	}
}