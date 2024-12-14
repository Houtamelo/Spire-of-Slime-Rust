use super::*;

mod asset_folder;
mod display_name;
mod girls;
mod npc;
mod perk_enum;
mod skill_variant;
mod variant;

pub use asset_folder::*;
pub use display_name::*;
pub use girls::*;
pub use npc::*;
pub use perk_enum::*;
pub use skill_variant::*;
pub use variant::*;

#[derive(Clone, Serialize, Deserialize)]
pub enum EntityDataVariant {
	Character(CharacterDataEnum),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CharacterDataEnum {
	Girl(GirlDataEnum),
	NPC(NpcName),
}

pub trait CharacterData {
	fn variant(&self) -> CharacterVariant;

	fn max_stamina(&self, level: i64) -> MaxStamina;
	fn dmg(&self, level: i64) -> SaneRange;
	fn spd(&self, level: i64) -> Speed;
	fn acc(&self, level: i64) -> Accuracy;
	fn crit(&self, level: i64) -> CritRate;
	fn dodge(&self, level: i64) -> Dodge;
	fn toughness(&self, level: i64) -> Toughness;
	fn stun_def(&self, level: i64) -> StunDef;
	fn debuff_res(&self, level: i64) -> DebuffRes;
	fn debuff_rate(&self, level: i64) -> DebuffRate;
	fn move_res(&self, level: i64) -> MoveRes;
	fn move_rate(&self, level: i64) -> MoveRate;
	fn poison_res(&self, level: i64) -> PoisonRes;
	fn poison_rate(&self, level: i64) -> PoisonRate;

	fn skills(&self) -> &[Skill];
}

pub trait NPCData {
	fn stamina_amplitude(&self, level: i64) -> Int;

	fn generate_random_stamina(&self, level: i64, rng: &mut impl Rng) -> MaxStamina
	where Self: CharacterData {
		let mut temp = self.max_stamina(level);
		let amplitude = self.stamina_amplitude(level);

		if amplitude > 0 {
			temp += rng.gen_range(0..=*amplitude);
		}

		temp
	}
}
