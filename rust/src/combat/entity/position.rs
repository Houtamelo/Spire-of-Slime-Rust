#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
	pub order: SaturatedU8,
	pub size: Bound_u8<1, { u8::MAX }>,
	pub side: Side,
}

impl Position {
	pub fn contains(&self, index: u8) -> bool {
		let index = index.squeeze_to_i64();
		
		let begin = self.order.squeeze_to_i64();
		let end = begin + self.size.squeeze_to_i64() - 1;
		
		return index >= begin && index <= end;
	}

	pub fn contains_any(&self, positions: &PositionMatrix) -> bool {
		let (begin, end) = {
			let temp_begin = self.order.squeeze_to_i64();
			let temp_end = temp_begin + self.size.squeeze_to_i64() - 1;
			(temp_begin.squeeze_to_usize(), temp_end.squeeze_to_usize())
		};

		for index in 0..positions.positions.len() {
			let at_index = positions.positions[index];
			if at_index == true
			&& index >= begin 
			&& index <= end {
				return true;
			}
		}

		return false;
	}
	
	// returns direction from source
	pub fn is_adjacent(x: &Position, y: &Position) -> Option<Direction> {
		if x.side != y.side {
			return None;
		}

		return if *x.order + *x.size == *y.order {
			Some(Direction::Edge)
		} else if *y.order + *y.size == *x.order {
			Some(Direction::Center)
		} else {
			None
		};
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
	Center,
	Edge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
	Left,
	Right
}

impl Side {
	pub fn is_left(&self) -> bool {
		return *self == Side::Left;
	}
	
	pub fn is_right(&self) -> bool {
		return *self == Side::Right;
	}
}