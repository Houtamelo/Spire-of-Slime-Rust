use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackedTicks {
	pub remaining_ms: Int,
	pub initial_ms:   Int,
}

impl TrackedTicks {
	pub fn from_ms(milliseconds: impl CramInto<Int>) -> TrackedTicks {
		let milliseconds = milliseconds.cram_into();
		TrackedTicks {
			remaining_ms: milliseconds,
			initial_ms:   milliseconds,
		}
	}
}

pub trait PercentageTools: Sized {
	fn with_percent(self, percent: impl CramInto<Int>) -> Self;
	fn set_percent(&mut self, percent: impl CramInto<Int>);
}

impl<T: CramInto<Int> + Copy> PercentageTools for T
where Int: CramInto<T>
{
	fn with_percent(self, percent: impl CramInto<Int>) -> Self {
		let mut this = self.cram_into();
		this *= percent.cram_into();
		this /= 100;
		this.cram_into()
	}

	fn set_percent(&mut self, percent: impl CramInto<Int>) { *self = self.with_percent(percent); }
}

new_generic_bound_signed!(BndInt < MIN, MAX > (i64));
