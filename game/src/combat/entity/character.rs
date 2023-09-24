use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::Position::Left;
use crate::combat::ModifiableStat;
use crate::util::TrackedTicks;
use crate::util::Range;

pub const MAX_LUST: isize = 200;

#[derive(Debug)]
pub struct CombatCharacter {
	pub guid: usize,
	pub last_damager_guid: usize,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: isize,
	pub stun_def: isize,
	pub stun_redundancy_ms: Option<i64>,
	pub girl_stats: Option<Girl_Stats>,
	pub size: usize,
	pub debuff_res: isize,
	pub debuff_rate: isize,
	pub move_res: isize,
	pub move_rate: isize,
	pub poison_res: isize,
	pub poison_rate: isize,
	pub spd: isize,
	pub acc: isize,
	pub crit: isize,
	pub dodge: isize,
	pub damage: Range,
	pub power: isize,
	pub persistent_effects: Vec<PersistentEffect>,
	pub state: CharacterState,
	pub position: Position
}

#[derive(Debug, Clone, Copy)]
pub struct Girl_Stats {
	pub lust: isize,
	pub temptation: isize,
	pub composure: isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
}

#[derive(Debug)]
pub struct DefeatedGirl_Entity {
	pub guid: usize,
	pub size: usize,
	pub lust: isize,
	pub temptation: isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
	pub position: Position
}

impl DefeatedGirl_Entity {
	pub fn to_grappled(self) -> GrappledGirl {
		return GrappledGirl::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			size: self.size,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
		});
	}
}

impl CombatCharacter {
	pub fn stat(&self, stat: ModifiableStat) -> isize {
		return match stat {
			ModifiableStat::DEBUFF_RES  => self.debuff_res,
			ModifiableStat::POISON_RES  => self.poison_res,
			ModifiableStat::MOVE_RES    => self.move_res,
			ModifiableStat::ACC         => self.acc,
			ModifiableStat::CRIT        => self.crit,
			ModifiableStat::DODGE       => self.dodge,
			ModifiableStat::TOUGHNESS   => self.toughness,
			ModifiableStat::COMPOSURE   => match &self.girl_stats {
				None => 0,
				Some(girl) => {girl.composure}
			},
			ModifiableStat::POWER       => self.power,
			ModifiableStat::SPD         => self.spd,
			ModifiableStat::DEBUFF_RATE => self.debuff_rate,
			ModifiableStat::POISON_RATE => self.poison_rate,
			ModifiableStat::MOVE_RATE   => self.move_rate,
			ModifiableStat::STUN_DEF    => self.stun_def,
		};
	}

	pub fn to_grappled(self, girl: Girl_Stats) -> GrappledGirl {
		return GrappledGirl::Alive(AliveGirl_Grappled {
			guid: self.guid,
			stamina_cur: self.stamina_cur,
			stamina_max: self.stamina_max,
			toughness: self.toughness,
			stun_def: self.stun_def,
			size: self.size,
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
			lust: girl.lust,
			temptation: girl.temptation,
			composure: girl.composure,
			orgasm_limit: girl.orgasm_limit,
			orgasm_count: girl.orgasm_count,
		});
	}
}

impl PartialEq<Self> for CombatCharacter {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for CombatCharacter { }

#[derive(Debug)]
pub enum CharacterState {
	Idle,
	Grappling { victim: GrappledGirl, lust_per_sec: usize, temptation_per_sec: usize, duration_ms: i64, accumulated_ms: i64 },
	Downed  { ticks: TrackedTicks },
	Stunned { ticks: TrackedTicks, state_before_stunned: StateBeforeStunned },
	Charging { skill_intention: SkillIntention },
	Recovering { ticks: TrackedTicks },
}

impl CharacterState {
	pub fn spd_charge_ms(remaining_ms: i64, character_speed: isize) -> i64 {
		return (remaining_ms * 100) / character_speed as i64;
	}
	
	pub fn spd_recovery_ms(remaining_ms: i64, character_speed: isize) -> i64 {
		return (remaining_ms * 100) / character_speed as i64;
	}
}

#[derive(Debug)]
pub enum StateBeforeStunned {
	Recovering { ticks: TrackedTicks },
	Charging { skill_intention: SkillIntention },
	Idle,
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
	pub guid: usize,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: isize,
	pub stun_def: isize,
	pub size: usize,
	pub debuff_res: isize,
	pub debuff_rate: isize,
	pub move_res: isize,
	pub move_rate: isize,
	pub poison_res: isize,
	pub poison_rate: isize,
	pub spd: isize,
	pub acc: isize,
	pub crit: isize,
	pub dodge: isize,
	pub damage: Range,
	pub power: isize,
	pub lust: isize,
	pub temptation: isize,
	pub composure: isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
}

impl AliveGirl_Grappled {
	// remember to set position afterwards
	pub fn to_non_grappled(self) -> CombatCharacter {
		let girl = Girl_Stats {
			lust: self.lust,
			temptation: self.temptation,
			composure: self.composure,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
		};
		return CombatCharacter {
			guid: self.guid,
			last_damager_guid: 0,
			stamina_cur: self.stamina_cur,
			stamina_max: self.stamina_max,
			toughness: self.toughness,
			stun_def: self.stun_def,
			stun_redundancy_ms: None,
			girl_stats: Some(girl),
			size: self.size,
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
			position: Left { order: 0, size: self.size },
		};
	}
}

impl AliveGirl_Grappled {
	pub fn to_defeated(self) -> GrappledGirl {
		return GrappledGirl::Defeated(DefeatedGirl_Grappled {
			guid: self.guid,
			size: self.size,
			lust: self.lust,
			temptation: self.temptation,
			orgasm_limit: self.orgasm_limit,
			orgasm_count: self.orgasm_count,
		});
	}
}

#[derive(Debug)]
pub struct DefeatedGirl_Grappled {
	pub guid: usize,
	pub size: usize,
	pub lust: isize,
	pub temptation: isize,
	pub orgasm_limit: isize,
	pub orgasm_count: isize,
}