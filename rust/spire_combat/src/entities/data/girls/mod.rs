use super::*;

mod ethel;
mod nema;

pub use ethel::*;
pub use nema::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum GirlDataEnum {
	Ethel(EthelData),
	Nema(NemaData),
}

pub trait GirlData {
	fn composure(&self) -> Composure;
	fn orgasm_limit(&self) -> OrgasmLimit;
}

#[repr(usize)]
#[derive(
	Serialize,
	Deserialize,
	FromRepr,
	EnumString,
	VariantNames,
	EnumCount,
	PartialEq,
	Eq,
	Hash,
	Debug,
	Clone,
	Copy,
)]
pub enum GirlName {
	Ethel,
	Nema,
}
