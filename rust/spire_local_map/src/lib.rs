mod coordinates;
mod generation;
mod state;
mod tile;

pub const SCENE_PATH: &str = "res://Core/Local Map/scene_local-map.tscn";

#[allow(unused_imports)]
pub mod prelude {
	pub use crate::{coordinates::*, generation::*, state::*, tile::*};
}

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use std::ops::{Add, AddAssign, Mul, MulAssign, RangeInclusive, Sub, SubAssign};

	pub use bracket_noise::prelude::FastNoise;
	pub use comfy_bounded_ints::prelude::*;
	pub use godot::{classes::*, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng};
	pub use serde::{Deserialize, Serialize};
	pub use shared::prelude::*;

	pub use crate::prelude::*;
}
