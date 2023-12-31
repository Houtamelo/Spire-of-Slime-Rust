pub mod panel_are_you_sure;

use houta_utils::prelude::*;
use std::ops::Deref;
use rand::prelude::StdRng;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrackedTicks {
	pub remaining_ms: i64,
	pub initial_ms: i64,
}

impl TrackedTicks {
	pub fn from_seconds(seconds: f64) -> TrackedTicks {
		return TrackedTicks {
			remaining_ms: (seconds * 1000.0) as i64,
			initial_ms: (seconds * 1000.0) as i64,
		};
	}

	pub fn from_milliseconds(milliseconds: i64) -> TrackedTicks {
		return TrackedTicks {
			remaining_ms: milliseconds,
			initial_ms: milliseconds,
		};
	}

	pub fn seconds(&self) -> f64 {
		return self.remaining_ms as f64 / 1000.0;
	}
}

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: BoundUSize<0, 100>) -> bool;
}

impl Base100ChanceGenerator for StdRng {
	fn base100_chance(&mut self, chance: BoundUSize<0, 100>) -> bool {
		return  self.gen_ratio(chance.get() as u32, 100);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GUID {
	value: usize,
}

impl GUID {
	pub fn new(value: usize) -> GUID {
		return GUID { value };
	}
}

impl From<usize> for GUID {
	fn from(value: usize) -> Self {
		return GUID { value };
	}
}

impl From<&usize> for GUID {
	fn from(value: &usize) -> Self {
		return GUID { value: *value };
	}
}

impl From<GUID> for usize {
	fn from(value: GUID) -> Self {
		return value.value;
	}
}

impl From<&GUID> for usize {
	fn from(value: &GUID) -> Self {
		return value.value;
	}
}

impl Deref for GUID {
	type Target = usize;

	fn deref(&self) -> &Self::Target {
		return &self.value;
	}
}