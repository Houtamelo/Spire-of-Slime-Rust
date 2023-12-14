pub mod file;

use gdnative::prelude::*;
use gdnative::api::*;
use file::SaveFile;
use houta_utils_gdnative::prelude::*;
use gdrust_export_nodepath::extends;
use std::collections::HashMap;

static save_dir: &str = "user://saves/";

#[extends(Node)]
pub struct SavesManager {
	saves: HashMap<String, SaveFile>,
}

#[methods]
impl SavesManager {
	pub fn godot_singleton() -> TInstance<'static, SavesManager, Shared> {
		let engine = Engine::godot_singleton();
		let singleton_base_obj = engine.get_singleton("SavesManager").expect("Failed to get singleton SavesManager");
		let singleton_tref = unsafe { singleton_base_obj.assume_safe() };
		let singleton_node = singleton_tref.cast::<Node>().expect("Failed to cast singleton SavesManager to Node");
		return singleton_node.cast_instance::<SavesManager>().expect("Failed to cast singleton SavesManager to SavesManager");
	}

	#[method]
	fn _ready(&mut self, #[base] _owner: &Node) {
		let save_folder_path = save_dir;
		let save_folder = Directory::new();

		if save_folder.open(save_folder_path).is_err() {
			if let Err(error) = save_folder.make_dir_recursive(save_folder_path) {
				godot_error!("Failed to create saves folder: {}", error);
			}

			return;
		}

		if let Err(error) = save_folder.list_dir_begin(true, true) {
			godot_error!("Failed to get files in saves folder: {}", error);
			return;
		}

		while let child_name = save_folder.get_next() && child_name.len() > 0 {
			if false == save_folder.current_is_dir() {
				continue;
			}

			let child_path = format!("{save_folder_path}/{child_name}");
			let child_folder = Directory::new();
			if let Err(error) = child_folder.open(child_path.as_str()) {
				godot_error!("Failed to open saves folder that was listed: {}", error);
				continue;
			}

			let save_file_path = format!("{child_path}/main.ron");
			if false == child_folder.file_exists(save_file_path.as_str()) {
				continue;
			}

			let save_file = File::new();
			if let Err(error) = save_file.open(save_file_path.as_str(), File::READ) {
				godot_error!("Failed to open saves file: {}", error);
				continue;
			}

			let save_ron = save_file.get_as_text(false).to_string();
			match ron::from_str::<SaveFile>(save_ron.as_str()) {
				Ok(save) => { self.saves.insert(save.name.clone(), save); },
				Err(err) => { godot_error!("Failed to deserialize saves at path: {save_file_path}, contents: {err}"); }
			}
		}
	}

	pub fn get_saves(&self) -> &HashMap<String, SaveFile> {
		return &self.saves;
	}

	pub fn add_save(&mut self, save: SaveFile) {
		if self.saves.get(&save.name).is_some() {
			godot_warn!("Trying to add a saves that already exists: {}", save.name);
		}

		self.saves.insert(save.name.clone(), save);
		self.write_saves_to_disk().log_if_err();
	}

	pub fn delete_save(&mut self, save_name: &str) {
		if self.saves.remove(save_name).is_none() {
			godot_warn!("Trying to delete a saves that doesn't exist: {}", save_name);
			return;
		}

		let save_path = format!("{save_dir}/{save_name}");
		let global_save_path = ProjectSettings::godot_singleton().globalize_path(save_path);
		OS::godot_singleton().move_to_trash(global_save_path).log_if_err();
	}

	pub fn overwrite_save(&mut self, mut save: SaveFile) {
		if self.saves.remove(&save.name).is_none() {
			godot_warn!("Trying to overwrite a saves that doesn't exist: {}", save.name);
		}

		save.is_dirty = true;
		self.saves.insert(save.name.clone(), save);
		self.write_saves_to_disk().log_if_err();
	}

	fn write_saves_to_disk(&mut self) -> Result<(), GodotError> {
		for save in self.saves.values_mut().filter(|save| save.is_dirty) {
			save.is_dirty = false;
			match ron::to_string(&save) {
				Ok(save_ron) => {
					let save_name = &save.name;
					let exclusive_folder_path = format!("{save_dir}/{save_name}");
					let exclusive_folder = Directory::new();
					if exclusive_folder.open(exclusive_folder_path.as_str()).is_err() {
						exclusive_folder.make_dir_recursive(exclusive_folder_path.as_str())?;
					}

					let save_file_name = "main.ron";
					let save_file_path = format!("{exclusive_folder_path}/{save_file_name}");

					backup_old_main(exclusive_folder_path.as_str(), &exclusive_folder, save_file_path.as_str()).log_if_err();

					let save_file = File::new();
					save_file.open(save_file_path, File::WRITE)?;
					save_file.store_string(save_ron.as_str());
					save_file.close();
				},
				Err(err) => {
					godot_error!("Failed to serialize saves: {}", err);
					save.is_dirty = true;
				}
			}
		}

		return Ok(());

		fn backup_old_main(exclusive_folder_path: &str, exclusive_folder: &Ref<Directory, Unique>, save_file_path: &str) -> Result<(), GodotError> {
			let save_file = File::new();
			save_file.open(save_file_path, File::READ)?;

			let old_main_save = save_file.get_as_text(false).to_string();

			let dummy_file = File::new();

			let backup_file_name = 'outer: loop {
				let mut oldest_file : Option<(String, i64)> = None;
				for backup_index in 1..=50 {
					let file_name = format!("backup_{backup_index}.ron");
					if exclusive_folder.file_exists(file_name.as_str()) {
						let file_path = format!("{exclusive_folder_path}/{file_name}");
						let file_time = dummy_file.get_modified_time(file_path);
						if let Some((_, oldest_time)) = oldest_file && oldest_time <= file_time {
							continue;
						} else {
							oldest_file = Some((file_name, file_time));
						}
					} else {
						break 'outer file_name;
					}
				}

				break oldest_file.unwrap().0;
			};

			let backup_file_path = format!("{exclusive_folder_path}/{backup_file_name}");
			let backup_file = File::new();
			backup_file.open(backup_file_path, File::WRITE)?;
			backup_file.store_string(old_main_save);
			backup_file.close();

			return Ok(());
		}
	}
}