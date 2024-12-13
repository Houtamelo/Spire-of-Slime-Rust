use super::*;

#[repr(usize)]
#[derive(
	Serialize,
	Deserialize,
	VariantNames,
	FromRepr,
	EnumString,
	EnumCount,
	PartialEq,
	Eq,
	Hash,
	Debug,
	Clone,
	Copy,
)]
pub enum CrabdraSkill {
	Crush,
	Glare,
}

pub static CRUSH: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Crabdra(CrabdraSkill::Crush),
		recovery_ms: 0.into(),
		charge_ms: 1500.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 85.into() },
		dmg_mode: DmgMode::Power {
			power: 100.into(),
			toughness_reduction: 0.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 7.into() },
		custom_modifiers: vec![],
		effects_caster: vec![],
		effects_target: vec![],
		caster_positions: positions!("✔️✔️🛑🛑"),
		target_positions: positions!("✔️🛑🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static GLARE: LazyLock<LewdSkill> = LazyLock::new(|| {
	LewdSkill {
		skill_name: SkillIdent::Crabdra(CrabdraSkill::Glare),
		recovery_ms: 0.into(),
		charge_ms: 1700.into(),
		acc_mode: AccuracyMode::NeverMiss,
		dmg_mode: DmgMode::NoDamage,
		crit_mode: CritMode::NeverCrit,
		effects_self: vec![],
		effects_target: target_effs![
			LustApplier {
				base_delta: SaneRange::new(5, 9).unwrap(),
			},
			TemptApplier {
				base_intensity: 100.into(),
			}
		],
		caster_positions: positions!("✔️✔️✔️✔️"),
		target_positions: positions!("✔️✔️🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});
