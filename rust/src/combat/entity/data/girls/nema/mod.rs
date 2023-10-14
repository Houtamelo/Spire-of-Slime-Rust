use lazy_static::lazy_static;
use crate::combat::entity::data::girls::nema::stats::NemaData;

pub mod stats;
pub mod skills;
pub mod perks;

lazy_static! { pub static ref default_nema: NemaData = NemaData {
	size        : 1  .into(),
	dmg         : 6..=10,
	spd         : 100.into(),
	acc         : 0  .into(),
	crit        : 0  .into(),
	dodge       : 5  .into(),
	stamina_max : 20 .into(),
	toughness   : 0  .into(),
	stun_def    : 0  .into(),
	debuff_res  : 0  .into(),
	debuff_rate : 0  .into(),
	move_res    : 0  .into(),
	move_rate   : 0  .into(),
	poison_res  : 10 .into(),
	poison_rate : 0  .into(),
	skills      : vec![skills::skill_nema_gawky.deref(),
	                   skills::skill_nema_calm .deref()],
	composure   : 0  .into(),
	orgasm_limit: 3  .into(),
};}