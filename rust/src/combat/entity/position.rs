use std::cmp::Ordering;
use std::collections::HashMap;
use gdnative::godot_error;
use crate::combat::skills::PositionMatrix;
use Position::Left as Left;
use Position::Right as Right;
use crate::combat::entity::Entity;
use crate::util::GUID;

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
	
	pub fn insert_entity(entity: Entity, side_1: (Side, &mut HashMap<GUID, Entity>), side_2: (Side, &mut HashMap<GUID, Entity>)) {
		let entity_allies= match &entity.position() {
			Left  { .. } if side_1.0 == Side::Left  => { side_1.1 },
			Left  { .. } if side_2.0 == Side::Left  => { side_2.1 },
			Right { .. } if side_1.0 == Side::Right => { side_1.1 },
			Right { .. } if side_2.0 == Side::Right => { side_2.1 },
			_ => {
				godot_error!("Warning: Trying to insert entity in a side but none of the provided sides match the entity's position! \nEntity: {:?} \nSide 1: {:?} \nSide 2: {:?}\n\n Entity will be dropped"
					, entity, side_1, side_2);
				return;
			}
		};
		
		entity_allies.insert(entity.guid(), entity);
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
			(Left  { order: order_a, size: size_a }, Left  { order: order_b, size: size_b }) => order_a == order_b && size_a == size_b,
			(Right { order: order_a, size: size_a }, Right { order: order_b, size: size_b }) => order_a == order_b && size_a == size_b,
			(Left  { .. }, Right { .. }) => false,
			(Right { .. }, Left  { .. }) => false,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
	Left,
	Right
}