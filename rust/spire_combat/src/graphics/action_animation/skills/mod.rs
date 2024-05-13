use crate::graphics::action_animation::skills::offensive::OffensiveAnim;
#[allow(unused_imports)]
use crate::prelude::*;

pub mod anim_utils;
pub mod offensive;
pub mod girls;
pub mod npcs;

pub enum SkillAnimation {
	Offensive(Box<dyn OffensiveAnim>),
}