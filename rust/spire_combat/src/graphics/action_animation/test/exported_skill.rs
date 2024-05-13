use gdnative::export::Export;
use crate::entity::data::skill_variant::EthelSkill;
use crate::prelude::SkillVariant;
use crate::prelude::{ExportInfo, FromVariant, FromVariantError, ToVariant, Variant};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SkillWrapper(pub SkillVariant);

impl Default for SkillWrapper {
	fn default() -> Self {
		SkillWrapper(SkillVariant::Ethel(EthelSkill::Safeguard))
	}
}

impl FromVariant for SkillWrapper {
	fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
		Ok(SkillWrapper(SkillVariant::from_variant(variant)?))
	}
}

impl ToVariant for SkillWrapper {
	fn to_variant(&self) -> Variant {
		self.0.to_variant()
	}
}

impl Export for SkillWrapper {
	type Hint = <SkillVariant as Export>::Hint;

	fn export_info(hint: Option<Self::Hint>) -> ExportInfo {
		SkillVariant::export_info(hint)
	}
}
