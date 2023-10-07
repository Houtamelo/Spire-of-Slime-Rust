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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Range {
	pub min: isize,
	pub max: isize,
}

impl Range {
	pub fn new(min: isize, max: isize) -> Range {
		return if min > max {
			Range { min, max: min }
		} else {
			Range { min, max }
		}
	}
}

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: isize) -> bool;
}

impl Base100ChanceGenerator for StdRng {
	fn base100_chance(&mut self, chance: isize) -> bool {
		if chance <= 100 {
			return self.gen_ratio(chance as u32, 100);
		} else {
			return true;
		}
	}
}