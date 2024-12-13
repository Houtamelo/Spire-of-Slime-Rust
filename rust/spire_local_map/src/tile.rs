use super::*;

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
		Tile {
			contents: TileContents::default(),
			scout_status: TileScoutStatus::Hidden,
			mist_status: TileMistStatus::Mist_Hard,
			is_explored: false,
			biome: Biome { id: 0 },
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Biome {
	pub id: u8,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum TileContents {
	#[default]
	Empty,
	Obstacle,
	Event(EventID),
	Trap(EventID),
	Enemies(EnemyGroup),
	RestSite,
}

impl Tile {
	pub fn is_obstacle(&self) -> bool { matches!(self.contents, TileContents::Obstacle) }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileScoutStatus {
	Hidden,
	Visible,
	ContentsRevealed,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum TileMistStatus {
	NoMist_Permanent,
	NoMist_Temporary { turns_until_soft: Int },
	Mist_Soft { turns_until_hard: Int },
	Mist_Hard,
}

impl TileMistStatus {
	#[allow(unused)]
	pub const DEFAULT_TURNS_UNTIL_SOFT: Int = int!(4);

	#[allow(unused)]
	pub const DEFAULT_TURNS_UNTIL_HARD: Int = int!(4);
}

// todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EnemyGroup {}

//todo!
#[derive(Debug, Serialize, Deserialize)]
pub struct EventID {}
