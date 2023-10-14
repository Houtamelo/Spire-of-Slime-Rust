use crate::combat::entity::data::girls::ethel::perks::EthelPerk;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::data::npc::bellplant::LurePerk;

#[derive(Debug, Clone)]
pub enum Perk {
	Ethel(EthelPerk),
	Nema(NemaPerk),
	BellPlantLure(LurePerk),
}