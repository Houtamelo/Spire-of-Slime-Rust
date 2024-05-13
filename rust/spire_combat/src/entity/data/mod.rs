#[allow(unused_imports)]
use crate::prelude::*;

pub mod npc;
pub mod girls;
pub mod skill_variant;
pub mod display_name;
pub mod variant;
pub mod asset_folder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityDataVariant {
	Character(CharacterDataVariant),
}

#[enum_delegate::delegate]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterDataVariant {
	Girl(GirlDataVariant),
	NPC(NPCVariant),
}

#[enum_delegate::delegate(for(
	CharacterDataVariant;
	GirlDataVariant;
))]
pub trait CharacterData {
	fn variant(&self) -> CharacterVariant;

	fn max_stamina(&self, level: u8) -> MaxStamina;
	fn dmg(&self, level: u8) -> CheckedRange;
	fn spd(&self, level: u8) -> Speed;
	fn acc(&self, level: u8) -> Accuracy;
	fn crit(&self, level: u8) -> CritRate;
	fn dodge(&self, level: u8) -> Dodge;
	fn toughness(&self, level: u8) -> Toughness;
	fn stun_def(&self, level: u8) -> StunDef;
	fn debuff_res(&self, level: u8) -> DebuffRes;
	fn debuff_rate(&self, level: u8) -> DebuffRate;
	fn move_res(&self, level: u8) -> MoveRes;
	fn move_rate(&self, level: u8) -> MoveRate;
	fn poison_res(&self, level: u8) -> PoisonRes;
	fn poison_rate(&self, level: u8) -> PoisonRate;

	fn skills<'a>(&'a self) -> &Cow<'a, [Skill]>;
}

pub trait NPCData {
	fn stamina_amplitude(&self, level: u8) -> u16;
	
	fn generate_random_stamina(
		&self,
		level: u8,
		rng: *mut Xoshiro256PlusPlus,
	) -> MaxStamina 
	  where Self: CharacterData {
		let base = self.max_stamina(level);
		let amplitude = self.stamina_amplitude(level);
		
		unsafe {
			MaxStamina::new(base.get() + (*rng).gen_range(0..=amplitude))
		}
	}
}

