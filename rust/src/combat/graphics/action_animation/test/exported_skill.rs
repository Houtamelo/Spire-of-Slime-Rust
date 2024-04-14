use gdnative::export::Export;
use crate::combat::entity::data::skill_name::EthelSkill;
use crate::combat::shared::SkillName;
use crate::internal_prelude::{ExportInfo, FromVariant, FromVariantError, ToVariant, Variant};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SkillWrapper(pub SkillName);

impl Default for SkillWrapper {
	fn default() -> Self {
		SkillWrapper(SkillName::FromEthel(EthelSkill::Safeguard))
	}
}

impl FromVariant for SkillWrapper {
	fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
		Ok(SkillWrapper(SkillName::from_variant(variant)?))
	}
}

impl ToVariant for SkillWrapper {
	fn to_variant(&self) -> Variant {
		self.0.to_variant()
	}
}

impl Export for SkillWrapper {
	type Hint = <SkillName as Export>::Hint;

	fn export_info(hint: Option<Self::Hint>) -> ExportInfo {
		SkillName::export_info(hint)
	}
}
