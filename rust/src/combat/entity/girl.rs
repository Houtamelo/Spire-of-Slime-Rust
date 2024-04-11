#[allow(unused_imports)]
use crate::*;

use crate::combat::shared::*;
use crate::combat::entity::character::OnZeroStamina;
use crate::combat::entity::data::girls::GirlData;

pub const MAX_LUST: u8 = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GirlState {
	pub lust: Lust,
	pub temptation  : Temptation,
	pub composure   : Composure,
	pub orgasm_limit: OrgasmLimit,
	pub orgasm_count: OrgasmCount,
	pub exhaustion  : Exhaustion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefeatedGirl_Entity {
	pub data: GirlData,
	pub guid: Uuid,
	pub lust: Lust,
	pub temptation  : Temptation,
	pub orgasm_limit: OrgasmLimit,
	pub orgasm_count: OrgasmCount,
	pub exhaustion  : Exhaustion,
	pub position: Position,
}

impl DefeatedGirl_Entity {
	pub fn into_grappled(self) -> GrappledGirlEnum {
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

	pub const fn position(&self) -> &Position {
		return &self.position;
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrappledGirlEnum {
	Alive(AliveGirl_Grappled),
	Defeated(DefeatedGirl_Grappled),
}

impl GrappledGirlEnum {
	pub const fn guid(&self) -> Uuid {
		return match self {
			GrappledGirlEnum::Alive(alive) => alive.guid,
			GrappledGirlEnum::Defeated(defeated) => defeated.guid,
		};
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliveGirl_Grappled {
	pub guid: Uuid,
	pub data: GirlData,
	pub stamina_cur: CurrentStamina,
	pub stamina_max: MaxStamina,
	pub lust: Lust,
	pub temptation : Temptation,
	pub composure  : Composure,
	pub toughness  : Toughness,
	pub stun_def   : StunDef,
	pub debuff_res : DebuffRes,
	pub debuff_rate: DebuffRate,
	pub move_res   : MoveRes,
	pub move_rate  : MoveRate,
	pub poison_res : PoisonRes,
	pub poison_rate: PoisonRate,
	pub spd   : Speed,
	pub acc   : Accuracy,
	pub crit  : CritRate,
	pub dodge : Dodge,
	pub damage: CheckedRange,
	pub power : Power,
	pub orgasm_limit: OrgasmLimit,
	pub orgasm_count: OrgasmCount,
	pub exhaustion  : Exhaustion,
	pub position_before_grappled: Position,
	pub on_defeat: OnZeroStamina,
	pub skill_use_counters: HashMap<SkillName, u16>,
	pub perks: Vec<Perk>,
}

impl AliveGirl_Grappled {
	pub fn into_non_grappled(self) -> CombatCharacter {
		let girl = GirlState {
			lust: self.lust,
			temptation: self.temptation,
			composure: self.composure,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
		};
		
		let position = Position {
			order: 0.into(),
			..self.position_before_grappled
		};
		
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
			state: CharacterState::Downed { ticks: TrackedTicks::from_milliseconds(2000.to_sat_u64()) },
			position,
			on_zero_stamina: self.on_defeat,
			skill_use_counters: self.skill_use_counters,
		};
	}

	pub fn into_defeated(self) -> GrappledGirlEnum {
		return GrappledGirlEnum::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			data: self.data,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
			position_before_grappled: self.position_before_grappled,
		});
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefeatedGirl_Grappled {
	pub data: GirlData,
	pub guid: Uuid,
	pub lust: Lust,
	pub temptation: Temptation,
	pub orgasm_limit: OrgasmLimit,
	pub orgasm_count: OrgasmCount,
	pub exhaustion  : Exhaustion,
	pub position_before_grappled: Position,
}

impl DefeatedGirl_Grappled {
	pub fn into_non_grappled(self) -> DefeatedGirl_Entity {
		return DefeatedGirl_Entity {
			guid: self.guid,
			data: self.data,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
			exhaustion: self.exhaustion,
			position: Position {
				order: 0.into(),
				..self.position_before_grappled
			},
		};
	}
}