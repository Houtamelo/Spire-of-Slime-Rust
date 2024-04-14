use gdnative::core_types::FromVariant;
use gdnative::export::Export;
use util_gdnative::prelude::ToVariant;
use crate::combat::shared::{CharacterName, GirlName};
use crate::internal_prelude::{ExportInfo, FromVariantError, Variant};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NameWrapper(pub CharacterName);

impl Default for NameWrapper {
	fn default() -> Self {
		NameWrapper(CharacterName::Girl(GirlName::Ethel))
	}
}

impl FromVariant for NameWrapper {
	fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
		Ok(NameWrapper(CharacterName::from_variant(variant)?))
	}
}

impl ToVariant for NameWrapper {
	fn to_variant(&self) -> Variant {
		self.0.to_variant()
	}
}

impl Export for NameWrapper {
	type Hint = <CharacterName as Export>::Hint;

	fn export_info(hint: Option<Self::Hint>) -> ExportInfo {
		CharacterName::export_info(hint)
	}
}