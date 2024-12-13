use super::*;

#[repr(transparent)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct SaneRange {
	inner: (Int, Int),
}

impl Deref for SaneRange {
	type Target = (Int, Int);

	fn deref(&self) -> &Self::Target { &self.inner }
}

impl Display for SaneRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}~{}", self.lower(), self.upper())
	}
}

impl SaneRange {
	pub const fn new_const(lower: Int, upper: Int) -> Option<Self> {
		if upper.get() >= lower.get() {
			Some(Self {
				inner: (lower, upper),
			})
		} else {
			None
		}
	}

	pub fn new(lower: impl CramInto<Int>, upper: impl CramInto<Int>) -> Option<Self> {
		let lower = lower.cram_into();
		let upper = upper.cram_into();
		Self::new_const(lower, upper)
	}

	pub fn ceil(lower: impl CramInto<Int>, upper: impl CramInto<Int>) -> Self {
		let lower = lower.cram_into();
		let upper = upper.cram_into();

		if upper >= lower {
			Self {
				inner: (lower, upper),
			}
		} else {
			Self {
				inner: (lower, lower),
			}
		}
	}

	pub fn floor(lower: impl CramInto<Int>, upper: impl CramInto<Int>) -> Self {
		let lower = lower.cram_into();
		let upper = upper.cram_into();

		if upper >= lower {
			Self {
				inner: (lower, upper),
			}
		} else {
			Self {
				inner: (upper, upper),
			}
		}
	}

	pub const fn lower(&self) -> Int { self.inner.0 }
	pub const fn upper(&self) -> Int { self.inner.1 }
	pub const fn deconstruct(&self) -> (Int, Int) { self.inner }

	/// Returns true if the bounds were set successfully.
	pub fn try_set(&mut self, lower: impl CramInto<Int>, upper: impl CramInto<Int>) -> bool {
		let lower = lower.cram_into();
		let upper = upper.cram_into();

		if upper >= lower {
			self.inner = (lower, upper);
			true
		} else {
			false
		}
	}

	pub const fn as_range(&self) -> std::ops::RangeInclusive<Int> { self.lower()..=self.upper() }
}

impl SampleRange<Int> for SaneRange {
	fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Int {
		rng.gen_range(*self.lower()..=*self.upper()).into()
	}

	fn is_empty(&self) -> bool { self.lower() > self.upper() }
}
