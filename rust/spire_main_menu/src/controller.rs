use gdnative_tweener::prelude::*;

use shared::node_utils::{InspectNode, SpawnAsInst, TryGetNode};

use crate::internal_prelude::*;

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
	references: Option<References>,
	iron_gauntlet_times_pressed: usize,
	tween_easter_egg_text: Option<TweenID<TweenProperty_f64>>,
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

#[derive(Debug)]
struct References {
	panel_new_game: Ref<Control>,
	panel_load_game: Ref<Control>,
	panel_credits: Ref<Control>,

	// Panel - NEW GAME
	label_easter_egg: Ref<Label>,
	line_edit_save_name: Ref<LineEdit>,
	panel_are_you_sure_overwrite_save: Instance<PanelAreYouSure>,

	// Panel - LOAD GAME
	container_load_buttons: Ref<Control>,
	prefab_load_button: Ref<PackedScene>,
}

impl References {
	fn load(owner: &Control) -> References {
		macro_rules! get_ref {
			($owner: ident :: <$node_ty: ty> ($path: literal)) => { unsafe {
				$owner.try_get_node::<$node_ty>($path).unwrap().assume_shared()
			}};
		}

		References {
			panel_new_game: get_ref!(owner::<Control>("panel_new-game")),
			panel_load_game: get_ref!(owner::<Control>("panel_load-game")),
			panel_credits: get_ref!(owner::<Control>("panel_credits")),
			label_easter_egg: get_ref!(owner::<Label>("panel_new-game/background_easter-egg/label_easter-egg")),
			line_edit_save_name: get_ref!(owner::<LineEdit>("panel_new-game/line-edit_save-name")),
			panel_are_you_sure_overwrite_save: unsafe {
				owner.get_node_as_instance("panel_new-game/panel_are-you-sure-overwrite-save")
				     .unwrap()
				     .claim()
			},
			container_load_buttons: get_ref!(owner::<Control>("panel_load-game/scroll-container_load-game/v_box_container")),
			prefab_load_button: load_resource_as("res://Core/Main Menu/UI/Load Game Panel/prefab_save-slot.tscn").unwrap(),
		}
	}
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

	fn get_refs(&mut self, owner: &Control) -> &References {
		self.references.get_or_insert_with(|| References::load(owner))
	}

	#[method]
	fn _ready(&mut self, #[base] owner: &Control) {
		self.grab_nodes_by_path(owner);
		self.references = Some(References::load(owner));

		owner.inspect_node::<Control>("background", |bg| {
			bg.connect_fn("gui_input", owner, fn_name(&Self::_background_gui_input));
		});
		
		owner.inspect_node::<Button>("panel_credits/button_return", |button| {
			button.connect_fn("pressed", &self.get_refs(owner).panel_credits, "hide");
		});
		
		owner.inspect_node::<Button>("button_discord", |button| {
			button.connect_fn_args("pressed", OS::godot_singleton(), 
			                       "shell_open", "https://discord.gg/Cacam7yuqR".to_shared_array());
		});
		
		owner.inspect_node::<Button>("button_subscribe-star", |button| {
			button.connect_fn_args("pressed", OS::godot_singleton(), 
			                       "shell_open", "https://subscribestar.adult/spire-of-slime-yonder".to_shared_array());
		});

		// Main Buttons
		{
			owner.inspect_node::<Button>("main-buttons/new-game", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_toggle_panel_new_game));
			});

			owner.inspect_node::<Button>("main-buttons/load-game", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_toggle_panel_load_game));
			});

			owner.inspect_node::<Button>("main-buttons/settings", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_open_settings_menu));
			});

			owner.inspect_node::<Button>("main-buttons/credits", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_toggle_panel_credits));
			});

			owner.inspect_node::<Button>("main-buttons/quit", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_quit_game));
			});
		}

		// Panel - NEW GAME
		{
			owner.inspect_node::<Button>("panel_new-game/button_start-game", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_button_pressed_start_game));
			});

			let config = ConfigFile::new();
			config.load(shared::CONFIG_PATH)
			      .map_err(|_| {
				      config.set_value("iron_gauntlet", "times_pressed", 0);
				      config.save(shared::CONFIG_PATH).log_if_err();
			      }).ok();

			self.iron_gauntlet_times_pressed =
				config.get_value("iron_gauntlet", "times_pressed", 0)
				      .to::<usize>()
				      .unwrap_or_else(|| {
					      godot_error!("Failed to get iron gauntlet times pressed from config.cfg");
					      0
				      });

			owner.inspect_node::<Button>("panel_new-game/fake-toggle_iron-gauntlet", |button| {
				button.connect_fn("pressed", owner, fn_name(&Self::_toggle_pressed_fake_iron_gauntlet));
			});

			self.get_refs(owner)
			    .panel_are_you_sure_overwrite_save
			    .connect_fn(panel_are_you_sure::SIGNAL_YES, owner, fn_name(&Self::_are_you_sure_overwrite_save_yes));
		}
	}

	pub fn create_and_assign_load_buttons(&mut self, owner: &Control, saves: impl IntoIterator<Item = &str>) {
		fn spawn_load_button(
			owner: &Control,
			prefab: &PackedScene,
			container: &Control
		) -> Instance<LoadButton> {
			let node = prefab.spawn_as_inst().unwrap();

			container.add_child(node.base(), false);

			node.connect_fn(load_button::SIGNAL_LOAD, owner, fn_name(&MainMenuController::_button_pressed_save_slot_load));
			node.connect_fn(load_button::SIGNAL_DELETE, owner, fn_name(&MainMenuController::_button_pressed_save_slot_delete));

			node.claim()
		}

		let refs = self.references.get_or_insert_with(|| References::load(owner));
		let container = unsafe { refs.container_load_buttons.assume_safe() };
		let prefab = unsafe { refs.prefab_load_button.assume_safe() };

		let save_count =
			saves.into_iter()
			     .enumerate()
			     .fold(0, |_, (idx, save)| {
				     let button =
					     match self.spawned_load_buttons.get_mut(idx) {
						     Some(button) => button,
						     None => {
							     let button = spawn_load_button(owner, &prefab, &container);
							     self.spawned_load_buttons.push(button);
							     self.spawned_load_buttons.last_mut().unwrap()
						     }
					     };

				     button.touch_assert_safe_mut(|button, base| {
					     button.save_name = Some(save.to_owned());
					     base.set_visible(true);
				     });

				     idx + 1
			     });

		self.spawned_load_buttons
		    .iter()
		    .skip(save_count)
		    .for_each(|button| {
			    button.touch_assert_safe_mut(|button, base| {
				    button.save_name = None;
				    base.set_visible(false);
			    });
		    });
	}

	#[method]
	fn _unhandled_input(&mut self, #[base] owner: &Control, event_ref: Ref<InputEvent>) {
		if shared::input::any_cancel_input(unsafe { &event_ref.assume_safe() }) {
			self.close_all_panels(owner);
		}
	}
	
	#[method]
	fn _background_gui_input(&mut self, #[base] owner: &Control, event: Ref<InputEvent>) {
		let event = unsafe { event.assume_safe() };
		if let Some(mouse_event) = event.cast::<InputEventMouseButton>() 
			&& !mouse_event.is_echo() 
			&& mouse_event.is_pressed()
			&& mouse_event.button_index() == GlobalConstants::BUTTON_LEFT {
			self.close_all_panels(owner);
		}
	}

	#[method]
	fn close_all_panels(&mut self, #[base] owner: &Control) {
		let refs = self.get_refs(owner);

		refs.panel_new_game.touch_assert_sane(|panel| {
			panel.set_visible(false);
		});
		refs.panel_load_game.touch_assert_sane(|panel| {
			panel.set_visible(false);
		});
		unsafe {
			refs.panel_are_you_sure_overwrite_save
			    .assume_safe()
			    .base()
			    .set_visible(false);
		}
	}

	#[method]
	fn _toggle_pressed_fake_iron_gauntlet(&mut self, #[base] owner: &Control) {
		self.iron_gauntlet_times_pressed += 1;

		let config = ConfigFile::new();

		config.load(shared::CONFIG_PATH)
		      .log_if_err();

		config.set_value("iron_gauntlet", "times_pressed", self.iron_gauntlet_times_pressed);

		config.save(shared::CONFIG_PATH)
		      .log_if_err();

		if self.iron_gauntlet_times_pressed >= easters_iron_gauntlet::MAX_PRESSES {
			crash_game();
		} else {
			let easter = easters_iron_gauntlet::get_easter(self.iron_gauntlet_times_pressed);
			self.set_easter_egg_text(owner, easter);
		}
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
		if save_name.is_empty() {
			self.set_easter_egg_text(owner, "You need to enter a save name");
			return;
		}

		let save_already_exists =
			self.spawned_load_buttons
			    .iter()
			    .any(|button_ref|
				    Some(true) == button_ref.map_assert_safe(|button, _| {
					    button.save_name.as_ref() == Some(&save_name)
				    })
			    );

		if !save_already_exists {
			owner.emit_signal(SIGNAL_NEW_GAME, &[save_name.to_variant()]);
		} else {
			let refs = self.get_refs(owner);

			refs.panel_are_you_sure_overwrite_save
			    .touch_assert_safe(|panel, base| {
				    panel.set_text(format!("Are you sure you want to overwrite save: {}", save_name).as_str());
				    base.set_visible(true);
			    });

			refs.line_edit_save_name.touch_assert_sane(|line_edit| {
				line_edit.release_focus(); // to make sure the player can't edit the save name while the `are you sure panel` is open
			});
		}
	}

	#[method]
	fn _on_line_edit_changed_save_name(&mut self, #[base] owner: &Control, new_text: GodotString) {
		let cleaned_text = {
			let mut temp = new_text.to_string();
			temp.retain(|char| char.is_ascii_alphanumeric());
			temp.chars()
			    .map(|char| char.to_ascii_lowercase())
			    .collect::<String>()
		};

		if cleaned_text == "alexa" {
			crash_game();
		} else {
			let easter_text = 
				easters_save_name::get_name_easter(cleaned_text.as_str()).unwrap_or_default();
			self.set_easter_egg_text(owner, easter_text);
		}
	}

	fn typed_save_name(&self) -> String {
		self.references.as_ref().map(|refs| {
			refs.line_edit_save_name
			    .unwrap_manual()
			    .text()
			    .to_string()
		}).unwrap()
	}

	#[method]
	fn _are_you_sure_overwrite_save_yes(&self, #[base] owner: &Control) {
		let save_name = self.typed_save_name();
		owner.emit_signal(SIGNAL_OVERWRITE_SAVE_AND_START, &[save_name.to_variant()]);
	}

	fn set_easter_egg_text(&mut self, owner: &Control, text: &str) {
		if let Some(tween) = self.tween_easter_egg_text.as_ref() { 
			tween.kill().log_if_err();
		}

		let label = self.get_refs(owner).label_easter_egg.unwrap_manual();
		label.set_text(text);
		label.set_percent_visible(0.0);

		self.tween_easter_egg_text =
			label.do_percent_visible(1.0, text.len() as f64 * 0.015)
			     .register()
			     .map_err(|err| {
				     godot_error!("{}():\n\
					 Failed to tween easter egg text: {}", 
					 fn_name(&Self::set_easter_egg_text), err);
			     }).ok();
	}

	#[method]
	fn _toggle_panel_new_game(&mut self, #[base] owner: &Control) {
		self.get_refs(owner).panel_new_game.touch_assert_sane(|panel| {
			panel.set_visible(!panel.is_visible());
		});
	}

	#[method]
	fn _toggle_panel_load_game(&mut self, #[base] owner: &Control) {
		self.get_refs(owner).panel_load_game.touch_assert_sane(|panel| {
			panel.set_visible(!panel.is_visible());
		});
	}

	#[method]
	fn _open_settings_menu(&self, #[base] owner: &Control) {
		owner.emit_signal(SIGNAL_OPEN_SETTINGS_MENU, &[]);
	}

	#[method]
	fn _toggle_panel_credits(&mut self, #[base] owner: &Control) {
		self.get_refs(owner).panel_credits.touch_assert_sane(|panel| {
			panel.set_visible(!panel.is_visible());
		});
	}

	#[method]
	fn _quit_game(&self, #[base] owner: &Control) {
		owner.get_tree().unwrap_manual().quit(0);
	}
}