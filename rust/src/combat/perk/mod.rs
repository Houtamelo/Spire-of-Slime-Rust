use std::collections::HashMap;
use rand::rngs::StdRng;
use crate::combat::effects::persistent::PersistentEffect;
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
}

impl Perk {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<GUID, Entity>, ms: i64, _seed: &mut StdRng) {
		for perk in owner.perks.iter_mut() {
			match perk {
				Perk::Ethel(EthelPerk::Tank_Vanguard { cooldown_ms }) => {
					*cooldown_ms = i64::clamp(*cooldown_ms - ms, 0, 10000);
				}
				Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms }) => {
					*accumulated_ms = i64::clamp(*accumulated_ms + ms, 0, 8000);
				},
				Perk::Nema(NemaPerk::Healer_Alarmed { duration_remaining_ms }) => {
					*duration_remaining_ms = i64::max(*duration_remaining_ms - ms, 0);
				},
				Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms }) => {
					*accumulated_ms = i64::clamp(*accumulated_ms + ms, 0, 7000);
				},
				Perk::Nema(NemaPerk::Poison_Acceptance { accumulated_ms }) => {
					if owner.persistent_effects.iter().any(|effect| matches!(effect, PersistentEffect::Poison {..})) {
						*accumulated_ms += ms;
					}

					let round_seconds = *accumulated_ms / 1000;
					if round_seconds > 0 {
						*accumulated_ms -= round_seconds * 1000;
						if let Some(girl) = &mut owner.girl_stats {
							girl.lust -= round_seconds * 3;
						}
					}
				},
				_ => {}
			}
		}

		others.insert(owner.guid, Entity::Character(owner));
	}
}