pub mod panel_are_you_sure;

use std::ops::Deref;
use rand::prelude::StdRng;
use rand::Rng;
use crate::BoundU32;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct I_Range {
	pub min: isize,
	pub max: isize,
}

impl I_Range {
	pub fn new(min: isize, max: isize) -> I_Range {
		return if min > max {
			I_Range { min, max: min }
		} else {
			I_Range { min, max }
		}
	}
}

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: BoundU32<0, 100>) -> bool;
}

impl Base100ChanceGenerator for StdRng {
	fn base100_chance(&mut self, chance: BoundU32<0, 100>) -> bool {
		return  self.gen_ratio(chance.get(), 100);
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