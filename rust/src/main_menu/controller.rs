use std::collections::HashMap;
use gdnative::api::*;
use gdnative::prelude::*;
use gdnative_export_node_as_path::extends;
use houta_utils::fn_name;
use houta_utils_gdnative::prelude::*;
use crate::save::file::SaveFile;
use crate::{CONFIG_PATH, util};

use crate::util::panel_are_you_sure;
use crate::util::panel_are_you_sure::PanelAreYouSure;

use super::{
	easters_iron_gauntlet,
	easters_save_name,
	load_button,
	LoadButton,
};

pub static SIGNAL_NEW_GAME: &str = "new_game";
pub static SIGNAL_LOAD_GAME: &str = "load_game";
pub static SIGNAL_DELETE_SAVE: &str = "delete_save";
pub static SIGNAL_OVERWRITE_SAVE_AND_START: &str = "overwrite_save_and_start";
pub static SIGNAL_OPEN_SETTINGS_MENU: &str = "open_settings_menu";

#[extends(Control)]
#[register_with(Self::register)]
#[derive(Debug)]
pub struct MainMenuController {
	#[export_path] background: Option<Ref<Control>>,

	#[export_path] button_new_game : Option<Ref<Button>>,
	#[export_path] button_load_game: Option<Ref<Button>>,
	#[export_path] button_settings : Option<Ref<Button>>,
	#[export_path] button_credit: Option<Ref<Button>>,
	#[export_path] button_exit: Option<Ref<Button>>,

	#[export_path] panel_new_game: Option<Ref<Control>>,
	#[export_path] panel_load_game: Option<Ref<Control>>,
	#[export_path] panel_credits: Option<Ref<Control>>,
	
	// Panel - NEW GAME
	#[export_path] fake_toggle_iron_gauntlet: Option<Ref<Button>>,
	iron_gauntlet_times_pressed: usize,
	#[export_path] label_easter_egg: Option<Ref<Label>>, 
	tween_easter_egg_text: Option<Ref<SceneTreeTween>>,
	#[export_path] line_edit_save_name: Option<Ref<LineEdit>>,
	#[export_path] button_start_game  : Option<Ref<Button>>,
	#[export_path] panel_are_you_sure_overwrite_save: Option<Instance<PanelAreYouSure>>,

	// Panel - LOAD GAME
	#[export_path] container_load_buttons: Option<Ref<Control>>,
	#[property] prefab_load_button: Option<Ref<PackedScene>>,
	spawned_load_buttons: Vec<Instance<LoadButton>>,
}

fn crash_game() {
	Engine::godot_singleton()
		.get_main_loop()
		.unwrap_manual()
		.cast::<SceneTree>()
		.unwrap()
		.quit(0);
}

#[methods]
impl MainMenuController {
	fn register(builder: &ClassBuilder<Self>) {
		builder.signal(SIGNAL_NEW_GAME)
			.with_param("save_name", VariantType::GodotString)
			.done();
		
		builder.signal(SIGNAL_LOAD_GAME)
			.with_param("save_name", VariantType::GodotString)
			.done();
		
		builder.signal(SIGNAL_DELETE_SAVE)
			.with_param("save_name", VariantType::GodotString)
			.done();
		
		builder.signal(SIGNAL_OVERWRITE_SAVE_AND_START)
			.with_param("save_name", VariantType::GodotString)
			.done();
		
		builder.signal(SIGNAL_OPEN_SETTINGS_MENU)
			.done();
	}
	
	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		let owner_ref = unsafe { owner.assume_shared() };

		self.background
			.unwrap_manual()
			.connect("gui_input", owner_ref, fn_name(&Self::close_all_panels), 
				VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();

		// Menu Buttons
		{
			self.button_new_game
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_open_panel_new_game),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
			self.button_load_game
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_open_panel_load_game), 
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
			self.button_settings
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_open_settings_menu),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
			self.button_credit
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_open_panel_credits), 
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
			self.button_exit
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_exit_game),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		}

		// Panel - NEW GAME
		{
			self.button_start_game
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_button_pressed_start_game),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();

			let config = ConfigFile::new();
			config.load(CONFIG_PATH)
				.touch_if_err(|err| {
					godot_error!("Failed to load config.cfg: {}", err);
					config.set_value("iron_gauntlet", "times_pressed", 0);
					config.save(CONFIG_PATH).log_if_err();
				});

			self.iron_gauntlet_times_pressed = config.get_value("iron_gauntlet", "times_pressed", 0)
				.to::<usize>()
				.unwrap();
			
			self.fake_toggle_iron_gauntlet
				.unwrap_manual()
				.connect("pressed", owner_ref, fn_name(&Self::_toggle_pressed_fake_iron_gauntlet), 
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();

			self.panel_are_you_sure_overwrite_save
				.unwrap_inst()
				.base()
				.connect(panel_are_you_sure::SIGNAL_YES, owner_ref, fn_name(&Self::_are_you_sure_overwrite_save_yes), 
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				.log_if_err();
		}
	}
	
	pub fn create_and_assign_load_buttons(&mut self, owner: TRef<Control>, saves: &HashMap<String, SaveFile>) {
		let owner_ref = unsafe { owner.assume_shared() };
		let container= self.container_load_buttons.unwrap_manual();
		let prefab= self.prefab_load_button.unwrap_refcount();

		let save_count = saves.len();
		(self.spawned_load_buttons.len()..=save_count)
			.into_iter()
			.for_each(|_| {
				let node = unsafe {
					prefab.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
					      .expect("Failed to create LoadButton node from packed scene")
					      .assume_safe_if_sane()
					      .expect("Failed to assume safe and sane LoadButton node from packed scene")
				};

				container.add_child(node, false);
				
				node.connect(load_button::SIGNAL_LOAD, owner_ref, fn_name(&Self::_button_pressed_save_slot_load),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				    .log_if_err();
				node.connect(load_button::SIGNAL_DELETE, owner_ref, fn_name(&Self::_button_pressed_save_slot_delete),
					VariantArray::new_shared(), Object::CONNECT_DEFERRED)
				    .log_if_err();
				
				let control = node
					.cast::<Control>()
					.expect("Failed to cast LoadButton node to Control");
				let instance = control
					.cast_instance::<LoadButton>()
					.expect("Failed to cast LoadButton control to LoadButton");
				self.spawned_load_buttons.push(instance.claim());
			});

		self.spawned_load_buttons
			.iter()
			.zip(saves.keys())
			.for_each(|(button_ref, save_name)| {
				button_ref.touch_assert_safe_mut(|button, base| {
					button.save_name = Some(save_name.clone());
					base.set_visible(true);
				})
			});

		self.spawned_load_buttons
			.iter()
			.skip(save_count)
			.for_each(|button_ref| {
				button_ref.touch_assert_safe_mut(|button, base| {
					button.save_name = None;
					base.set_visible(false);
				});
			});
	}
	
	#[method]
	fn _unhandled_input(&self, event_ref: Ref<InputEvent>) {
		let event = unsafe { event_ref.assume_safe() };
		if util::any_cancel_input(&event) {
			self.close_all_panels();
		}
	}

	#[method]
	fn close_all_panels(&self) {
		self.panel_new_game
			.unwrap_manual()
			.set_visible(false);
		self.panel_are_you_sure_overwrite_save
			.touch_assert_safe(|_, base| 
				base.set_visible(false));
		self.panel_load_game
			.unwrap_manual()
			.set_visible(false);
	}

	#[method]
	fn _toggle_pressed_fake_iron_gauntlet(&mut self) {
		self.iron_gauntlet_times_pressed += 1;

		let config = ConfigFile::new();
		
		config.load(CONFIG_PATH)
			.log_if_err();
		
		config.set_value("iron_gauntlet", "times_pressed", self.iron_gauntlet_times_pressed);
		
		config.save(CONFIG_PATH)
			.log_if_err();

		if self.iron_gauntlet_times_pressed >= easters_iron_gauntlet::MAX_PRESSES {
			crash_game();
			return;
		}
		
		let easter = easters_iron_gauntlet::get_easter(self.iron_gauntlet_times_pressed);
		self.set_easter_egg_text(easter);
	}

	#[method]
	fn _button_pressed_save_slot_load(&self, #[base] owner: &Control, save_name: Variant) {
		owner.emit_signal(SIGNAL_LOAD_GAME, &[save_name]);
	}

	#[method]
	fn _button_pressed_save_slot_delete(&mut self, #[base] owner: &Control, save_name: Variant) {
		owner.emit_signal(SIGNAL_DELETE_SAVE, &[save_name]);
	}

	#[method]
	fn _button_pressed_start_game(&mut self, #[base] owner: &Control) {
		let save_name = self.typed_save_name();
		if save_name.len() == 0 {
			self.set_easter_egg_text("You need to enter a save name");
			return;
		}
		
		let save_already_exists = self.spawned_load_buttons
			.iter()
			.any(|button_ref|
				Some(true) == button_ref
					.map_assert_safe(|button, _|
						button.save_name.as_ref() == Some(&save_name))
			);

		if save_already_exists {
			self.panel_are_you_sure_overwrite_save
				.touch_assert_safe(|panel, base| {
					panel.set_text(format!("Are you sure you want to overwrite save: {}", save_name).as_str());
					base.set_visible(true);
				});

			self.line_edit_save_name
				.unwrap_manual()
				.release_focus();  // to make sure the player can't edit the save name while the are you sure panel is open
		} else {
			owner.emit_signal(SIGNAL_NEW_GAME, &[save_name.to_variant()]);
		}
	}

	#[method]
	fn _on_line_edit_changed_save_name(&mut self, new_text: GodotString) {
		let cleaned_text = { 
			let mut temp = new_text.to_string();
			temp.retain(|char|
				char.is_ascii_alphanumeric());
			temp.chars()
			    .map(|char| char.to_ascii_lowercase())
			    .collect::<String>()
		};

		if cleaned_text == "alexa" {
			crash_game();
		} else {
			let easter_text = easters_save_name::get_name_easter(cleaned_text.as_str())
				.unwrap_or_default();
			self.set_easter_egg_text(easter_text);
		}
	}

	fn typed_save_name(&self) -> String {
		return self.line_edit_save_name
			.unwrap_manual()
			.text()
			.to_string();
	}

	#[method]
	fn _are_you_sure_overwrite_save_yes(&self, #[base] owner: &Control) {
		let save_name = self.typed_save_name();
		owner.emit_signal(SIGNAL_OVERWRITE_SAVE_AND_START, &[save_name.to_variant()]);
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
				godot_error!("{}():\n\
					Failed to create tween for easter egg text", 
					fn_name(&Self::set_easter_egg_text));
			}
		}
	}

	#[method]
	fn _open_panel_new_game(&self) {
		self.panel_new_game
			.touch_assert_sane(|panel| 
				panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _open_panel_load_game(&self) {
		self.panel_load_game
			.touch_assert_sane(|panel| 
				panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _open_settings_menu(&self, #[base] owner: &Control) {
		owner.emit_signal(SIGNAL_OPEN_SETTINGS_MENU, &[]);
	}

	#[method]
	fn _open_panel_credits(&self) {
		self.panel_credits
			.touch_assert_sane(|panel| 
				panel.set_visible(!panel.is_visible()));
	}

	#[method]
	fn _exit_game(&self) {
		self.background
			.unwrap_manual()
			.get_tree()
			.unwrap_manual()
			.quit(0);
	}
}