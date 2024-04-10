#[allow(unused_imports)]
use crate::*;
use std::num::NonZeroI8;
use comfy_bounded_ints::prelude::Bound_u64;
use serde::{Deserialize, Serialize};

pub mod persistent;
pub mod onTarget;
pub mod onSelf;

pub type IntervalMS = Bound_u64<250, {u64::MAX}>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveDirection {
	ToCenter(NonZeroI8),
	ToEdge(NonZeroI8),
}
