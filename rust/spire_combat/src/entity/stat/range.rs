use rand::distributions::uniform::SampleRange;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::prelude::*;

#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub struct CheckedRange {
	inner: (u8, u8)
}

impl CheckedRange {
	pub const fn new(bound_lower: u8, bound_upper: u8) -> Option<Self> {
		return 
			if bound_upper >= bound_lower { 
				Some(Self { inner: (bound_lower, bound_upper) }) 
			} else { 
				None 
			};
	}

	pub const fn ceil(bound_lower: u8, bound_upper: u8) -> Self {
		return
			if bound_upper >= bound_lower {
				Self { inner: (bound_lower, bound_upper) }
			} else {
				Self { inner: (bound_lower, bound_lower) }
			};
	}
	
	pub const fn floor(bound_lower: u8, bound_upper: u8) -> Self {
		return
			if bound_upper >= bound_lower {
				Self { inner: (bound_lower, bound_upper) }
			} else {
				Self { inner: (bound_upper, bound_upper) }
			};
	}

	pub const fn bound_lower(&self) -> u8 {
		return self.inner.0;
	}

	pub const fn bound_upper(&self) -> u8 {
		return self.inner.1;
	}

	/// Returns true if the bounds were set successfully.
	pub fn set(&mut self, bound_lower: u8, bound_upper: u8) -> Result<(), String> {
		return 
			if bound_upper >= bound_lower { 
				self.inner = (bound_lower, bound_upper);
				Ok(())
			} else {
				Err(format!("{}: bound_upper ({}) is less than bound_lower ({})", full_fn_name(&Self::set), bound_upper, bound_lower))
			};
	}

	pub const fn range(&self) -> std::ops::RangeInclusive<u8> {
		return self.bound_lower()..=self.bound_upper();
	}
}

impl SampleRange<u8> for CheckedRange {
	fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> u8 {
		rng.gen_range(self.bound_lower()..=self.bound_upper())
	}

	fn is_empty(&self) -> bool {
		self.bound_lower() > self.bound_upper()
	}
}