pub mod nema;
pub mod ethel;

use serde::{Deserialize, Serialize};
use crate::combat::entity::data::girls::ethel::stats::EthelData;
use crate::combat::entity::data::girls::nema::stats::NemaData;
use crate::combat::entity::stat::{Composure, OrgasmLimit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GirlData {
	Ethel(EthelData),
	Nema(NemaData),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Serialize, Deserialize)]
pub enum GirlName {
	Ethel,
	Nema
}

pub trait GirlTrait {
	fn name        (&self) -> GirlName;
	fn composure   (&self) -> Composure;
	fn orgasm_limit(&self) -> OrgasmLimit;
}