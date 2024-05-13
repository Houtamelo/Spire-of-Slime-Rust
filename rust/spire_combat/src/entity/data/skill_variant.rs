#[allow(unused_imports)]
use crate::prelude::*;
use std::str::FromStr;
use gdnative::export::Export;
use gdnative::export::hint::{EnumHint, IntHint};
use strum::{EnumCount, VariantNames};

pub use crate::entity::data::girls::ethel::skills::EthelSkill;
pub use crate::entity::data::girls::nema::skills::NemaSkill;
pub use crate::entity::data::npc::bellplant::BellPlantSkill;
pub use crate::entity::data::npc::crabdra::CrabdraSkill;

#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub enum SkillVariant {
	Nema(NemaSkill),
	Ethel(EthelSkill),
	Crabdra(CrabdraSkill),
	BellPlant(BellPlantSkill),
}

impl FromStr for SkillVariant {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self> {
		if let Ok(ok) = NemaSkill::from_str(s) {
			return Ok(Self::Nema(ok));
		}
		
		if let Ok(ok) = EthelSkill::from_str(s) {
			return Ok(Self::Ethel(ok));
		}
		
		if let Ok(ok) = CrabdraSkill::from_str(s) {
			return Ok(Self::Crabdra(ok));
		}
		
		if let Ok(ok) = BellPlantSkill::from_str(s) {
			return Ok(Self::BellPlant(ok));
		}
		
		Err(anyhow!("Invalid SkillName: {s}"))
	}
}

impl FromVariant for SkillVariant {
	fn from_variant(variant: &Variant) -> std::result::Result<Self, FromVariantError> {
		let val = variant.try_to::<usize>()?;
		
		Ok(match val {
			0..NemaSkill::COUNT => {
				SkillVariant::Nema(NemaSkill::from_repr(val).unwrap())
			}
			NemaSkill::COUNT
			..
			const { NemaSkill::COUNT + EthelSkill::COUNT } => {
				SkillVariant::Ethel(EthelSkill::from_repr(val - NemaSkill::COUNT).unwrap())
			}
			const { NemaSkill::COUNT + EthelSkill::COUNT }
			..
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT } => {
				SkillVariant::Crabdra(CrabdraSkill::from_repr(val - NemaSkill::COUNT - EthelSkill::COUNT).unwrap())
			}
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT }
			.. 
			const { NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT + BellPlantSkill::COUNT } => {
				SkillVariant::BellPlant(BellPlantSkill::from_repr(
					val - NemaSkill::COUNT - EthelSkill::COUNT - CrabdraSkill::COUNT).unwrap())
			}
			_ => return Err(FromVariantError::Unspecified),
		})
	}
}

impl ToVariant for SkillVariant {
	fn to_variant(&self) -> Variant {
		match self {
			SkillVariant::Nema(nema) => {
				(*nema as usize).to_variant()
			}
			SkillVariant::Ethel(ethel) => {
				(NemaSkill::COUNT + *ethel as usize).to_variant()
			}
			SkillVariant::Crabdra(crabdra) => {
				(NemaSkill::COUNT + EthelSkill::COUNT + *crabdra as usize).to_variant()
			}
			SkillVariant::BellPlant(bellplant) => {
				(NemaSkill::COUNT + EthelSkill::COUNT + CrabdraSkill::COUNT + *bellplant as usize).to_variant()
			}
		}
	}
}

impl Export for SkillVariant {
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