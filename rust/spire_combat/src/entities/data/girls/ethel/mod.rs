use super::*;

mod perks;
mod skills;
mod stats;

pub use perks::*;
pub use skills::*;
pub use stats::*;

pub static DEFAULT_ETHEL: LazyLock<EthelData> = LazyLock::new(|| {
	EthelData {
		size: Size::from(1),
		dmg: SaneRange::new(8, 12).unwrap(),
		spd: Speed::from(100),
		acc: Accuracy::from(0),
		crit: CritRate::from(0),
		dodge: Dodge::from(10),
		max_stamina: MaxStamina::from(30),
		toughness: Toughness::from(0),
		stun_def: StunDef::from(10),
		debuff_res: DebuffRes::from(0),
		debuff_rate: DebuffRate::from(0),
		move_res: MoveRes::from(10),
		move_rate: MoveRate::from(0),
		poison_res: PoisonRes::from(0),
		poison_rate: PoisonRate::from(0),
		skills: vec![
			Skill::Offensive(CLASH.clone()),
			Skill::Defensive(SAFEGUARD.clone()),
			Skill::Offensive(JOLT.clone()),
		],
		composure: Composure::from(0),
		orgasm_limit: OrgasmLimit::from(3),
	}
});
