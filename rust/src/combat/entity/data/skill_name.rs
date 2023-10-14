use crate::combat::entity::data::girls::ethel::skills::EthelSkillName;
use crate::combat::entity::data::girls::nema::skills::NemaSkillName;
use crate::combat::entity::data::npc::bellplant::BellPlantSkillName;
use crate::combat::entity::data::npc::crabdra::CrabdraSkillName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillName {
	FromNema(NemaSkillName),
	FromEthel(EthelSkillName),
	FromCrabdra(CrabdraSkillName),
	FromBellPlant(BellPlantSkillName),
}