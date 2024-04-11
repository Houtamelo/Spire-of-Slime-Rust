#[allow(unused_imports)]
use crate::*;
use std::num::NonZeroI8;

pub mod persistent;
pub mod onTarget;
pub mod onSelf;

pub type IntervalMS = Bound_u64<250, {u64::MAX}>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveDirection {
	ToCenter(NonZeroI8),
	ToEdge(NonZeroI8),
}
