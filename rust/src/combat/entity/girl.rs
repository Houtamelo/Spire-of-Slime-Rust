use std::collections::HashMap;
use crate::{BoundISize, BoundU32};
use crate::combat::entity::character::{CharacterState, CombatCharacter, OnDefeat};
use crate::combat::entity::data::character::CharacterData;
use crate::combat::entity::data::girls::{GirlData};
use crate::combat::entity::data::skill_name::SkillName;
use crate::combat::entity::position::Position;
use crate::combat::perk::Perk;
use crate::util::{GUID, I_Range, TrackedTicks};

pub const MAX_LUST: isize = 200;

#[derive(Debug, Clone)]
pub struct GirlState {
	pub lust        : BoundU32<0, 200>,
	pub temptation  : BoundU32<0, 100>,
	pub composure   : BoundISize< -100, 300>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub exhaustion  : BoundU32<0, 100>,
}

#[derive(Debug, Clone)]
pub struct DefeatedGirl_Entity {
	pub data: GirlData,
	pub guid: GUID,
	pub lust        : BoundU32<0, 200>,
	pub temptation  : BoundU32<0, 100>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub exhaustion  : BoundU32<0, 100>,
	pub position: Position,
}

impl DefeatedGirl_Entity {
	pub fn to_grappled(self) -> GrappledGirlEnum {
		return GrappledGirlEnum::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			data: self.data,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
			position_before_grappled: self.position,
		});
	}

	pub fn position(&self) -> &Position {
		return &self.position;
	}
}

#[derive(Debug, Clone)]
pub enum GrappledGirlEnum {
	Alive(AliveGirl_Grappled),
	Defeated(DefeatedGirl_Grappled),
}

impl GrappledGirlEnum {
	pub fn guid(&self) -> GUID {
		return match self {
			GrappledGirlEnum::Alive(alive) => alive.guid,
			GrappledGirlEnum::Defeated(defeated) => defeated.guid,
		};
	}
}

#[derive(Debug, Clone)]
pub struct AliveGirl_Grappled {
	pub guid: GUID,
	pub data: GirlData,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub lust       : BoundU32<0, 200>,
	pub temptation : BoundU32<0, 100>,
	pub composure  : BoundISize< -100, 300>,
	pub toughness  : BoundISize< -100, 100>,
	pub stun_def   : BoundISize< -100, 300>,
	pub debuff_res : BoundISize< -300, 300>,
	pub debuff_rate: BoundISize< -300, 300>,
	pub move_res   : BoundISize< -300, 300>,
	pub move_rate  : BoundISize< -300, 300>,
	pub poison_res : BoundISize< -300, 300>,
	pub poison_rate: BoundISize< -300, 300>,
	pub spd        : BoundU32<   20, 300>,
	pub acc        : BoundISize< -300, 300>,
	pub crit       : BoundISize< -300, 300>,
	pub dodge      : BoundISize< -300, 300>,
	pub damage     : I_Range,
	pub power      : BoundU32<0, 500>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub exhaustion  : BoundU32<0, 100>,
	pub position_before_grappled: Position,
	pub on_defeat: OnDefeat,
	pub skill_use_counters: HashMap<SkillName, usize>,
	pub perks: Vec<Perk>,
}

impl AliveGirl_Grappled {
	pub fn to_non_grappled(self) -> CombatCharacter {
		let girl = GirlState {
			lust: self.lust,
			temptation: self.temptation,
			composure: self.composure,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
		};
		
		let mut position = self.position_before_grappled;
		*position.order_mut() = 0;
		
		return CombatCharacter {
			guid: self.guid,
			data: CharacterData::Girl(self.data),
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
			dmg: self.damage,
			power: self.power,
			persistent_effects: vec![],
			perks: self.perks,
			state: CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2000) },
			position,
			on_defeat: self.on_defeat,
			skill_use_counters: self.skill_use_counters,
		};
	}

	pub fn to_defeated(self) -> Option<GrappledGirlEnum> {
		return match self.on_defeat {
			OnDefeat::Vanish => None,
			OnDefeat::CorpseOrDefeatedGirl => Some(GrappledGirlEnum::Defeated(DefeatedGirl_Grappled {
				guid: self.guid,
				data: self.data,
				lust: self.lust,
				temptation: self.temptation,
				orgasm_limit: self.orgasm_limit,
				orgasm_count: self.orgasm_count,
				exhaustion: self.exhaustion,
				position_before_grappled: self.position_before_grappled,
			}))
		};
	}
}

#[derive(Debug, Clone)]
pub struct DefeatedGirl_Grappled {
	pub data: GirlData,
	pub guid: GUID,
	pub lust      : BoundU32<0, 200>,
	pub temptation: BoundU32<0, 100>,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub exhaustion  : BoundU32<0, 100>,
	pub position_before_grappled: Position,
}

impl DefeatedGirl_Grappled {
	pub fn to_non_grappled(self) -> DefeatedGirl_Entity {
		let mut position = self.position_before_grappled;
		*position.order_mut() = 0;
		
		return DefeatedGirl_Entity {
			guid: self.guid,
			data: self.data,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
			position,
		};
	}
}