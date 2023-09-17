
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RemainingTicks {
	pub remaining_ms: i64,
	pub initial_ms: i64,
}

impl RemainingTicks {
	pub fn from_seconds(seconds: f64) -> RemainingTicks {
		return RemainingTicks {
			remaining_ms: (seconds * 1000.0) as i64,
			initial_ms: (seconds * 1000.0) as i64,
		};
	}
	
	pub fn from_milliseconds(milliseconds: i64) -> RemainingTicks {
		return RemainingTicks {
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
