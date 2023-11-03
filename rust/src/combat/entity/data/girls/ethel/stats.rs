use std::ops::RangeInclusive;
use rand::prelude::StdRng;
use proc_macros::insert_combat_character_fields;
use crate::{BoundISize, BoundU32};
use crate::combat::entity::data::character::{CharacterDataTrait, SkillUserTrait};
use crate::combat::entity::data::girls::{GirlName, GirlTrait};
use crate::combat::skill_types::Skill;

insert_combat_character_fields!(
#[derive(Debug, Clone)]
pub struct EthelData {
	pub(super) skills: Vec<&'static Skill>,
	pub(super) composure   : crate::BoundISize< -100, 300>,
	pub(super) orgasm_limit: BoundU32<1, 8>,
});

impl GirlTrait for EthelData {
	fn name        (&self) -> GirlName              { return GirlName::Nema   ; }
	fn composure   (&self) -> BoundISize<-100, 300> { return self.composure   ; }
	fn orgasm_limit(&self) -> BoundU32<1, 8>        { return self.orgasm_limit; }
}

impl CharacterDataTrait for EthelData {
	fn stamina_max(&self, _level: usize, _rng: Option<&mut StdRng>) -> BoundU32<1, 500> { return self.stamina_max; }
	fn dmg        (&self, _level: usize) -> RangeInclusive<usize> { return self.dmg.clone(); }
	fn spd        (&self, _level: usize) -> BoundU32  <  20, 300> { return self.spd        ; }
	fn acc        (&self, _level: usize) -> BoundISize<-300, 300> { return self.acc        ; }
	fn crit       (&self, _level: usize) -> BoundISize<-300, 300> { return self.crit       ; }
	fn dodge      (&self, _level: usize) -> BoundISize<-300, 300> { return self.dodge      ; }
	fn toughness  (&self, _level: usize) -> BoundISize<-100, 100> { return self.toughness  ; }
	fn stun_def   (&self, _level: usize) -> BoundISize<-100, 300> { return self.stun_def   ; }
	fn debuff_res (&self, _level: usize) -> BoundISize<-300, 300> { return self.debuff_res ; }
	fn debuff_rate(&self, _level: usize) -> BoundISize<-300, 300> { return self.debuff_rate; }
	fn move_res   (&self, _level: usize) -> BoundISize<-300, 300> { return self.move_res   ; }
	fn move_rate  (&self, _level: usize) -> BoundISize<-300, 300> { return self.move_rate  ; }
	fn poison_res (&self, _level: usize) -> BoundISize<-300, 300> { return self.poison_res ; }
	fn poison_rate(&self, _level: usize) -> BoundISize<-300, 300> { return self.poison_rate; }
}

impl SkillUserTrait for EthelData { fn skills(&self) -> &Vec<&Skill> { return &self.skills; } }