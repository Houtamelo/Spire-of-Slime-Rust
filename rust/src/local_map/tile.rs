use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
	pub contents: TileContents,
	pub scout_status: TileScoutStatus,
	pub mist_status: TileMistStatus,
	pub is_explored: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TileContents {
	Empty,
	Obstacle,
	Event(EventID),
	Trap(EventID),
	Enemies(EnemyGroup),
	RestSite,
}

impl Default for TileContents { fn default() -> Self { return TileContents::Empty; } }

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileScoutStatus {
	Hidden,
	Visible,
	ContentsRevealed,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TileMistStatus {
	NoMist_Permanent,
	NoMist_Temporary { turns_until_soft: u8 },
	Mist_Soft { turns_until_hard: u8 },
	Mist_Hard,
}

pub const DEFAULT_TURNS_UNTIL_SOFT: u8 = 4;
pub const DEFAULT_TURNS_UNTIL_HARD: u8 = 4;

// todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyGroup {
	
}

//todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EventID {}