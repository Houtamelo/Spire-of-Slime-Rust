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
	Clone,
	Copy,
	Debug,
)]
pub enum BellPlantSkill {
	Engorge,
	InvigoratingFluids,
}

define_perks! {
	BellPlantPerk as Perk::BellPlant {
		@NO_IMPL AlluringScent { duration_ms: Int },
	}
}

impl IPerk for AlluringScent {
	fn tick(
		&mut self,
		actor: &mut Ptr<Actor>,
		ctx: &mut ActorContext,
		delta_ms: Int,
	) -> PerkTickResult {
		self.duration_ms -= delta_ms;

		if self.duration_ms > 0 {
			PerkTickResult::Active
		} else {
			PerkTickResult::Ended
		}
	}
}

pub static ENGORGE: LazyLock<LewdSkill> = LazyLock::new(|| {
	LewdSkill {
		skill_name: SkillIdent::BellPlant(BellPlantSkill::Engorge),
		recovery_ms: 0.into(),
		charge_ms: 2000.into(),
		acc_mode: AccuracyMode::NeverMiss,
		dmg_mode: DmgMode::NoDamage,
		crit_mode: CritMode::NeverCrit,
		effects_self: vec![],
		effects_target: target_effs![
			LustApplier {
				base_delta: SaneRange::new(6, 10).unwrap(),
			},
			TemptApplier {
				base_intensity: 80.into(),
			}
		],
		caster_positions: positions!("✔️🛑🛑🛑"),
		target_positions: positions!("✔️🛑🛑🛑"),
		multi_target: false,
		use_counter: UseCounter::Unlimited,
	}
});

pub static INVIGORATING_FLUIDS: LazyLock<DefensiveSkill> = LazyLock::new(|| {
	DefensiveSkill {
		skill_name: SkillIdent::BellPlant(BellPlantSkill::InvigoratingFluids),
		recovery_ms: 0.into(),
		charge_ms: 2000.into(),
		crit_mode: CritMode::CanCrit { chance: 5.into() },
		effects_caster: vec![],
		effects_target: target_effs![PersistentHealApplier {
			base_duration_ms: 4000.into(),
			base_heal_per_interval: 1.into(),
		},],
		caster_positions: positions!("🛑✔️✔️✔️"),
		target_positions: positions!("✔️✔️✔️✔️"),
		ally_requirement: AllyRequirement::CanSelf,
		multi_target: true,
		use_counter: UseCounter::Limited { max_uses: 2.into() },
	}
});
