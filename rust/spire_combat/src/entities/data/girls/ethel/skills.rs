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
pub enum EthelSkill {
	Safeguard,
	Clash,
	Jolt,
	Sever,
	Pierce,
	Challenge,
}

pub static SAFEGUARD: LazyLock<DefensiveSkill> = LazyLock::new(|| {
	DefensiveSkill {
		skill_name: SkillIdent::Ethel(EthelSkill::Safeguard),
		recovery_ms: 1000.into(),
		charge_ms: 0.into(),
		crit_mode: CritMode::NeverCrit,
		effects_caster: self_effs![BuffApplier {
			base_duration_ms: 5000.into(),
			stat: StatEnum::Dodge,
			base_stat_increase: 15.into(),
		},],
		effects_target: target_effs![MakeSelfGuardTarget {
			base_duration_ms: 5000.into(),
		},],
		caster_positions: PositionMatrix::ANY,
		target_positions: PositionMatrix::ANY,
		ally_requirement: AllyRequirement::NotSelf,
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static CLASH: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Ethel(EthelSkill::Clash),
		recovery_ms: 1500.into(),
		charge_ms: 0.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 95.into() },
		dmg_mode: DmgMode::Power {
			power: 100.into(),
			toughness_reduction: 5.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 9.into() },
		custom_modifiers: vec![],
		effects_caster: vec![],
		effects_target: vec![],
		caster_positions: positions!("✔️✔️🛑🛑"),
		target_positions: positions!("✔️✔️🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static JOLT: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Ethel(EthelSkill::Jolt),
		recovery_ms: 1500.into(),
		charge_ms: 0.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 95.into() },
		dmg_mode: DmgMode::Power {
			power: 50.into(),
			toughness_reduction: 0.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 5.into() },
		custom_modifiers: vec![],
		effects_caster: self_effs![MoveApplier {
			base_apply_chance: None,
			direction: MoveDirection::Front(1.into()),
		},],
		effects_target: target_effs![
			MoveApplier {
				base_apply_chance: Some(100.into()),
				direction: MoveDirection::Back(1.into()),
			},
			StunApplier {
				base_force: 100.into(),
			},
		],
		caster_positions: positions!("✔️✔️🛑🛑"),
		target_positions: positions!("✔️🛑🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static SEVER: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Ethel(EthelSkill::Sever),
		recovery_ms: 1500.into(),
		charge_ms: 0.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 90.into() },
		dmg_mode: DmgMode::Power {
			power: 60.into(),
			toughness_reduction: 0.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 0.into() },
		custom_modifiers: vec![],
		effects_caster: vec![],
		effects_target: vec![],
		caster_positions: positions!("✔️🛑🛑🛑"),
		target_positions: positions!("✔️✔️🛑🛑"),
		multi_target: true,
		use_counter: UseCounter::Unlimited,
	}
});

pub static CHALLENGE: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Ethel(EthelSkill::Challenge),
		recovery_ms: 1750.into(),
		charge_ms: 0.into(),
		can_be_riposted: false,
		acc_mode: AccuracyMode::NeverMiss,
		dmg_mode: DmgMode::NoDamage,
		crit_mode: CritMode::NeverCrit,
		custom_modifiers: vec![],
		effects_caster: self_effs![RiposteApplier {
			base_duration_ms: 4000.into(),
			acc_mode: AccuracyMode::CanMiss { acc: 75.into() },
			crit_mode: CritMode::CanCrit {
				chance: { -5 }.into(),
			},
			base_skill_power: 65.into(),
		},],
		effects_target: target_effs![MarkApplier {
			base_duration_ms: 5000.into(),
		},],
		caster_positions: positions!("✔️🛑🛑🛑"),
		target_positions: positions!("✔️✔️✔️✔️"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static PIERCE: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Ethel(EthelSkill::Pierce),
		recovery_ms: 1500.into(),
		charge_ms: 0.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 100.into() },
		dmg_mode: DmgMode::Power {
			power: 80.into(),
			toughness_reduction: 15.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 13.into() },
		custom_modifiers: vec![CustomOffensiveModifier::BonusVsMarked {
			power: 50,
			acc:   10,
			crit:  0,
		}],
		effects_caster: vec![],
		effects_target: vec![],
		caster_positions: positions!("✔️🛑🛑🛑"),
		target_positions: positions!("✔️✔️✔️🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});
