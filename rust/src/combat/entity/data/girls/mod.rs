pub mod nema;
pub mod ethel;

use crate::{BoundISize, BoundU32};
use crate::combat::entity::data::girls::ethel::stats::EthelData;
use crate::combat::entity::data::girls::nema::stats::NemaData;

#[derive(Debug, Clone)]
pub enum GirlData {
	Ethel(EthelData),
	Nema(NemaData),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum GirlName {
	Ethel,
	Nema
}

pub trait GirlTrait {
	fn name        (&self) -> GirlName;
	fn composure   (&self) -> BoundISize<-100, 300>;
	fn orgasm_limit(&self) -> BoundU32<1, 8>;
}