#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;

use enum_dispatch::enum_dispatch;
use rand_xoshiro::Xoshiro256PlusPlus;
use crate::combat::entity::data::girls::GirlData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterData {
	Girl(GirlData),
	NPC(NPCName),
}

#[enum_dispatch(AttackedAnim)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CharacterName {
	Girl(GirlName),
	NPC(NPCName),
}

pub trait CharacterDataTrait {
	fn max_stamina(&self, level: u8, rng: Option<&mut Xoshiro256PlusPlus>) -> MaxStamina;
	fn dmg  (&self, level: u8) -> CheckedRange;
	fn spd  (&self, level: u8) -> Speed;
	fn acc  (&self, level: u8) -> Accuracy;
	fn crit (&self, level: u8) -> CritRate;
	fn dodge(&self, level: u8) -> Dodge;
	fn toughness  (&self, level: u8) -> Toughness;
	fn stun_def   (&self, level: u8) -> StunDef;
	fn debuff_res (&self, level: u8) -> DebuffRes;
	fn debuff_rate(&self, level: u8) -> DebuffRate;
	fn move_res   (&self, level: u8) -> MoveRes;
	fn move_rate  (&self, level: u8) -> MoveRate;
	fn poison_res (&self, level: u8) -> PoisonRes;
	fn poison_rate(&self, level: u8) -> PoisonRate;
}

pub trait SkillUser {
	fn skills(&self) -> &[Skill];
}
