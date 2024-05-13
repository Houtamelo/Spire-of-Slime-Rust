#[allow(unused_imports)]
use crate::prelude::*;

mod persistent;
mod on_target;
mod on_self;

pub use on_self::SelfApplier;
pub use on_target::{TargetApplier, DebuffApplierKind};
pub use persistent::{PersistentDebuff, PersistentEffect, PoisonAdditive};

pub type IntervalMS = Bound_u64<250, {u64::MAX}>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveDirection {
	ToCenter(NonZeroI8),
	ToEdge(NonZeroI8),
}
