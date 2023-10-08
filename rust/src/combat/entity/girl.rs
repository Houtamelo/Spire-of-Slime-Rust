use crate::combat::entity::character::{CharacterState, CombatCharacter, OnDefeat};
use crate::combat::entity::position::Position;
use crate::util::{I_Range, TrackedTicks};

pub const MAX_LUST: isize = 200;

#[derive(Debug, Clone, Copy)]
pub struct Girl_Stats {
	pub lust        : isize,
	pub temptation  : isize,
	pub composure   : isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
}

#[derive(Debug)]
pub struct DefeatedGirl_Entity {
	pub guid        : usize,
	pub lust        : isize,
	pub temptation  : isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position: Position,
}

impl DefeatedGirl_Entity {
	pub fn to_grappled(self) -> GrappledGirl {
		return GrappledGirl::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			position_before_grappled: self.position,
		});
	}
}

#[derive(Debug)]
pub enum GrappledGirl {
	Alive(AliveGirl_Grappled),
	Defeated(DefeatedGirl_Grappled),
}

impl GrappledGirl {
	pub fn guid(&self) -> usize {
		return match self {
			GrappledGirl::Alive(alive) => alive.guid,
			GrappledGirl::Defeated(defeated) => defeated.guid,
		};
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AliveGirl_Grappled {
	pub guid        : usize,
	pub stamina_cur : isize,
	pub stamina_max : isize,
	pub toughness   : isize,
	pub stun_def    : isize,
	pub debuff_res  : isize,
	pub debuff_rate : isize,
	pub move_res    : isize,
	pub move_rate   : isize,
	pub poison_res  : isize,
	pub poison_rate : isize,
	pub spd         : isize,
	pub acc         : isize,
	pub crit        : isize,
	pub dodge       : isize,
	pub damage      : I_Range,
	pub power       : isize,
	pub lust        : isize,
	pub temptation  : isize,
	pub composure   : isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position_before_grappled: Position,
	pub on_defeat: OnDefeat,
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
		};
	}

	pub fn to_defeated(self) -> Option<GrappledGirl> {
		return match self.on_defeat {
			OnDefeat::Vanish => None,
			OnDefeat::CorpseOrDefeatedGirl => Some(GrappledGirl::Defeated(DefeatedGirl_Grappled {
				guid: self.guid,
				lust: self.lust,
				temptation: self.temptation,
				orgasm_limit: self.orgasm_limit,
				orgasm_count: self.orgasm_count,
				position_before_grappled: self.position_before_grappled,
			}))
		};
	}
}

#[derive(Debug)]
pub struct DefeatedGirl_Grappled {
	pub guid: usize,
	pub lust: isize,
	pub temptation: isize,
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
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			position,
		};
	}
}