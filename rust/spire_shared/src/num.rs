#![allow(clippy::derive_ord_xor_partial_ord)]
#![allow(clippy::derived_hash_with_manual_eq)]

use super::*;

new_bound_signed!(IntPercent(i64)[0, 100]);

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub struct Percentage_0_100 {
	inner_value: f64,
}

bound_f64_impl!(Percentage_0_100, 0.0, 100.0);

#[macro_export]
macro_rules! int {
	($val:expr) => {
		comfy_bounded_ints::prelude::Int::new($val)
	};
}

pub trait DisplaySign {
	fn display_sign(self) -> String;
}

impl<T: CramInto<Int>> DisplaySign for T {
	fn display_sign(self) -> String {
		let int = self.cram_into();
		if int > 0 {
			format!("+{int}")
		} else {
			format!("{int}")
		}
	}
}
