#![feature(let_chains)]
#![feature(macro_metavar_expr)]

mod affairs;
mod controller;
mod file;
mod states;
mod stats;
mod upgrades;
mod world_location;

use internal_prelude::*;

pub mod prelude {
	pub use crate::{
		affairs::*,
		controller::*,
		file::*,
		states::*,
		stats::*,
		upgrades::*,
		world_location::*,
	};
}

#[allow(unused_imports)]
mod internal_prelude {
	pub use std::collections::HashMap;

	pub use anyhow::{Result, anyhow, bail};
	pub use comfy_bounded_ints::prelude::*;
	pub use declarative_type_state::*;
	pub use getrandom::getrandom;
	pub use godot::{classes::*, global::Error, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use pluck::pluck;
	pub use rand_xoshiro::{
		Xoshiro256PlusPlus,
		rand_core::{RngCore, SeedableRng},
	};
	pub use serde::*;

	pub use crate::prelude::*;
}
