use util::prelude::DynamicArray;

use crate::combat::entity::data::girls::ethel::stats::EthelData;
use crate::combat::entity::stat::*;

pub mod stats;
pub mod skills;
pub mod perks;

pub static DEFAULT_ETHEL: EthelData = EthelData {
	size : Size::new(1),
	dmg  : CheckedRange::new(8, 12).unwrap(),
	spd  : Speed::new(100),
	acc  : Accuracy::new(0),
	crit : CritRate::new(0),
	dodge: Dodge::new(10),
	max_stamina: MaxStamina::new(30),
	toughness  : Toughness::new(0),
	stun_def   : StunDef::new(10),
	debuff_res : DebuffRes::new(0),
	debuff_rate: DebuffRate::new(0),
	move_res   : MoveRes::new(10),
	move_rate  : MoveRate::new(0),
	poison_res : PoisonRes::new(0),
	poison_rate: PoisonRate::new(0),
	skills: DynamicArray::Static(&[
		skills::CLASH_CONST, 
		skills::SAFEGUARD_CONST, 
		skills::JOLT_CONST
	]),
	composure   : Composure::new(0),
	orgasm_limit: OrgasmLimit::new(3),
};