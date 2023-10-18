use std::collections::HashMap;
use rand::rngs::StdRng;
use crate::combat::entity::character::CombatCharacter;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema ::perks::*;
use crate::combat::entity::data::npc::bellplant::LurePerk;
use crate::combat::entity::Entity;
use crate::util::GUID;

#[derive(Debug, Clone)]
pub enum Perk {
	Ethel(EthelPerk),
	Nema(NemaPerk),
	BellPlantLure(LurePerk),
	AffectedByParalyzingToxins(AffectedByParalyzingToxins),     //todo! If ethel has ParalyzingToxins perk, this perk needs to be added to every single character in combat
	AffectedByConcentratedToxins(AffectedByConcentratedToxins), //todo! If ethel has ConcentratedToxins perk, this perk needs to be added to every single character in combat
}

impl Perk {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<GUID, Entity>, ms: i64, seed: &mut StdRng) {
		for perk in owner.perks.iter_mut() {
			match perk {
				Perk::Nema(NemaPerk::Healer_Awe { stacks, accumulated_ms }) => {
					*accumulated_ms += ms;
					while *accumulated_ms > 1000 {
						*accumulated_ms -= 1000;
						*stacks += 1;
					}
				},
				_ => {}
			}
		}

		others.insert(owner.guid, Entity::Character(owner));
	}
}