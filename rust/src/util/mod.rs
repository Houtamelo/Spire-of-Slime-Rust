pub mod panel_are_you_sure;
mod int_conversions;

use comfy_bounded_ints::prelude::{Bound_i64, Bound_u64};
use comfy_bounded_ints::types::Bound_u8;
use gdnative::api::{GlobalConstants, InputEvent, InputEventKey, InputEventMouseButton};
use gdnative::object::{Ref, TRef};
use houta_utils::prelude::*;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

pub type SaturatedU64 = Bound_u64<0, {u64::MAX}>;
pub type SaturatedI64 = Bound_i64<{i64::MIN}, {i64::MAX}>;
pub use int_conversions::*;
pub type PercentageU8 = Bound_u8<0, 100>;

pub fn any_cancel_input(event: &TRef<InputEvent>) -> bool {
	return (event.is_action("ui_cancel", false))
		|| (event.cast::<InputEventMouseButton>().is_some_and(|mouse_event| mouse_event.button_index() == 2))
		|| (event.cast::<InputEventKey>().is_some_and(|key_event| key_event.scancode() == 16777217)); // Escape key
}

pub fn is_confirm_input(event: Ref<InputEvent>) -> bool {
	let safe_event = unsafe { event.assume_safe() };
	
	if safe_event.cast::<InputEventMouseButton>().is_some_and(|mouse_event|
		mouse_event.is_pressed() && mouse_event.button_index() == GlobalConstants::BUTTON_LEFT)
	{
		return true;
	}
	
	if safe_event.is_action_pressed("ui_accept", false, true) {
		return true;
	}
	
	return false;
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