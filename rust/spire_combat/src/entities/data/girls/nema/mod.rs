use super::*;

mod perks;
mod skills;
mod stats;

pub use perks::*;
pub use skills::*;
pub use stats::*;

pub static DEFAULT_NEMA: LazyLock<NemaData> = LazyLock::new(|| {
	NemaData {
		size: Size::from(1),
		dmg: SaneRange::new(6, 10).unwrap(),
		spd: Speed::from(100),
		acc: Accuracy::from(0),
		crit: CritRate::from(0),
		dodge: Dodge::from(5),
		max_stamina: MaxStamina::from(20),
		toughness: Toughness::from(0),
		stun_def: StunDef::from(0),
		debuff_res: DebuffRes::from(0),
		debuff_rate: DebuffRate::from(0),
		move_res: MoveRes::from(0),
		move_rate: MoveRate::from(0),
		poison_res: PoisonRes::from(10),
		poison_rate: PoisonRate::from(0),
		composure: Composure::from(0),
		orgasm_limit: OrgasmLimit::from(3),
		skills: vec![
			Skill::Offensive(GAWKY.clone()),
			Skill::Defensive(CALM.clone()),
		],
	}
});
