use std::collections::HashMap;
use std::rc::Rc;
use bounded_integer::BoundedIsize;
use crate::combat::entity::character::{CharacterState, CombatCharacter, OnDefeat};
use crate::combat::entity::position::Position;
use crate::util::{GUID, I_Range, TrackedTicks};

pub const MAX_LUST: isize = 200;

#[derive(Debug, Clone)]
pub struct Girl_Stats {
	pub lust        : BoundedIsize<0, 200>,
	pub temptation  : BoundedIsize<0, 100>,
	pub composure   : BoundedIsize< -100, 300>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
}

#[derive(Debug, Clone)]
pub struct DefeatedGirl_Entity {
	pub guid        : GUID,
	pub data_key: Rc<String>,
	pub lust        : BoundedIsize<0, 200>,
	pub temptation  : BoundedIsize<0, 100>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position: Position,
}

impl DefeatedGirl_Entity {
	pub fn to_grappled(self) -> GrappledGirl {
		return GrappledGirl::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			data_key: self.data_key,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			position_before_grappled: self.position,
		});
	}
}

#[derive(Debug, Clone)]
pub enum GrappledGirl {
	Alive(AliveGirl_Grappled),
	Defeated(DefeatedGirl_Grappled),
}

impl GrappledGirl {
	pub fn guid(&self) -> GUID {
		return match self {
			GrappledGirl::Alive(alive) => alive.guid,
			GrappledGirl::Defeated(defeated) => defeated.guid,
		};
	}
}

#[derive(Debug, Clone)]
pub struct AliveGirl_Grappled {
	pub guid        : GUID,
	pub data_key    : Rc<String>,
	pub stamina_cur : isize,
	pub stamina_max : isize,
	pub lust       : BoundedIsize<    0, 200>,
	pub temptation : BoundedIsize<    0, 100>,
	pub composure  : BoundedIsize< -100, 300>,
	pub toughness  : BoundedIsize< -100, 100>,
	pub stun_def   : BoundedIsize< -100, 300>,
	pub debuff_res : BoundedIsize< -300, 300>,
	pub debuff_rate: BoundedIsize< -300, 300>,
	pub move_res   : BoundedIsize< -300, 300>,
	pub move_rate  : BoundedIsize< -300, 300>,
	pub poison_res : BoundedIsize< -300, 300>,
	pub poison_rate: BoundedIsize< -300, 300>,
	pub spd        : BoundedIsize<   20, 300>,
	pub acc        : BoundedIsize< -300, 300>,
	pub crit       : BoundedIsize< -300, 300>,
	pub dodge      : BoundedIsize< -300, 300>,
	pub damage     : I_Range,
	pub power      : BoundedIsize<0, 500>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position_before_grappled: Position,
	pub on_defeat: OnDefeat,
	pub skill_use_counters: HashMap<Rc<String>, usize>,
}

impl AliveGirl_Grappled {
	pub fn to_non_grappled(self) -> CombatCharacter {
		let girl = Girl_Stats {
			lust: self.lust,
			temptation: self.temptation,
			composure: self.composure,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
		};
		
		let mut position = self.position_before_grappled;
		*position.order_mut() = 0;
		
		return CombatCharacter {
			guid: self.guid,
			data_key: self.data_key,
			last_damager_guid: None,
			stamina_cur: self.stamina_cur,
			stamina_max: self.stamina_max,
			toughness: self.toughness,
			stun_def: self.stun_def,
			stun_redundancy_ms: None,
			girl_stats: Some(girl),
			debuff_res: self.debuff_res,
			debuff_rate: self.debuff_rate,
			move_res: self.move_res,
			move_rate: self.move_rate,
			poison_res: self.poison_res,
			poison_rate: self.poison_rate,
			spd: self.spd,
			acc: self.acc,
			crit: self.crit,
			dodge: self.dodge,
			damage: self.damage,
			power: self.power,
			persistent_effects: vec![],
			state: CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2000) },
			position,
			on_defeat: self.on_defeat,
			skill_use_counters: self.skill_use_counters,
		};
	}

	pub fn to_defeated(self) -> Option<GrappledGirl> {
		return match self.on_defeat {
			OnDefeat::Vanish => None,
			OnDefeat::CorpseOrDefeatedGirl => Some(GrappledGirl::Defeated(DefeatedGirl_Grappled {
				guid: self.guid,
				data_key: self.data_key,
				lust: self.lust,
				temptation: self.temptation,
				orgasm_limit: self.orgasm_limit,
				orgasm_count: self.orgasm_count,
				position_before_grappled: self.position_before_grappled,
			}))
		};
	}
}

#[derive(Debug, Clone)]
pub struct DefeatedGirl_Grappled {
	pub guid: GUID,
	pub data_key: Rc<String>,
	pub lust      : BoundedIsize<0, 200>,
	pub temptation: BoundedIsize<0, 100>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position_before_grappled: Position,
}

impl DefeatedGirl_Grappled {
	pub fn to_non_grappled(self) -> DefeatedGirl_Entity {
		let mut position = self.position_before_grappled;
		*position.order_mut() = 0;
		
		return DefeatedGirl_Entity {
			guid: self.guid,
			data_key: self.data_key,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			position,
		};
	}
}