#![feature(macro_metavar_expr)]
#![allow(clippy::too_many_arguments)]

mod applying;
mod controller;
mod settings;

pub mod prelude {
	pub use crate::{applying::*, controller::*, settings::*};
}

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use audio::{BUS_MAIN, BUS_MUSIC, BUS_SFX, BUS_VOICE};
	pub use godot::{
		classes::{
			display_server::{VSyncMode, WindowMode},
			*,
		},
		prelude::*,
	};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use shared::prelude::PanelAreYouSure;
	pub use strum::IntoEnumIterator;

	pub use crate::prelude::*;
}
