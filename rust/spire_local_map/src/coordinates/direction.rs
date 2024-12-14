use super::*;

#[derive(
	Default,
	Debug,
	Copy,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Serialize,
	Deserialize,
	GodotConvert,
	Var,
	Export,
)]
#[godot(via = u8)]
#[repr(u8)]
pub enum HexagonDirection {
	#[default]
	SouthEast = 0,
	East = 1,
	NorthEast = 2,
	NorthWest = 3,
	West = 4,
	SouthWest = 5,
}

#[allow(unused)]
impl HexagonDirection {
	pub const ALL: &'static [HexagonDirection] = &[
		HexagonDirection::SouthEast,
		HexagonDirection::East,
		HexagonDirection::NorthEast,
		HexagonDirection::NorthWest,
		HexagonDirection::West,
		HexagonDirection::SouthWest,
	];

	pub const fn to_axial_vector(self) -> Axial {
		match self {
			HexagonDirection::SouthEast => Axial { q: 0, r: 1 },
			HexagonDirection::East => Axial { q: 1, r: 0 },
			HexagonDirection::NorthEast => Axial { q: 1, r: -1 },
			HexagonDirection::NorthWest => Axial { q: 0, r: -1 },
			HexagonDirection::West => Axial { q: -1, r: 0 },
			HexagonDirection::SouthWest => Axial { q: -1, r: 1 },
		}
	}

	pub const fn reverse(&self) -> HexagonDirection {
		match self {
			HexagonDirection::SouthEast => HexagonDirection::NorthWest,
			HexagonDirection::East => HexagonDirection::West,
			HexagonDirection::NorthEast => HexagonDirection::SouthWest,
			HexagonDirection::NorthWest => HexagonDirection::SouthEast,
			HexagonDirection::West => HexagonDirection::East,
			HexagonDirection::SouthWest => HexagonDirection::NorthEast,
		}
	}

	pub const fn arc(&self) -> [HexagonDirection; 3] {
		match self {
			HexagonDirection::SouthEast => {
				[
					HexagonDirection::SouthEast,
					HexagonDirection::East,
					HexagonDirection::SouthWest,
				]
			}
			HexagonDirection::East => {
				[
					HexagonDirection::East,
					HexagonDirection::NorthEast,
					HexagonDirection::SouthEast,
				]
			}
			HexagonDirection::NorthEast => {
				[
					HexagonDirection::NorthEast,
					HexagonDirection::NorthWest,
					HexagonDirection::East,
				]
			}
			HexagonDirection::NorthWest => {
				[
					HexagonDirection::NorthWest,
					HexagonDirection::West,
					HexagonDirection::NorthEast,
				]
			}
			HexagonDirection::West => {
				[
					HexagonDirection::West,
					HexagonDirection::SouthWest,
					HexagonDirection::NorthWest,
				]
			}
			HexagonDirection::SouthWest => {
				[
					HexagonDirection::SouthWest,
					HexagonDirection::SouthEast,
					HexagonDirection::West,
				]
			}
		}
	}

	pub const fn arc_axial(&self) -> [Axial; 3] {
		match self {
			HexagonDirection::SouthEast => {
				[
					HexagonDirection::SouthEast.to_axial_vector(),
					HexagonDirection::East.to_axial_vector(),
					HexagonDirection::SouthWest.to_axial_vector(),
				]
			}
			HexagonDirection::East => {
				[
					HexagonDirection::East.to_axial_vector(),
					HexagonDirection::NorthEast.to_axial_vector(),
					HexagonDirection::SouthEast.to_axial_vector(),
				]
			}
			HexagonDirection::NorthEast => {
				[
					HexagonDirection::NorthEast.to_axial_vector(),
					HexagonDirection::NorthWest.to_axial_vector(),
					HexagonDirection::East.to_axial_vector(),
				]
			}
			HexagonDirection::NorthWest => {
				[
					HexagonDirection::NorthWest.to_axial_vector(),
					HexagonDirection::West.to_axial_vector(),
					HexagonDirection::NorthEast.to_axial_vector(),
				]
			}
			HexagonDirection::West => {
				[
					HexagonDirection::West.to_axial_vector(),
					HexagonDirection::SouthWest.to_axial_vector(),
					HexagonDirection::NorthWest.to_axial_vector(),
				]
			}
			HexagonDirection::SouthWest => {
				[
					HexagonDirection::SouthWest.to_axial_vector(),
					HexagonDirection::SouthEast.to_axial_vector(),
					HexagonDirection::West.to_axial_vector(),
				]
			}
		}
	}
}

impl TryFrom<u8> for HexagonDirection {
	type Error = String;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(HexagonDirection::SouthEast),
			1 => Ok(HexagonDirection::East),
			2 => Ok(HexagonDirection::NorthEast),
			3 => Ok(HexagonDirection::NorthWest),
			4 => Ok(HexagonDirection::West),
			5 => Ok(HexagonDirection::SouthWest),
			_ => Err(format!("Invalid HexagonDirection value: {value}")),
		}
	}
}
