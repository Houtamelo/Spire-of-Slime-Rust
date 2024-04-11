#[allow(unused_imports)]
use crate::*;

use rand_xoshiro::Xoshiro256PlusPlus;
use crate::combat::shared::*;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::data::girls::ethel::perks::*;
use crate::combat::entity::data::girls::nema::perks::*;
use crate::combat::entity::data::npc::bellplant::LurePerk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Perk {
	Ethel(EthelPerk),
	Nema(NemaPerk),
	BellPlantLure(LurePerk),
}

impl Perk {
	pub(in crate::combat) fn tick_all(mut owner: CombatCharacter, others: &mut HashMap<Uuid, Entity>, 
	                                  ms: SaturatedU64, _rng: &mut Xoshiro256PlusPlus) {
		for perk in owner.perks.iter_mut() {
			match perk {
				Perk::Ethel(EthelPerk::Tank_Vanguard { cooldown_ms }) => {
					cooldown_ms.set(u64::min(cooldown_ms.get() - ms.get(), 10000));
				}
				Perk::Nema(NemaPerk::Healer_Awe { accumulated_ms }) => {
					accumulated_ms.set(u64::min(accumulated_ms.get() + ms.get(), 8000));
				},
				Perk::Nema(NemaPerk::Healer_Alarmed { duration_remaining_ms }) => {
					duration_remaining_ms.set(u64::min(duration_remaining_ms.get() - ms.get(), 0));
				},
				Perk::Nema(NemaPerk::BattleMage_Trust { accumulated_ms }) => {
					accumulated_ms.set(u64::min(accumulated_ms.get() + ms.get(), 7000));
				},
				Perk::Nema(NemaPerk::Poison_Acceptance { accumulated_ms }) => {
					if any_matches!(owner.persistent_effects, PersistentEffect::Poison {..}) {
						*accumulated_ms += ms;
					}

					let round_seconds = accumulated_ms.get() / 1000;
					if round_seconds > 0 {
						*accumulated_ms -= round_seconds * 1000;
						owner.girl_stats.touch(|girl| *girl.lust -= round_seconds * 3);
					}
				},
				_ => {}
			}
		}

		others.insert(owner.guid, Entity::Character(owner));
	}
}

macro_rules! has_perk {
    ($character: expr, $perk_pattern: pat) => {{
	    'outer: loop {             
		    for perk in $character.perks.iter() { 
			    if let $perk_pattern = perk {
				    break 'outer true; 
			    }
		    }  
		    
		    for effect in $character.persistent_effects.iter() { 
			    if let crate::combat::effects::persistent::PersistentEffect::TemporaryPerk { perk, .. } = effect
			        && let $perk_pattern = perk { 
				    break 'outer true;
			    }
		    }  
		    
		    break false;
	    }
    }};
}

macro_rules! get_perk {
    ($character: expr, $perk_pattern: pat) => {{
	    'outer: loop {             
		    for perk in $character.perks.iter() { 
			    if let $perk_pattern = perk {
				    break 'outer Some(perk); 
			    }
		    }  
		    
		    for effect in $character.persistent_effects.iter() { 
			    if let crate::combat::effects::persistent::PersistentEffect::TemporaryPerk { perk, .. } = effect
			        && let $perk_pattern = perk { 
				    break 'outer Some(perk);
			    }
		    }  
		    
		    break None;
	    }
    }};
}

macro_rules! get_perk_mut {
    ($character: expr, $perk_pattern: pat) => {{
	    'outer: loop {             
		    for perk in $character.perks.iter_mut() { 
			    if let $perk_pattern = perk {
				    break 'outer Some(perk); 
			    }
		    }  
		    
		    for effect in $character.persistent_effects.iter_mut() { 
			    if let crate::combat::effects::persistent::PersistentEffect::TemporaryPerk { perk, .. } = effect
			        && let $perk_pattern = perk { 
				    break 'outer Some(perk);
			    }
		    }  
		    
		    break None;
	    }
    }};
}

pub(crate) use {has_perk, get_perk, get_perk_mut};
