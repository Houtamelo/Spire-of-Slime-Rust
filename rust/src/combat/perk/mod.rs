use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;
use crate::combat::entity::data::npc::bellplant::LurePerk;

#[derive(Debug, Clone)]
pub enum Perk {
	Ethel(EthelPerk),
	Nema(NemaPerk),
	BellPlantLure(LurePerk),
	AffectedByParalyzingToxins(AffectedByParalyzingToxins),     //todo! If ethel has ParalyzingToxins perk, this perk needs to be added to every single character in combat
	AffectedByConcentratedToxins(AffectedByConcentratedToxins), //todo! If ethel has ConcentratedToxins perk, this perk needs to be added to every single character in combat
}