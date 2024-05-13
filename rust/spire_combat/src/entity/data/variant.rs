#[allow(unused_imports)]
use crate::prelude::*;

use std::str::FromStr;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use strum::{EnumCount, VariantNames};

#[enum_delegate::delegate]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CharacterVariant {
	Girl(GirlVariant),
	NPC(NPCVariant),
}

impl FromStr for CharacterVariant {
	type Err = anyhow::Error;
	
	fn from_str(s: &str) -> Result<Self> {
		if let Ok(girl) = GirlVariant::from_str(s) {
			return Ok(CharacterVariant::Girl(girl))
		}
		
		if let Ok(npc) = NPCVariant::from_str(s) {
			return Ok(CharacterVariant::NPC(npc))
		}
		
		Err(anyhow!("No character named `{s}` found."))
	}
}

impl FromVariant for CharacterVariant {
	fn from_variant(variant: &Variant) -> std::result::Result<Self, FromVariantError> {
		let val = variant.try_to::<usize>()?;

		Ok(match val {
			0..GirlVariant::COUNT => {
				CharacterVariant::Girl(GirlVariant::from_repr(val).unwrap())
			}
			GirlVariant::COUNT..const { GirlVariant::COUNT + NPCVariant::COUNT } => {
				CharacterVariant::NPC(NPCVariant::from_repr(val - GirlVariant::COUNT).unwrap())
			}
			_ => return Err(FromVariantError::Unspecified),
		})
	}
}

impl ToVariant for CharacterVariant {
	fn to_variant(&self) -> Variant {
		match self {
			CharacterVariant::Girl(girl) => {
				(*girl as usize).to_variant()
			}
			CharacterVariant::NPC(npc) => {
				(GirlVariant::COUNT + *npc as usize).to_variant()
			}
		}
	}
}

impl Export for CharacterVariant {
	type Hint = IntHint<u16>;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		let values =
			GirlVariant::VARIANTS.iter()
			                     .chain(NPCVariant::VARIANTS)
			                     .map(|v| v.to_string())
			                     .collect::<Vec<_>>();

		Self::Hint::Enum(EnumHint::new(values)).export_info()
	}
}
