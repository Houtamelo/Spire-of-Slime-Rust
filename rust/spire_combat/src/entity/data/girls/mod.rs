#[allow(unused_imports)]
use crate::prelude::*;
use strum_macros::{EnumCount, EnumString, FromRepr, VariantNames};

pub mod nema;
pub mod ethel;

#[enum_delegate::delegate]
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum GirlDataVariant {
	Ethel(EthelData),
	Nema(NemaData),
}

#[enum_delegate::delegate(for(GirlDataVariant))]
pub trait GirlData {
	fn composure(&self) -> Composure;
	fn orgasm_limit(&self) -> OrgasmLimit;
}

#[repr(usize)]
#[derive(Serialize, Deserialize)]
#[derive(FromVariant, ToVariant)]
#[derive(FromRepr, EnumString, VariantNames, EnumCount)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Debug, Clone, Copy)]
pub enum GirlVariant {
	Ethel,
	Nema
}