pub mod stats;
pub mod skills;
pub mod perks;

use houta_utils::prelude::DynamicArray;
use crate::combat::entity::data::girls::nema::stats::NemaData;
use crate::combat::entity::stat::*;

pub static DEFAULT_NEMA: NemaData = NemaData {
	size : Size::new(1),
	dmg  : CheckedRange::new(6, 10).unwrap(),
	spd  : Speed::new(100),
	acc  : Accuracy::new(0),
	crit : CritChance::new(0),
	dodge: Dodge::new(5),
	max_stamina : MaxStamina::new(20),
	toughness   : Toughness::new(0),
	stun_def    : StunDef::new(0),
	debuff_res  : DebuffRes::new(0),
	debuff_rate : DebuffRate::new(0),
	move_res    : MoveRes::new(0),
	move_rate   : MoveRate::new(0),
	poison_res  : PoisonRes::new(10),
	poison_rate : PoisonRate::new(0),
	composure   : Composure::new(0),
	orgasm_limit: OrgasmLimit::new(3),
	skills      : DynamicArray::Static(&[
		skills::GAWKY_CONST, 
		skills::CALM_CONST
	]),
};