#![warn(clippy::missing_const_for_fn)]
#![feature(let_chains)]
#![feature(macro_metavar_expr)]

mod game_manager;
mod save;
mod start_screen;

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use std::fmt::{Debug, Formatter};

	pub use comfy_bounded_ints::prelude::*;
	pub use godot::{
		classes::*,
		global::{Error, Key},
		prelude::*,
	};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use rand_xoshiro::Xoshiro256PlusPlus;
	pub use serde::{Deserialize, Serialize};
	pub use shared::prelude::*;
	pub use spire_tween::prelude::*;
	pub use uuid::Uuid;

	pub use crate::save::*;
}

struct SpireLibrary;

#[godot::prelude::gdextension]
unsafe impl godot::prelude::ExtensionLibrary for SpireLibrary {}
