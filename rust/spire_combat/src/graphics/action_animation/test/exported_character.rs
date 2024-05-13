use gdnative::core_types::FromVariant;
use gdnative::export::Export;
use util_gdnative::prelude::ToVariant;
use crate::prelude::{CharacterVariant, GirlVariant};
use crate::prelude::{ExportInfo, FromVariantError, Variant};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NameWrapper(pub CharacterVariant);

impl Default for NameWrapper {
	fn default() -> Self {
		NameWrapper(CharacterVariant::Girl(GirlVariant::Ethel))
	}
}

impl FromVariant for NameWrapper {
	fn from_variant(variant: &Variant) -> Result<Self, FromVariantError> {
		Ok(NameWrapper(CharacterVariant::from_variant(variant)?))
	}
}

impl ToVariant for NameWrapper {
	fn to_variant(&self) -> Variant {
		self.0.to_variant()
	}
}

impl Export for NameWrapper {
	type Hint = <CharacterVariant as Export>::Hint;

	fn export_info(hint: Option<Self::Hint>) -> ExportInfo {
		CharacterVariant::export_info(hint)
	}
}