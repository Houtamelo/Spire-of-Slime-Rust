#![feature(let_chains)]
#![feature(macro_metavar_expr)]

mod controller;
mod easters_iron_gauntlet;
mod easters_save_name;
mod load_button;

#[allow(unused_imports)]
pub mod prelude {
	pub use crate::{controller::*, load_button::*};
}

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use godot::{classes::*, global::Error, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use shared::prelude::*;
	pub use spire_tween::prelude::*;

	pub use crate::prelude::*;
	pub(crate) use crate::{easters_iron_gauntlet, easters_save_name};
}
