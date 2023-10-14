use std::convert::Into;
use lazy_static::lazy_static;
use crate::combat::entity::data::girls::ethel::stats::EthelData;

pub mod stats;
pub mod skills;
pub mod perks;

lazy_static! { pub static ref default_ethel: EthelData = EthelData {
	size        : 1.into(),
	dmg         : 8..=12,
	spd         : 100.into(),
	acc         : 0  .into(),
	crit        : 0  .into(),
	dodge       : 10 .into(),
	stamina_max : 30 .into(),
	toughness   : 0  .into(),
	stun_def    : 10 .into(),
	debuff_res  : 0  .into(),
	debuff_rate : 0  .into(),
	move_res    : 10 .into(),
	move_rate   : 0  .into(),
	poison_res  : 0  .into(),
	poison_rate : 0  .into(),
	skills      : vec![&skills::skill_ethel_clash,
	                    skills::skill_ethel_safeguard.deref(),
	                    skills::skill_ethel_jolt.deref()],
	composure   : 0  .into(),
	orgasm_limit: 3  .into(),
};}