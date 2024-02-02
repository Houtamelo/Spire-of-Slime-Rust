use houta_utils::prelude::IterGenerator;
use rand::Rng;

pub trait WeightedRandom<'a, T> 
	where Self: IterGenerator<'a, (T, &'a f32)> 
{
	fn get_weighted_random(&'a self, rng: &mut impl Rng) -> Option<T> {
		let weight_sum = self
			.iterate()
			.fold(0., |acc, (_, weight)| acc + weight);
		
		let random = rng.gen_range(0.0..=weight_sum);
		
		let mut current_sum = weight_sum;
		for (value, weight) in self.iterate() {
			current_sum -= weight;
			if current_sum <= random {
				return Some(value);
			}
		}
		
		return None;
	}
}

impl<'a, T, TImpl> WeightedRandom<'a, T> for TImpl
	where TImpl: IterGenerator<'a, (T, &'a f32)> { }