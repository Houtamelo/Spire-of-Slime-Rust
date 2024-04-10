#[allow(unused_imports)]
use crate::*;

#[derive(Serialize, Deserialize,
	Clone, Copy, Debug,
	ToVariant, FromVariant
)]
pub struct WorldPath { 
	a: WorldLocation, 
	b: WorldLocation
}

impl WorldPath {
	pub fn new(a: WorldLocation, b: WorldLocation) -> Option<Self> {
		return if a != b {
			Some(Self { a, b })
		} else {
			None
		};
	}
	
	pub fn contains(&self, location: WorldLocation) -> bool {
		return self.a == location || self.b == location;
	}
	
	pub fn point_a(&self) -> WorldLocation {
		return self.a;
	}
	
	pub fn point_b(&self) -> WorldLocation {
		return self.b;
	}
}

impl PartialEq for WorldPath {
	fn eq(&self, other: &Self) -> bool {
		return (self.a == other.a && self.b == other.b)
			|| (self.a == other.b && self.b == other.a);
	}
}

impl Eq for WorldPath {}

impl Hash for WorldPath {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		let id = match (self.a, self.b) {
			(WorldLocation::Chapel, WorldLocation::Grove) 
			| (WorldLocation::Grove, WorldLocation::Chapel) 
			=> 0,
			(WorldLocation::Grove, WorldLocation::Forest)
			| (WorldLocation::Forest, WorldLocation::Grove) 
			=> 1,
			(WorldLocation::Forest, WorldLocation::Cave)
			| (WorldLocation::Cave, WorldLocation::Forest) 
			=> 2,
			(WorldLocation::Cave, WorldLocation::Chapel)
			| (WorldLocation::Chapel, WorldLocation::Cave) 
			=> 3,
			(WorldLocation::Chapel, WorldLocation::Forest)
			| (WorldLocation::Forest, WorldLocation::Chapel) 
			=> 4,
			(WorldLocation::Grove, WorldLocation::Cave)
			| (WorldLocation::Cave, WorldLocation::Grove) 
			=> 5,
			(WorldLocation::Chapel, WorldLocation::Chapel)
			| (WorldLocation::Grove, WorldLocation::Grove)
			| (WorldLocation::Forest, WorldLocation::Forest)
			| (WorldLocation::Cave, WorldLocation::Cave) => {
				godot_warn!("WorldPath::hash(): WorldPath cannot start and end at the same spot: {self:?}");
				69
			},
		};
		
		id.hash(state);
	}
}
