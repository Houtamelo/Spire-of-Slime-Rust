pub mod persistent;
pub mod onTarget;
pub mod onSelf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveDirection {
	ToCenter(isize),
	ToEdge(isize),
}
