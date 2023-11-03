use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveFile {
	pub(super) name: String,
	pub(super) is_dirty: bool,
}

impl SaveFile {
	pub fn new(name: String) -> SaveFile {
		return SaveFile {
			name,
			is_dirty: true,
		};
	}

	pub fn name(&self) -> &String { return &self.name; }
}