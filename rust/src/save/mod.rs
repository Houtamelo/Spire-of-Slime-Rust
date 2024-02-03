pub mod file;
pub mod singleton;
pub mod affairs;
mod states;
mod upgrades;
mod stats;

use std::collections::HashSet;
use crate::WorldLocation;
use crate::save::file::SaveFile;


impl SaveFile {
	pub fn unlocked_locations(&self) -> HashSet<WorldLocation> {
		todo!();
	}
}