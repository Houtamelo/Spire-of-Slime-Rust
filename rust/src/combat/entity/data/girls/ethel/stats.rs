#[allow(unused_imports)]
use crate::*;
use util::prelude::DynamicArray;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

use crate::combat::entity::data::character::CharacterDataTrait;
use crate::combat::entity::data::girls::{GirlName, GirlTrait};
use crate::combat::entity::stat::*;
use crate::combat::skill_types::Skill;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthelData {
	pub(super) skills: DynamicArray<Skill>,
	pub(super) composure: Composure,
	pub(super) orgasm_limit: OrgasmLimit,
	pub(super) size: Size,
    pub(super) dmg: CheckedRange,
    pub(super) spd: Speed,
    pub(super) acc: Accuracy,
    pub(super) crit : CritRate,
    pub(super) dodge: Dodge,
    pub(super) max_stamina: MaxStamina,
    pub(super) toughness  : Toughness,
    pub(super) stun_def   : StunDef,
    pub(super) debuff_res : DebuffRes,
    pub(super) debuff_rate: DebuffRate,
    pub(super) move_res   : MoveRes,
    pub(super) move_rate  : MoveRate,
    pub(super) poison_res : PoisonRes,
    pub(super) poison_rate: PoisonRate,
}

impl GirlTrait for EthelData {
	fn name(&self) -> GirlName { return GirlName::Nema; }
	fn composure(&self) -> Composure { return self.composure; }
	fn orgasm_limit(&self) -> OrgasmLimit { return self.orgasm_limit; }
}

impl CharacterDataTrait for EthelData {
	fn max_stamina(&self, _level: u8, _rng: Option<&mut Xoshiro256PlusPlus>) -> MaxStamina { return self.max_stamina; }
	fn dmg  (&self, _level: u8) -> CheckedRange { return self.dmg.clone(); }
	fn spd  (&self, _level: u8) -> Speed      { return self.spd  ; }
	fn acc  (&self, _level: u8) -> Accuracy   { return self.acc  ; }
	fn crit (&self, _level: u8) -> CritRate { return self.crit ; }
	fn dodge(&self, _level: u8) -> Dodge      { return self.dodge; }
	fn toughness  (&self, _level: u8) -> Toughness  { return self.toughness  ; }
	fn stun_def   (&self, _level: u8) -> StunDef    { return self.stun_def   ; }
	fn debuff_res (&self, _level: u8) -> DebuffRes  { return self.debuff_res ; }
	fn debuff_rate(&self, _level: u8) -> DebuffRate { return self.debuff_rate; }
	fn move_res   (&self, _level: u8) -> MoveRes    { return self.move_res   ; }
	fn move_rate  (&self, _level: u8) -> MoveRate   { return self.move_rate  ; }
	fn poison_res (&self, _level: u8) -> PoisonRes  { return self.poison_res ; }
	fn poison_rate(&self, _level: u8) -> PoisonRate { return self.poison_rate; }
}