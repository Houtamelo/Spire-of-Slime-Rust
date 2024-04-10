#[allow(unused_imports)]
use crate::*;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use crate::local_map::coordinates::direction::HexagonDirection;
use crate::local_map::coordinates::offset::Offset;
use super::direction;

//pub const ZERO: Axial = Axial { q: 0, r: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Axial {
	pub q: i16,
	pub r: i16,
}

impl Axial {
	pub const ZERO: Axial = Axial { q: 0, r: 0 };
	
	pub fn s(&self) -> i16 { return -self.q - self.r; }
	
	pub fn abs(&self) -> i16 { return self.q.abs() + self.r.abs() + self.s().abs(); }
	
	pub fn are_neighbors(a: &Axial, b: &Axial) -> bool {
		return a.manhattan_distance(b) == 1;
	}
	
	pub fn manhattan_distance(&self, other: &Axial) -> u16 {
		return (((self.q - other.q).abs() + (self.q - other.q + self.r - other.r).abs() + (self.r - other.r).abs()) / 2)
			.squeeze_to_u16();
	}

	pub fn neighbors(&self) -> [(HexagonDirection, Axial); 6] {
		return [
			(HexagonDirection::SouthEast, *self + HexagonDirection::SouthEast.to_axial_vector()),
			(HexagonDirection::East     , *self + HexagonDirection::East     .to_axial_vector()),
			(HexagonDirection::NorthEast, *self + HexagonDirection::NorthEast.to_axial_vector()),
			(HexagonDirection::NorthWest, *self + HexagonDirection::NorthWest.to_axial_vector()),
			(HexagonDirection::West     , *self + HexagonDirection::West     .to_axial_vector()),
			(HexagonDirection::SouthWest, *self + HexagonDirection::SouthWest.to_axial_vector()),
		];
	}
	
	pub fn ring(&self, radius: i16) -> Vec<Axial> {
		let mut results = Vec::with_capacity((radius * 6) as usize);
		
		let mut current = *self + (HexagonDirection::West.to_axial_vector() * radius);
		for direction in direction::ALL {
			let dir_vector = direction.to_axial_vector();
			for _ in 0..radius {
				current += dir_vector;
				results.push(current);
			}
		}
		
		return results;
	}

	const SQRT_3: f32 = 1.7320508075688772935274463415059;
	const SQRT_3_div_3: f32 = Self::SQRT_3 / 3.;
	const SQRT_3_div_2: f32 = Self::SQRT_3 / 2.;

	pub fn to_cartesian(&self, radius: f32) -> (f32, f32) {
		let (q, r) = (self.q as f32, self.r as f32);
	
		let x = radius * ((Self::SQRT_3 * q) + Self::SQRT_3_div_2 * r);
		let y = radius * (1.5 * r);
		return (x, y);
	}
	
	pub fn round_from_cartesian(x: f32, y: f32, radius: f32) -> Axial {
		let float_q = ((Self::SQRT_3_div_3 * x) - (y / 3.)) / radius;
		let float_r = (2. * y) / (3. * radius);
		let float_s = -float_q - float_r;

		let round_q = float_q.round();
		let round_r = float_r.round();
		let round_s = float_s.round();

		let q_diff = (round_q - float_q).abs();
		let r_diff = (round_r - float_r).abs();
		let s_diff = (round_s - float_s).abs();

		if q_diff > r_diff && q_diff > s_diff {
			return Self { q: (- round_r - round_s) as i16, r: round_r as i16 };
		} else if r_diff > s_diff {
			return Self { q: round_q as i16, r: (- round_q - round_s) as i16 };
		} else {
			return Self { q: round_q as i16, r: round_r as i16 };
		}
	}
}

impl From<Offset> for Axial {
	fn from(value: Offset) -> Self {
		let (col, row) = (value.col(), value.row());
		
		let q = col - ((row - (row & 1)) / 2);
		let r = row;
		
		return Axial { q, r };
	}
}

impl Add for Axial {
	type Output = Axial;

	fn add(self, rhs: Axial) -> Axial {
		return Axial { q: self.q + rhs.q, r: self.r + rhs.r };
	}
}

impl AddAssign for Axial {
	fn add_assign(&mut self, rhs: Axial) {
		self.q += rhs.q;
		self.r += rhs.r;
	}
}

impl Sub for Axial {
	type Output = Axial;

	fn sub(self, rhs: Axial) -> Axial {
		return Axial { q: self.q - rhs.q, r: self.r - rhs.r };
	}
}

impl SubAssign for Axial {
	fn sub_assign(&mut self, rhs: Axial) {
		self.q -= rhs.q;
		self.r -= rhs.r;
	}
}

impl Mul<i16> for Axial {
	type Output = Axial;

	fn mul(self, rhs: i16) -> Self::Output {
		return Axial { q: self.q * rhs, r: self.r * rhs };
	}
}

impl MulAssign<i16> for Axial {
	fn mul_assign(&mut self, rhs: i16) {
		self.q *= rhs;
		self.r *= rhs;
	}
}