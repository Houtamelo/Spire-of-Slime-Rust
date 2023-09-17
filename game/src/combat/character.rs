use crate::util::RemainingTicks;
use crate::util::Range;

#[derive(Debug)]
pub struct Character {
	guid: usize,
	last_damager_guid: Option<usize>,
	stamina_cur: isize,
	stamina_max: isize,
	toughness: isize,
	charge: Option<RemainingTicks>,
	recovery: Option<RemainingTicks>,
	stun: Option<RemainingTicks>,
	stun_def: isize,
	stun_redundancy_ms: Option<i64>,
	girl: Option<Girl>,
	size: isize,
	debuff_res: isize,
	debuff_rate: isize,
	move_res: isize,
	move_rate: isize,
	poison_res: isize,
	poison_rate: isize,
	spd: isize,
	acc: isize,
	crit: isize,
	dodge: isize,
	damage: Range,
	power: isize,
	persistent_effects: Vec<persistent::PersistentEffect>,
}

#[derive(Debug)]
pub struct Girl {
	lust: isize,
	temptation: isize,
	composure: isize,
	downed: Option<RemainingTicks>,
}

impl Character {
	pub fn stat(&self, stat: ModifiableStat) -> isize {
		return match stat {
			ModifiableStat::DEBUFF_RES => self.debuff_res,
			ModifiableStat::POISON_RES => self.poison_res,
			ModifiableStat::MOVE_RES => self.move_res,
			ModifiableStat::ACC => self.acc,
			ModifiableStat::CRIT => self.crit,
			ModifiableStat::DODGE => self.dodge,
			ModifiableStat::TOUGHNESS => self.toughness,
			ModifiableStat::COMPOSURE => match &self.girl {
				None => 0,
				Some(girl) => {girl.composure}
			},
			ModifiableStat::POWER => self.power,
			ModifiableStat::SPD => self.spd,
			ModifiableStat::DEBUFF_RATE => self.debuff_rate,
			ModifiableStat::POISON_RATE => self.poison_rate,
			ModifiableStat::MOVE_RATE => self.move_rate,
			ModifiableStat::STUN_DEF => self.stun_def,
		};
	}
}

impl PartialEq<Self> for Character {
	fn eq(&self, other: &Self) -> bool { return self.guid == other.guid; }
}

impl Eq for Character { }