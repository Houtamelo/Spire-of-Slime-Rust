use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Offset {
	pub col: i32,
	pub row: i32,
}

impl Offset {
	pub const ZERO: Offset = Offset { col: 0, row: 0 };

	pub fn col(&self) -> i32 { self.col }
	pub fn row(&self) -> i32 { self.row }
}

impl From<Axial> for Offset {
	fn from(axial: Axial) -> Self {
		let (q, r) = (axial.q, axial.r);

		let col = q + ((r - (r & 1)) / 2);
		let row = r;

		Offset { col, row }
	}
}

impl Add for Offset {
	type Output = Offset;

	fn add(self, rhs: Self) -> Self::Output {
		Offset {
			col: self.col + rhs.col,
			row: self.row + rhs.row,
		}
	}
}

impl AddAssign for Offset {
	fn add_assign(&mut self, rhs: Self) {
		self.col += rhs.col;
		self.row += rhs.row;
	}
}

impl Sub for Offset {
	type Output = Offset;

	fn sub(self, rhs: Self) -> Self::Output {
		Offset {
			col: self.col - rhs.col,
			row: self.row - rhs.row,
		}
	}
}

impl SubAssign for Offset {
	fn sub_assign(&mut self, rhs: Self) {
		self.col -= rhs.col;
		self.row -= rhs.row;
	}
}
