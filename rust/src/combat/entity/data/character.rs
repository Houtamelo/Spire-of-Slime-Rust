use std::ops::RangeInclusive;
use rand::prelude::StdRng;
use crate::{BoundISize, BoundU32};
use crate::combat::entity::data::girls::GirlData;
use crate::combat::entity::data::npc::NPCData;
use crate::combat::skill_types::Skill;

#[derive(Debug, Clone)]
pub enum CharacterData {
	Girl(GirlData),
	NPC(NPCData),
}

pub trait CharacterDataTrait {
	fn stamina_max(&self, level: usize, rng: Option<&mut StdRng>) -> BoundU32  <1, 500>;
	fn dmg        (&self, level: usize) -> RangeInclusive<usize>;
	fn spd        (&self, level: usize) -> BoundU32  <  20, 300>;
	fn acc        (&self, level: usize) -> BoundISize<-300, 300>;
	fn crit       (&self, level: usize) -> BoundISize<-300, 300>;
	fn dodge      (&self, level: usize) -> BoundISize<-300, 300>;
	fn toughness  (&self, level: usize) -> BoundISize<-100, 100>;
	fn stun_def   (&self, level: usize) -> BoundISize<-100, 300>;
	fn debuff_res (&self, level: usize) -> BoundISize<-300, 300>;
	fn debuff_rate(&self, level: usize) -> BoundISize<-300, 300>;
	fn move_res   (&self, level: usize) -> BoundISize<-300, 300>;
	fn move_rate  (&self, level: usize) -> BoundISize<-300, 300>;
	fn poison_res (&self, level: usize) -> BoundISize<-300, 300>;
	fn poison_rate(&self, level: usize) -> BoundISize<-300, 300>;
}

pub trait SkillUserTrait {
	fn skills(&self) -> &Vec<&Skill>;
}
