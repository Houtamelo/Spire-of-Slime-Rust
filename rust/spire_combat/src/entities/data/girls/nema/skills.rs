use super::*;

#[repr(usize)]
#[derive(
	Serialize,
	Deserialize,
	FromRepr,
	EnumString,
	EnumCount,
	VariantNames,
	PartialEq,
	Eq,
	Hash,
	Debug,
	Clone,
	Copy,
)]
pub enum NemaSkill {
	Calm,
	Gawky,
}

pub static CALM: LazyLock<DefensiveSkill> = LazyLock::new(|| {
	DefensiveSkill {
		skill_name: SkillIdent::Nema(NemaSkill::Calm),
		recovery_ms: 1500.into(),
		charge_ms: 0.into(),
		crit_mode: CritMode::CanCrit { chance: 5.into() },
		effects_caster: self_effs![ChangeExhaustionApplier {
			base_delta: 1.into(),
		}],
		effects_target: target_effs![HealApplier {
			base_multiplier: 100.into(),
		},],
		caster_positions: PositionMatrix::ANY,
		target_positions: PositionMatrix::ANY,
		ally_requirement: AllyRequirement::CanSelf,
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static GAWKY: LazyLock<OffensiveSkill> = LazyLock::new(|| {
	OffensiveSkill {
		ident: SkillIdent::Nema(NemaSkill::Gawky),
		recovery_ms: 1000.into(),
		charge_ms: 0.into(),
		can_be_riposted: true,
		acc_mode: AccuracyMode::CanMiss { acc: 90.into() },
		dmg_mode: DmgMode::Power {
			power: 25.into(),
			toughness_reduction: 0.into(),
		},
		crit_mode: CritMode::CanCrit { chance: 15.into() },
		custom_modifiers: vec![],
		effects_caster: self_effs![MoveApplier {
			base_apply_chance: None,
			direction: MoveDirection::Back(1.into()),
		},],
		effects_target: target_effs![StunApplier {
			base_force: 40.into(),
		},],
		caster_positions: positions!("✔️✔️🛑🛑"),
		target_positions: positions!("✔️✔️🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});
