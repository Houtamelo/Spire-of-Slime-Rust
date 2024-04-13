#[allow(unused_imports)]
use crate::*;
use std::str::FromStr;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use strum::{EnumCount, VariantNames};

pub use crate::combat::entity::data::girls::ethel::skills::EthelSkill;
pub use crate::combat::entity::data::girls::nema::skills::NemaSkill;
pub use crate::combat::entity::data::npc::bellplant::BellPlantSkill;
pub use crate::combat::entity::data::npc::crabdra::CrabdraSkill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillName {
	FromNema(NemaSkill),
	FromEthel(EthelSkill),
	FromCrabdra(CrabdraSkill),
	FromBellPlant(BellPlantSkill),
}

impl FromStr for SkillName {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		if let Ok(ok) = NemaSkill::from_str(s) {
			return Ok(Self::FromNema(ok));
		}
		
		if let Ok(ok) = EthelSkill::from_str(s) {
			return Ok(Self::FromEthel(ok));
		}
		
		if let Ok(ok) = CrabdraSkill::from_str(s) {
			return Ok(Self::FromCrabdra(ok));
		}
		
		if let Ok(ok) = BellPlantSkill::from_str(s) {
			return Ok(Self::FromBellPlant(ok));
		}
		
		Err(anyhow!("Invalid SkillName: {s}"))
	}
}

impl FromVariant for SkillName {
	fn from_variant(variant: &Variant) -> std::result::Result<Self, FromVariantError> {
		let val = variant.try_to::<usize>()?;
		
		Ok(match val {
			0..NemaSkill::COUNT => {
				SkillName::FromNema(NemaSkill::from_repr(val).unwrap())
			}
			NemaSkill::COUNT
			..
			const { NemaSkill::COUNT + EthelSkill::COUNT } => {
				SkillName::FromEthel(EthelSkill::from_repr(val - NemaSkill::COUNT).unwrap())
			}
			const { NemaSkill::COUNT + EthelSkill::COUNT }
			..
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT } => {
				SkillName::FromCrabdra(CrabdraSkill::from_repr(val - NemaSkill::COUNT - EthelSkill::COUNT).unwrap())
			}
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT }
			.. 
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT + BellPlantSkill::COUNT } => {
				SkillName::FromBellPlant(BellPlantSkill::from_repr(val - NemaSkill::COUNT - EthelSkill::COUNT - CrabdraSkill::COUNT).unwrap())
			}
			_ => return Err(FromVariantError::Unspecified),
		})
	}
}

impl ToVariant for SkillName {
	fn to_variant(&self) -> Variant {
		match self {
			SkillName::FromNema(nema) => {
				(*nema as usize).to_variant()
			}
			SkillName::FromEthel(ethel) => {
				(NemaSkill::COUNT + *ethel as usize).to_variant()
			}
			SkillName::FromCrabdra(crabdra) => {
				(NemaSkill::COUNT + EthelSkill::COUNT + *crabdra as usize).to_variant()
			}
			SkillName::FromBellPlant(bellplant) => {
				(NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT + *bellplant as usize).to_variant()
			}
		}
	}
}

impl Export for SkillName {
	type Hint = IntHint<u16>;

	fn export_info(_hint: Option<Self::Hint>) -> ExportInfo {
		let values =
			NemaSkill::VARIANTS.iter()
				.chain(EthelSkill::VARIANTS)
				.chain(CrabdraSkill::VARIANTS)
				.chain(BellPlantSkill::VARIANTS)
				.map(|v| v.to_string())
				.collect::<Vec<_>>();

		Self::Hint::Enum(EnumHint::new(values)).export_info()
	}
}