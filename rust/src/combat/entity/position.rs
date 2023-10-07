use crate::combat::entity::Position::Right;
use crate::combat::skills::PositionMatrix;

#[derive(Debug, Clone, Copy, Eq)]
pub enum Position {
	Left  { order: usize, size: usize },
	Right { order: usize, size: usize },
}

impl Position {
	pub fn order(&self) -> &usize {
		return match self {
			Left  { order, .. } => order,
			Right { order, .. } => order,
		};
	}
	
	pub fn order_mut(&mut self) -> &mut usize {
		return match self {
			Left  { order, .. } => order,
			Right { order, .. } => order,
		};
	}
	
	pub fn size(&self) -> &usize {
		return match self {
			Left  { size, .. } => size,
			Right { size, .. } => size,
		};
	}
	
	pub fn size_mut(&mut self) -> &mut usize {
		return match self {
			Left  { size, .. } => size,
			Right { size, .. } => size,
		};
	}
	
	pub fn contains(&self, index: usize) -> bool {
		let begin = *self.order();
		let end = begin + self.size() - 1;
		return index >= begin && index <= end;
	}
	
	pub fn contains_any(&self, positions: &PositionMatrix) -> bool {
		let begin = *self.order();
		let end = begin + self.size() - 1;
		
		for index in 0..positions.indexed_positions.len() {
			let at_index = positions.indexed_positions[index];
			if at_index == true && index >= begin && index <= end {
				return true;
			}
		}
		
		return false;
	}
}

impl Position {
	pub fn same_side(a: &Position, b: &Position) -> bool {
		return match (a, b) {
			(Left  { .. }, Left  { .. }) => true,
			(Right { .. }, Right { .. }) => true,
			_ => false,
		};
	}
	
	pub fn opposite_side(a: &Position, b: &Position) -> bool {
		return match (a, b) {
			(Left  { .. }, Right { .. }) => true,
			(Right { .. }, Left  { .. }) => true,
			_ => false,
		};
	}
	
	pub fn deconstruct(self) -> (usize, usize) {
		return match self {
			Left  { order, size } => (order, size),
			Right { order, size } => (order, size),
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
			(Left  { order: order_a, .. }, Left  { order: order_b, .. }) => order_a.cmp(order_b),
			(Right { order: order_a, .. }, Right { order: order_b, .. }) => order_a.cmp(order_b),
			(Left  { .. }, Right { .. }) => {
				godot_error!("Warning: Trying to compare left and right characters, this should not happen! \nA: {:?} \nB: {:?}", self, other);
				Ordering::Less
			}
			(Right { .. }, Left  { .. }) => {
				godot_error!("Warning: Trying to compare left and right characters, this should not happen! \nA: {:?} \nB: {:?}", self, other);
				Ordering::Greater
			}
		};
	}
}