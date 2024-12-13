use super::*;

mod anim_utils;
mod girls;
mod npcs;
mod offensive;

pub use anim_utils::*;
pub use girls::*;
pub use npcs::*;
pub use offensive::*;

pub enum SkillAnimation {
	Offensive(Box<dyn OffensiveAnim>),
}
