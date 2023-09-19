#[derive(Debug, Clone, Copy, Eq)]
pub enum Position {
	Left  { order: usize, size: usize },
	Right { order: usize, size: usize },
}

impl Position {
	pub fn order(&self) -> &usize {
		return match self {
			Position::Left  { order, .. } => order,
			Position::Right { order, .. } => order,
		};
	}
	
	pub fn order_mut(&mut self) -> &mut usize {
		return match self {
			Position::Left  { order, .. } => order,
			Position::Right { order, .. } => order,
		};
	}
	
	pub fn size(&self) -> &usize {
		return match self {
			Position::Left  { size, .. } => size,
			Position::Right { size, .. } => size,
		};
	}
	
	pub fn size_mut(&mut self) -> &mut usize {
		return match self {
			Position::Left  { size, .. } => size,
			Position::Right { size, .. } => size,
		};
	}
}

impl Position {
	pub fn same_side(a: &Position, b: &Position) -> bool {
		return match (a, b) {
			(Position::Left  { .. }, Position::Left  { .. }) => true,
			(Position::Right { .. }, Position::Right { .. }) => true,
			_ => false,
		};
	}
}

impl PartialEq<Self> for Position {
	fn eq(&self, other: &Self) -> bool {
		return match (self, other) {
			(Position::Left  { order: order_a, size: size_a }, Position::Left  { order: order_b, size: size_b }) => order_a == order_b && size_a == size_b,
			(Position::Right { order: order_a, size: size_a }, Position::Right { order: order_b, size: size_b }) => order_a == order_b && size_a == size_b,
			(Position::Left  { .. }, Position::Right { .. }) => false,
			(Position::Right { .. }, Position::Left  { .. }) => false,
		};
	}
}

impl PartialOrd<Self> for Position {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		return Some(self.cmp(other));
	}
}

impl Ord for Position {
	fn cmp(&self, other: &Self) -> Ordering {
		return match (self, other) {
			(Position::Left  { order: order_a, .. }, Position::Left  { order: order_b, .. }) => order_a.cmp(order_b),
			(Position::Right { order: order_a, .. }, Position::Right { order: order_b, .. }) => order_a.cmp(order_b),
			(Position::Left  { .. }, Position::Right { .. }) => {
				eprintln!("Warning: Trying to compare left and right characters, this should not happen! \nA: {:?} \nB: {:?}", self, other);
				Ordering::Less
			}
			(Position::Right { .. }, Position::Left  { .. }) => {
				eprintln!("Warning: Trying to compare left and right characters, this should not happen! \nA: {:?} \nB: {:?}", self, other);
				Ordering::Greater
			}
		};
	}
}