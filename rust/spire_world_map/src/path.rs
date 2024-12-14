use super::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct WorldPath(WorldLocation, WorldLocation);

impl WorldPath {
	pub const fn new(a: WorldLocation, b: WorldLocation) -> Option<Self> {
		let a_int = a as i64;
		let b_int = b as i64;

		if a_int != b_int {
			Some(Self(a, b))
		} else {
			None
		}
	}

	pub fn point_a(&self) -> WorldLocation { self.0 }
	pub fn point_b(&self) -> WorldLocation { self.1 }

	pub fn contains(&self, location: WorldLocation) -> bool {
		self.0 == location || self.1 == location
	}
}

impl PartialEq for WorldPath {
	fn eq(&self, other: &Self) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
	}
}

impl Eq for WorldPath {}

impl Hash for WorldPath {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let id = match (self.0, self.1) {
			| (WorldLocation::Chapel, WorldLocation::Grove)
			| (WorldLocation::Grove, WorldLocation::Chapel) => 0,
			| (WorldLocation::Grove, WorldLocation::Forest)
			| (WorldLocation::Forest, WorldLocation::Grove) => 1,
			| (WorldLocation::Forest, WorldLocation::Cave)
			| (WorldLocation::Cave, WorldLocation::Forest) => 2,
			| (WorldLocation::Cave, WorldLocation::Chapel)
			| (WorldLocation::Chapel, WorldLocation::Cave) => 3,
			| (WorldLocation::Chapel, WorldLocation::Forest)
			| (WorldLocation::Forest, WorldLocation::Chapel) => 4,
			| (WorldLocation::Grove, WorldLocation::Cave)
			| (WorldLocation::Cave, WorldLocation::Grove) => 5,
			| (WorldLocation::Chapel, WorldLocation::Chapel)
			| (WorldLocation::Grove, WorldLocation::Grove)
			| (WorldLocation::Forest, WorldLocation::Forest)
			| (WorldLocation::Cave, WorldLocation::Cave) => {
				godot_warn!(
					"WorldPath::hash(): WorldPath cannot start and end at the same spot: {self:?}"
				);
				69
			}
		};

		id.hash(state);
	}
}

impl GodotConvert for WorldPath {
	type Via = Array<i64>;
}

impl ToGodot for WorldPath {
	type ToVia<'v> = Self::Via;

	fn to_godot(&self) -> Self::Via { array![self.0.to_godot(), self.1.to_godot()] }
}

impl FromGodot for WorldPath {
	fn try_from_godot(array: Self::Via) -> Result<Self, ConvertError> {
		if array.len() != 2 {
			return Err(ConvertError::new(format!(
				"WorldPath::try_from_godot(): Array length must be 2, got {array:?}"
			)));
		}

		let (a_int, b_int) = (array.get(0).unwrap(), array.get(1).unwrap());
		let (a, b) = (WorldLocation::try_from_godot(a_int)?, WorldLocation::try_from_godot(b_int)?);

		Self::new(a, b).ok_or_else(|| {
			ConvertError::new(format!("The path endpoints cannot be the same. Got: {a:?}"))
		})
	}
}
