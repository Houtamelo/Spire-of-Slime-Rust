#[allow(unused_imports)]
use crate::*;
use crate::save::file::SaveFile;
use world_map::WorldLocation;

pub mod file;
pub mod affairs;
mod states;
mod upgrades;
mod stats;

mod controller;
pub use controller::SaveFilesController;

impl SaveFile {
	pub fn unlocked_locations(&self) -> HashSet<WorldLocation> {
		todo!();
	}
}