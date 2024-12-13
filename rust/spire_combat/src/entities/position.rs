use std::cmp::Ordering;

use super::*;

new_bound_unsigned!(Position(i64)[0, 16]);

impl Position {
	pub fn contains(&self, size: Size, index: usize) -> bool {
		let end: usize = (self + *size - 1).cram_into();
		index >= self && index <= end
	}

	pub fn contains_any(&self, size: Size, PositionMatrix(positions): &PositionMatrix) -> bool {
		let end: usize = (self + *size - 1).cram_into();

		for index in 0..positions.len() {
			let at_index = positions[index];
			if at_index == true && index >= self && index <= end {
				return true;
			}
		}

		false
	}

	pub fn is_adjacent(a_pos: Self, a_size: Size, b_pos: Self, b_size: Size) -> Option<Direction> {
		match a_pos.cmp(&b_pos) {
			Ordering::Less => {
				if b_pos == (*a_pos + *a_size - 1) {
					Some(Direction::Back)
				} else {
					None
				}
			}
			Ordering::Greater => {
				if a_pos == (*b_pos + *b_size - 1) {
					Some(Direction::Front)
				} else {
					None
				}
			}
			Ordering::Equal => None,
		}
	}
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
	Front,
	Back,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Team {
	Left,
	Right,
}

impl Team {
	pub fn is_left(&self) -> bool { *self == Team::Left }
	pub fn is_right(&self) -> bool { *self == Team::Right }

	pub fn opposite(&self) -> Team {
		match self {
			Team::Left => Team::Right,
			Team::Right => Team::Left,
		}
	}
}
