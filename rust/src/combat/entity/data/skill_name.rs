#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
use crate::combat::entity::data::girls::ethel::skills::EthelSkill;
use crate::combat::entity::data::girls::nema::skills::NemaSkill;
use crate::combat::entity::data::npc::bellplant::BellPlantSkill;
use crate::combat::entity::data::npc::crabdra::CrabdraSkill;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillName {
	FromNema(NemaSkill),
	FromEthel(EthelSkill),
	FromCrabdra(CrabdraSkill),
	FromBellPlant(BellPlantSkill),
}