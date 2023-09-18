use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};
use fyrox::rand::Rng;
use fyrox::rand::rngs::StdRng;
use crate::combat::{Character, Girl, CombatState, Side};
use crate::combat::ModifiableStat::{DEBUFF_RATE, DEBUFF_RES, MOVE_RATE, MOVE_RES, STUN_DEF};
use crate::util::RemainingTicks;

pub mod persistent;
pub mod onTarget;
pub mod onSelf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveDirection {
	ToCenter(isize),
	ToEdge(isize),
}
