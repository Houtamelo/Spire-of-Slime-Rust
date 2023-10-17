use rand::prelude::IteratorRandom;

static AllStats: [ModifiableStat; 14] = [
	ModifiableStat::ACC,
	ModifiableStat::CRIT,
	ModifiableStat::DODGE,
	ModifiableStat::TOUGHNESS,
	ModifiableStat::COMPOSURE,
	ModifiableStat::POWER,
	ModifiableStat::SPD,
	ModifiableStat::DEBUFF_RES,
	ModifiableStat::POISON_RES,
	ModifiableStat::MOVE_RES,
	ModifiableStat::DEBUFF_RATE,
	ModifiableStat::POISON_RATE,
	ModifiableStat::MOVE_RATE,
	ModifiableStat::STUN_DEF,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ModifiableStat {
	ACC,
	CRIT,
	DODGE,
	TOUGHNESS,
	COMPOSURE,
	POWER,
	SPD,
	DEBUFF_RES,
	POISON_RES,
	MOVE_RES,
	DEBUFF_RATE,
	POISON_RATE,
	MOVE_RATE,
	STUN_DEF,
}

impl ModifiableStat {
	pub fn get_non_girl_random_except(rng: &mut StdRng, except: ModifiableStat) -> ModifiableStat {
		let mut possibles : HashSet<ModifiableStat> = AllStats.into();
		possibles.remove(&ModifiableStat::COMPOSURE);
		possibles.remove(&except);
		return possibles.iter().choose(rng).unwrap().clone();
	}

	pub fn get_non_girl_random(rng: &mut StdRng) -> ModifiableStat {
		let mut possibles : HashSet<ModifiableStat> = AllStats.into();
		possibles.remove(&ModifiableStat::COMPOSURE);
		return *possibles.iter().choose(rng).unwrap();
	}
}