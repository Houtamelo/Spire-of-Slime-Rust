use std::cell::RefCell;
use std::rc::{Rc, Weak};
use persistent::PersistentEffect;
use crate::combat::effects::persistent;
use crate::combat::ModifiableStat;
use crate::combat::skills::{PositionSetup, Skill};
use crate::util::RemainingTicks;
use crate::util::Range;

#[derive(Debug)]
pub struct CombatCharacter {
	pub guid: usize,
	pub last_damager: Weak<RefCell<CombatCharacter>>,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: isize,
	pub stun_def: isize,
	pub stun_redundancy_ms: Option<i64>,
	pub girl: Option<Girl>,
	pub size: isize,
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
}

#[derive(Debug)]
pub struct Girl {
	pub lust: isize,
	pub temptation: isize,
	pub composure: isize,
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
			ModifiableStat::COMPOSURE   => match &self.girl {
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
}

impl PartialEq<Self> for CombatCharacter {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for CombatCharacter { }

#[derive(Debug)]
pub enum CharacterState {
	Idle,
	Grappling { victim: Rc<RefCell<CombatCharacter>>, lust_per_sec: usize, temptation_per_sec: usize, accumulated_ms: i64 },
	Downed { remaining: RemainingTicks },
	Stunned { remaining: RemainingTicks, skill_intention: Option<SkillIntention>, recovery: Option<RemainingTicks> },
	Charging { skill_intention: SkillIntention },
	Recovering { remaining: RemainingTicks },
}