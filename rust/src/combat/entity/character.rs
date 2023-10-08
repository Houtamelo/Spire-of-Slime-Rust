use std::rc::Rc;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::girl::*;
use crate::combat::entity::position::Position;
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::ModifiableStat;
use crate::util::{I_Range, TrackedTicks};

#[derive(Debug)]
pub struct CombatCharacter {
	pub guid: usize,
	pub last_damager_guid: Option<usize>,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: isize,
	pub stun_def: isize,
	pub stun_redundancy_ms: Option<i64>,
	pub girl_stats: Option<Girl_Stats>,
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
	pub damage: I_Range,
	pub power: isize,
	pub persistent_effects: Vec<PersistentEffect>,
	pub state: CharacterState,
	pub position: Position,
	pub on_defeat: OnDefeat,
	//pub skill_use_counters: HashMap<Rc<String>, usize>, todo!
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
			position_before_grappled: self.position,
			on_defeat: self.on_defeat,
		});
	}

	pub fn increment_skill_counter(&self, skill_key: &mut Rc<String>) {
		
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

#[derive(Debug, Copy, Clone)]
pub enum OnDefeat {
	Vanish,
	CorpseOrDefeatedGirl,
}