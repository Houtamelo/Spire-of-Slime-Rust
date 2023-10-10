use std::collections::HashMap;
use std::rc::Rc;
use bounded_integer::BoundedIsize;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::girl::*;
use crate::combat::entity::position::Position;
use crate::combat::entity::skill_intention::SkillIntention;
use crate::combat::ModifiableStat;
use crate::util::{GUID, I_Range, TrackedTicks};

#[derive(Debug, Clone)]
pub struct CombatCharacter {
	pub guid: GUID,
	pub data_key: Rc<String>,
	pub last_damager_guid: Option<GUID>,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: BoundedIsize< -100, 100>,
	pub stun_def: BoundedIsize< -100, 300>,
	pub stun_redundancy_ms: Option<i64>,
	pub girl_stats: Option<Girl_Stats>,
	pub debuff_res: BoundedIsize< -300, 300>,
	pub debuff_rate: BoundedIsize< -300, 300>,
	pub move_res: BoundedIsize< -300, 300>,
	pub move_rate: BoundedIsize< -300, 300>,
	pub poison_res: BoundedIsize< -300, 300>,
	pub poison_rate: BoundedIsize< -300, 300>,
	pub spd: BoundedIsize<20, 300>,
	pub acc: BoundedIsize< -300, 300>,
	pub crit: BoundedIsize< -300, 300>,
	pub dodge: BoundedIsize< -300, 300>,
	pub damage: I_Range,
	pub power: BoundedIsize<0, 500>,
	pub persistent_effects: Vec<PersistentEffect>,
	pub state: CharacterState,
	pub position: Position,
	pub on_defeat: OnDefeat,
	pub skill_use_counters: HashMap<Rc<String>, usize>,
}

impl CombatCharacter {
	pub fn stat(&self, stat: ModifiableStat) -> isize {
		return match stat {
			ModifiableStat::DEBUFF_RES  => self.debuff_res.get(),
			ModifiableStat::POISON_RES  => self.poison_res.get(),
			ModifiableStat::MOVE_RES    => self.move_res.get(),
			ModifiableStat::ACC         => self.acc.get(),
			ModifiableStat::CRIT        => self.crit.get(),
			ModifiableStat::DODGE       => self.dodge.get(),
			ModifiableStat::TOUGHNESS   => self.toughness.get(),
			ModifiableStat::COMPOSURE   => match &self.girl_stats {
				None => 0,
				Some(girl) => {girl.composure.get()}
			},
			ModifiableStat::POWER       => self.power.get(),
			ModifiableStat::SPD         => self.spd.get(),
			ModifiableStat::DEBUFF_RATE => self.debuff_rate.get(),
			ModifiableStat::POISON_RATE => self.poison_rate.get(),
			ModifiableStat::MOVE_RATE   => self.move_rate.get(),
			ModifiableStat::STUN_DEF    => self.stun_def.get(),
		};
	}

	pub fn to_grappled(self, girl: Girl_Stats) -> GrappledGirl {
		return GrappledGirl::Alive(AliveGirl_Grappled {
			guid: self.guid,
			data_key: self.data_key,
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
			skill_use_counters: self.skill_use_counters,
		});
	}

	pub fn increment_skill_counter(&mut self, skill_key: &Rc<String>) {
		self.skill_use_counters.entry(skill_key.clone()).and_modify(|c| *c += 1).or_insert(1);
	}
	
	pub fn skill_counter_bellow_limit(&self, skill_key: &Rc<String>, limit: usize) -> bool {
		return match self.skill_use_counters.get(skill_key) {
			None => true,
			Some(count) => *count < limit,
		};
	}
	
	/// Used to check if character died after losing stamina.
	pub fn is_alive(&self) -> bool {
		return self.stamina_cur > 0 && self.stamina_max > 0;
	}
	
	pub fn is_dead(&self) -> bool {
		return !self.is_alive();
	}
}

impl PartialEq<Self> for CombatCharacter {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for CombatCharacter { }

#[derive(Debug, Clone)]
pub struct State_Grappling {
	pub victim: GrappledGirl,
	pub lust_per_sec: usize,
	pub temptation_per_sec: usize,
	pub duration_ms: i64,
	pub accumulated_ms: i64
}

#[derive(Debug, Clone)]
pub enum CharacterState {
	Idle,
	Grappling(State_Grappling),
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

#[derive(Debug, Clone)]
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