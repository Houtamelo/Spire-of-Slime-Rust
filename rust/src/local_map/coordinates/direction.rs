use serde::{Deserialize, Serialize};
use crate::local_map::coordinates::axial::Axial;

pub static ALL: [HexagonDirection; 6] = [
	HexagonDirection::SouthEast,
	HexagonDirection::East,
	HexagonDirection::NorthEast,
	HexagonDirection::NorthWest,
	HexagonDirection::West,
	HexagonDirection::SouthWest
];

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HexagonDirection {
	SouthEast = 0,
	East = 1,
	NorthEast = 2,
	NorthWest = 3,
	West = 4,
	SouthWest = 5
}

impl HexagonDirection {
	pub fn to_axial_vector(self) -> Axial {
		return match self {
			HexagonDirection::SouthEast => Axial { q: 0, r: 1 },
			HexagonDirection::East => Axial { q: 1, r: 0},
			HexagonDirection::NorthEast => Axial { q: 1, r: -1},
			HexagonDirection::NorthWest => Axial { q: 0, r: -1},
			HexagonDirection::West => Axial { q: -1, r: 0},
			HexagonDirection::SouthWest => Axial { q: -1, r: 1},
		};
	}
}
