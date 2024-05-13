use comfy_bounded_ints::prelude::Bound_u8;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::num::PercentageU8;

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: PercentageU8) -> bool;
}

impl Base100ChanceGenerator for Xoshiro256PlusPlus {
	fn base100_chance(&mut self, chance: Bound_u8<0, 100>) -> bool {
		return self.gen_ratio(chance.get() as u32, 100);
	}
}