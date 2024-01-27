use std::cmp::Ordering;
use std::collections::HashMap;
use comfy_bounded_ints::prelude::{Bound_u8, SqueezeTo_i64, SqueezeTo_u8, SqueezeTo_usize};
use gdnative::godot_error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::combat::skill_types::PositionMatrix;
use Position::Left as Left;
use Position::Right as Right;
use crate::combat::entity::Entity;

#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize)]
pub enum Position {
	Left  { order: Bound_u8<0, {u8::MAX}>, size: Bound_u8<1, {u8::MAX}> },
	Right { order: Bound_u8<0, {u8::MAX}>, size: Bound_u8<1, {u8::MAX}> },
}

impl Position {
	pub fn order(&self) -> Bound_u8<0, {u8::MAX}> {
		return match self {
			Left  { order, .. } => *order,
			Right { order, .. } => *order,
		};
	}

	pub fn order_mut(&mut self) -> &mut Bound_u8<0, {u8::MAX}> {
		return match self {
			Left  { order, .. } => order,
			Right { order, .. } => order,
		};
	}

	pub fn size(&self) -> Bound_u8<1, {u8::MAX}> {
		return match self {
			Left  { size, .. } => *size,
			Right { size, .. } => *size,
		};
	}

	pub fn contains(&self, index: u8) -> bool {
		let index = index.squeeze_to_i64();
		
		let begin = self.order().squeeze_to_i64();
		let end = begin + self.size().squeeze_to_i64() - 1;
		
		return index >= begin && index <= end;
	}

	pub fn contains_any(&self, positions: &PositionMatrix) -> bool {
		let (begin, end) = {
			let temp_begin = self.order().squeeze_to_i64();
			let temp_end = temp_begin + self.size().squeeze_to_i64() - 1;
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
	
	pub fn insert_entity(entity: Entity, side_1: (Side, &mut HashMap<Uuid, Entity>), side_2: (Side, &mut HashMap<Uuid, Entity>)) {
		let entity_allies= match &entity.position() {
			Left  { .. } if side_1.0 == Side::Left  => { side_1.1 },
			Left  { .. } if side_2.0 == Side::Left  => { side_2.1 },
			Right { .. } if side_1.0 == Side::Right => { side_1.1 },
			Right { .. } if side_2.0 == Side::Right => { side_2.1 },
			_ => {
				godot_error!("Warning: Trying to insert entity in a side but none of the provided sides match the entity's position! \n\
				Entity: {entity:?}\n\
				Side 1: {side_1:?}\n\
				Side 2: {side_2:?}\n
				Entity will be dropped");
				return;
			}
		};
		
		entity_allies.insert(entity.guid(), entity);
	}

	// returns direction from source
	pub fn is_adjacent(source: &Position, other: &Position) -> Option<Direction> {
		if Position::is_opposite_side(source, other) {
			return None;
		}

		let (a_order, a_size) = {
			let (temp_order, temp_size) = source.deconstruct();
			(temp_order.squeeze_to_u8(), temp_size.squeeze_to_u8())
		};
		let (b_order, b_size) = {
			let (temp_order, temp_size) = other.deconstruct();
			(temp_order.squeeze_to_u8(), temp_size.squeeze_to_u8())
		};

		return if a_order + a_size == b_order {
			Some(Direction::Edge)
		} else if b_order + b_size == a_order {
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

impl Position {
	pub fn is_same_side(a: &Position, b: &Position) -> bool {
		return match (a, b) {
			(Left  { .. }, Left  { .. }) => true,
			(Right { .. }, Right { .. }) => true,
			_ => false,
		};
	}

	pub fn is_opposite_side(a: &Position, b: &Position) -> bool {
		return match (a, b) {
			(Left  { .. }, Right { .. }) => true,
			(Right { .. }, Left  { .. }) => true,
			_ => false,
		};
	}

	pub fn deconstruct(self) -> (Bound_u8<0, {u8::MAX}>, Bound_u8<1, {u8::MAX}>) {
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