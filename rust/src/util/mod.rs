pub mod panel_are_you_sure;
pub mod weighted_rand;
mod int_conversions;

use comfy_bounded_ints::prelude::{Bound_i64, Bound_u64};
use comfy_bounded_ints::types::Bound_u8;
use gdnative::api::{InputEvent, InputEventKey, InputEventMouseButton};
use gdnative::object::TRef;
use houta_utils::prelude::*;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

pub type SaturatedU64 = Bound_u64<0, {u64::MAX}>;
pub type SaturatedI64 = Bound_i64<{i64::MIN}, {i64::MAX}>;
pub use int_conversions::*;
pub type PercentageU8 = Bound_u8<0, 100>;

pub fn fn_name<T: ?Sized>(_val: &T) -> &'static str {
	let name = std::any::type_name::<T>();
	
	return match &name[..name.len()].rfind(':') {
		Some(pos) => &name[pos + 1..name.len()],
		None => &name,
	};
}

pub const fn full_fn_name<T: ?Sized>(_val: &T) -> &'static str {
	return std::any::type_name::<T>();
}

pub fn any_cancel_input(event: &TRef<InputEvent>) -> bool {
	return (event.is_action("ui_cancel", false))
		|| (event.cast::<InputEventMouseButton>().is_some_and(|mouse_event| mouse_event.button_index() == 2))
		|| (event.cast::<InputEventKey>().is_some_and(|key_event| key_event.scancode() == 16777217)); // Escape key
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackedTicks {
	pub remaining_ms: SaturatedU64,
	pub initial_ms: SaturatedU64,
}

impl TrackedTicks {
	pub fn from_milliseconds(milliseconds: SaturatedU64) -> TrackedTicks {
		return TrackedTicks {
			remaining_ms: milliseconds,
			initial_ms: milliseconds,
		};
	}
}

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: PercentageU8) -> bool;
}

impl Base100ChanceGenerator for Xoshiro256PlusPlus {
	fn base100_chance(&mut self, chance: Bound_u8<0, 100>) -> bool {
		return self.gen_ratio(chance.get() as u32, 100);
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Percentage_0_100 {
	inner_value: f64,
}

bound_f64_impl!(Percentage_0_100, 0.0, 100.0);