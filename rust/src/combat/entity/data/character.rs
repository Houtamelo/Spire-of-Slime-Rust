use std::str::FromStr;
#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;

use enum_dispatch::enum_dispatch;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use rand_xoshiro::Xoshiro256PlusPlus;
use strum::{EnumCount, VariantNames};
use crate::combat::entity::data::girls::GirlData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterData {
	Girl(GirlData),
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

#[enum_dispatch(EntityAnim)]
#[enum_dispatch(AttackedAnim)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CharacterName {
	Girl(GirlName),
	NPC(NPCName),
}

impl FromStr for CharacterName {
	type Err = anyhow::Error;
	
	fn from_str(s: &str) -> Result<Self> {
		if let Ok(girl) = GirlName::from_str(s) {
			return Ok(CharacterName::Girl(girl))
		}
		
		if let Ok(npc) = NPCName::from_str(s) {
			return Ok(CharacterName::NPC(npc))
		}
		
		Err(anyhow!("No character named `{s}` found."))
	}
}

impl FromVariant for CharacterName {
	fn from_variant(variant: &Variant) -> std::result::Result<Self, FromVariantError> {
		let val = variant.try_to::<usize>()?;

		Ok(match val {
			0..GirlName::COUNT => {
				CharacterName::Girl(GirlName::from_repr(val).unwrap())
			}
			GirlName::COUNT..const { GirlName::COUNT + NPCName::COUNT } => {
				CharacterName::NPC(NPCName::from_repr(val - GirlName::COUNT).unwrap())
			}
			_ => return Err(FromVariantError::Unspecified),
		})
	}
}

impl ToVariant for CharacterName {
	fn to_variant(&self) -> Variant {
		match self {
			CharacterName::Girl(girl) => {
				(*girl as usize).to_variant()
			}
			CharacterName::NPC(npc) => {
				(GirlName::COUNT + *npc as usize).to_variant()
			}
		}
	}
}

impl Export for CharacterName {
	type Hint = IntHint<u16>;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		let values =
			GirlName::VARIANTS.iter()
			                  .chain(NPCName::VARIANTS)
			                  .map(|v| v.to_string())
			                  .collect::<Vec<_>>();

		Self::Hint::Enum(EnumHint::new(values)).export_info()
	}
}

