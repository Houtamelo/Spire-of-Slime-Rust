use godot::classes::file_access::ModeFlags;

use super::*;

const DIRECTORY: &str = "user://save/";

#[derive(Default)]
pub struct SaveFilesController {
	saves: HashMap<String, SaveFile>,
}

impl SaveFilesController {
	pub fn load_saves_from_disk(&mut self) {
		self.saves.clear();

		let mut saves_dir = match DirAccess::open(DIRECTORY) {
			Some(folder) => folder,
			None => {
				DirAccess::make_dir_recursive_absolute(DIRECTORY);

				let open_err = DirAccess::get_open_error();
				return godot_print!(
					"Could not open save folder.\n\
						 Error: {open_err:?}"
				);
			}
		};

		match saves_dir.list_dir_begin() {
			Error::OK => {}
			err => return godot_error!("SavesFolder::list_dir_begin() failed.\n Error: {err:?}"),
		}

		while let file = saves_dir.get_next()
			&& !file.is_empty()
		{
			if !saves_dir.current_is_dir() {
				continue;
			}

			match read_save(format!("{file}/main.ron")) {
				Ok(save) => {
					self.saves.insert(save.name.clone(), save);
				}
				Err(err) => {
					godot_error!("{err}");
				}
			}
		}
	}

	pub const fn get_saves(&self) -> &HashMap<String, SaveFile> { &self.saves }

	pub fn get_save(&self, save_name: &str) -> Option<&SaveFile> { self.saves.get(save_name) }

	pub fn add_save(&mut self, save: SaveFile) {
		if self.saves.contains_key(&save.name) {
			godot_warn!("Trying to add a save that already exists: {}", save.name);
		}

		self.saves.insert(save.name.clone(), save);
		self.write_saves_to_disk().log_if_err();
	}

	pub fn delete_save(&mut self, save_name: &str) {
		if self.saves.remove(save_name).is_some() {
			let folder_path = format!("{}/{save_name}", DIRECTORY);
			let global_path = ProjectSettings::singleton().globalize_path(&folder_path);
			Os::singleton().move_to_trash(&global_path).log_if_err();
		} else {
			godot_warn!("Tried to delete a save that doesn't exist, named \"{}\"", save_name);
		}
	}

	pub fn overwrite_save(&mut self, mut save: SaveFile) {
		if self.saves.remove(&save.name).is_none() {
			godot_warn!("Tried to overwrite a save that doesn't exist, named \"{}\"", save.name);
		}

		save.is_dirty = true;
		self.saves.insert(save.name.clone(), save);
		self.write_saves_to_disk().log_if_err();
	}

	fn write_saves_to_disk(&mut self) -> Result<()> {
		self.saves
			.values_mut()
			.filter(|save| save.is_dirty)
			.try_for_each(|save| {
				let save_name = &save.name;
				let folder_path = GString::from(format!("{DIRECTORY}/{save_name}"));

				if !DirAccess::dir_exists_absolute(&folder_path) {
					match DirAccess::make_dir_recursive_absolute(&folder_path) {
						Error::OK => {}
						err => {
							bail!("Could not create save folder. \n Error: {err:?}");
						}
					}
				}

				let save_path = format!("{folder_path}/main.ron");
				backup_old_main(folder_path, save_path.clone()).log_if_err();
				write_save(save, save_path.clone())?;

				save.is_dirty = false;
				Ok(())
			})
	}
}

fn read_save(path: String) -> Result<SaveFile> {
	let raw_text = FileAccess::get_file_as_string(&path);

	if raw_text.is_empty() {
		bail!(
			"Failed to open save file at \"{path}\". \n\
			   Error: {:?}",
			FileAccess::get_open_error()
		);
	}

	ron::from_str(raw_text.to_string().as_str()).map_err(|err| {
		anyhow!(
			"Failed to parse save file at \"{path}\". \n\
			 Error: {err}"
		)
	})
}

fn write_save(save: &SaveFile, path: String) -> Result<()> {
	let save_ron = ron::to_string(&save)?;

	FileAccess::open(&path, ModeFlags::WRITE)
		.ok_or_else(|| {
			anyhow!(
				"Failed to open save file at \"{path}\". \n\
			 Error: {:?}",
				FileAccess::get_open_error()
			)
		})
		.map(|mut save_file| {
			save_file.store_string(&save_ron);
			save_file.close();
		})
}

#[allow(clippy::never_loop)]
fn backup_old_main(folder_path: GString, main_path: String) -> Result<()> {
	let old_main = read_save(main_path)?;
	let mut folder = DirAccess::open(&folder_path).ok_or_else(|| {
		anyhow!(
			"Could not open save folder. \n\
			 Error: {:?}",
			DirAccess::get_open_error()
		)
	})?;

	let backup_file_name = 'outer: loop {
		let mut oldest_file = None;
		for backup_index in 1..=50 {
			let file_name = format!("backup_{backup_index}.ron");
			if folder.file_exists(&file_name) {
				let file_path = format!("{folder_path}/{file_name}");
				let file_time = FileAccess::get_modified_time(&file_path);
				if let Some((_, oldest_time)) = oldest_file
					&& oldest_time <= file_time
				{
					continue;
				} else {
					oldest_file = Some((file_name, file_time));
				}
			} else {
				break 'outer file_name;
			}
		}

		break oldest_file
			.map(pluck!(.0))
			.unwrap_or_else(|| String::from("backup_1.ron"));
	};

	let backup_file_path = format!("{folder_path}/{backup_file_name}");
	write_save(&old_main, backup_file_path)
}
