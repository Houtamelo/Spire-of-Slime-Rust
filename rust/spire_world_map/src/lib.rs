mod controller;
mod path;

pub mod prelude {
	pub use crate::{controller::*, path::*};
}

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use godot::{classes::*, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use serde::{Deserialize, Serialize};
	pub use serialization::prelude::*;
	pub use shared::prelude::*;

	pub use crate::prelude::*;
}
