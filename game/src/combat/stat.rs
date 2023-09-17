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