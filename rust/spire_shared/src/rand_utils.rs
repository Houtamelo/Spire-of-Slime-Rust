use super::*;

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: impl CramInto<u32>) -> bool;
}

impl Base100ChanceGenerator for Xoshiro256PlusPlus {
	fn base100_chance(&mut self, chance: impl CramInto<u32>) -> bool {
		let chance = chance.cram_into();
		if chance > 100 {
			true
		} else {
			self.gen_ratio(chance, 100)
		}
	}
}
