use crate::internal_prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
	pub contents: TileContents,
	pub scout_status: TileScoutStatus,
	pub mist_status: TileMistStatus,
	pub is_explored: bool,
	pub biome: Biome,
}

impl Default for Tile {
	fn default() -> Self {
		return Tile {
			contents: TileContents::default(),
			scout_status: TileScoutStatus::Hidden,
			mist_status: TileMistStatus::Mist_Hard,
			is_explored: false,
			biome: Biome { id: 0 },
		};
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biome { pub id: u8 }

#[derive(Debug, Serialize, Deserialize)]
pub enum TileContents {
	Empty,
	Obstacle,
	Event(EventID),
	Trap(EventID),
	Enemies(EnemyGroup),
	RestSite,
}

impl Tile {
	pub fn is_obstacle(&self) -> bool {
		return matches!(self.contents, TileContents::Obstacle);
	}
}

impl Default for TileContents { fn default() -> Self { return TileContents::Empty; } }

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileScoutStatus {
	Hidden,
	Visible,
	ContentsRevealed,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone)]
pub enum TileMistStatus {
	NoMist_Permanent,
	NoMist_Temporary { turns_until_soft: u8 },
	Mist_Soft { turns_until_hard: u8 },
	Mist_Hard,
}

#[allow(unused)]
pub const DEFAULT_TURNS_UNTIL_SOFT: u8 = 4;

#[allow(unused)]
pub const DEFAULT_TURNS_UNTIL_HARD: u8 = 4;

// todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyGroup {
	
}

//todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EventID {}