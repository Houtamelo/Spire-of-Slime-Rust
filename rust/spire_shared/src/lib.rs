#![feature(try_trait_v2)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(step_trait)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]

mod enumerator_macro;
mod impl_trait_cast;
mod input;
mod into_boxed_impl;
mod num;
mod panel_are_you_sure;
mod rand_utils;

pub const CONFIG_PATH: &str = "user://config.cfg";

impl FromResidual for () {
	fn from_residual(_residual: <Self as Try>::Residual) -> Self {}
}

impl Try for () {
	type Output = std::convert::Infallible;
	type Residual = Option<std::convert::Infallible>;

	fn from_output(_output: Self::Output) -> Self {}

	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> { ControlFlow::Break(None) }
}

#[allow(unused_imports)]
pub mod prelude {
	pub use crate::{
		CONFIG_PATH,
		enumerator,
		impl_trait_cast::*,
		input::*,
		int,
		into_boxed_impl::*,
		num::*,
		panel_are_you_sure::*,
		rand_utils::*,
	};
}

use std::ops::{ControlFlow, FromResidual, Try};

use internal_prelude::*;

#[allow(unused_imports)]
mod internal_prelude {
	pub use std::{
		backtrace::Backtrace,
		fmt::{Debug, Formatter},
		hash::{Hash, Hasher},
		ops::{Deref, DerefMut},
	};

	pub use comfy_bounded_ints::prelude::*;
	pub use godot::{classes::*, meta::PropertyHintInfo, obj::Bounds, prelude::*};
	pub use houtamelo_utils::prelude::*;
	pub use houtamelo_utils_gdext::prelude::*;
	pub use rand::Rng;
	pub use rand_xoshiro::Xoshiro256PlusPlus;

	pub use crate::prelude::*;
}
