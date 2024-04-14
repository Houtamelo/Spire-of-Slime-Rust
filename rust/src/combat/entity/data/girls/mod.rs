use enum_variant_type::EnumVariantType;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};

#[allow(unused_imports)]
use crate::*;
use crate::combat::entity::data::girls::ethel::stats::EthelData;
use crate::combat::entity::data::girls::nema::stats::NemaData;
use crate::combat::entity::stat::{Composure, OrgasmLimit};

pub mod nema;
pub mod ethel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GirlData {
	Ethel(EthelData),
	Nema(NemaData),
}

pub trait GirlTrait {
	fn name(&self) -> GirlName;
	fn composure   (&self) -> Composure;
	fn orgasm_limit(&self) -> OrgasmLimit;
}

#[repr(usize)]
#[derive(EnumVariantType)]
#[evt(derive(Clone, Copy, Debug, PartialEq, Eq, Hash))]
#[derive(VariantNames)]
#[derive(FromRepr)]
#[derive(EnumCount)]
#[derive(EnumString)]
#[derive(FromVariant, ToVariant)]
#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum GirlName {
	Ethel,
	Nema
}
