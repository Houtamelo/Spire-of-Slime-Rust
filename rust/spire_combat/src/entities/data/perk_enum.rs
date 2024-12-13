use super::*;

delegated_enum! {
	ENUM_OUT: {
		#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
		pub enum Perk {
			BellPlant(BellPlantPerk),
		}
	}

	DELEGATES: {
		impl trait IPerk {
			[fn tick(
				&mut self,
				actor: &mut Ptr<Actor>,
				ctx: &mut ActorContext,
				delta_ms: Int,
			) -> PerkTickResult]
		}
	}
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
		pub enum GirlPerk {
			Ethel(EthelPerk),
			Nema(NemaPerk),
		}
	}

	DELEGATES: {
		impl trait IGirlPerk {
			[fn tick(
				&mut self,
				actor: &mut Ptr<Actor>,
				girl: &mut Ptr<Girl>,
				ctx: &mut ActorContext,
				delta_ms: Int,
			) -> PerkTickResult]
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PerkID(pub u64);

impl Perk {
	pub fn id(&self) -> PerkID {
		use std::hash::Hasher;

		let mut hasher = zwohash::ZwoHasher::default();
		self.hash(&mut hasher);
		PerkID(hasher.finish())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct GirlPerkID(pub u64);

impl GirlPerk {
	pub fn id(&self) -> GirlPerkID {
		use std::hash::Hasher;

		let mut hasher = zwohash::ZwoHasher::default();
		self.hash(&mut hasher);
		GirlPerkID(hasher.finish())
	}
}
