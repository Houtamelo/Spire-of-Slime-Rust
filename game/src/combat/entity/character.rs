use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::ModifiableStat;
use crate::util::RemainingTicks;
use crate::util::Range;

#[derive(Debug)]
pub struct CombatCharacter {
	pub guid: usize,
	pub last_damager_guid: usize,
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
	pub position: Position
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

	pub fn to_grappled(self) -> GrappledCharacter {
		return GrappledCharacter {
			guid: self.guid,
			stamina_cur: self.stamina_cur,
			stamina_max: self.stamina_max,
			toughness: self.toughness,
			stun_def: self.stun_def,
			girl: self.girl,
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
			power: self.power
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
	Grappling { victim: GrappledCharacter, lust_per_sec: usize, temptation_per_sec: usize, accumulated_ms: i64 },
	Downed { remaining: RemainingTicks },
	Stunned { remaining: RemainingTicks, skill_intention: Option<SkillIntention>, recovery: Option<RemainingTicks> },
	Charging { skill_intention: SkillIntention },
	Recovering { remaining: RemainingTicks },
}

#[derive(Debug)]
pub struct GrappledCharacter {
	pub guid: usize,
	pub stamina_cur: isize,
	pub stamina_max: isize,
	pub toughness: isize,
	pub stun_def: isize,
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
}