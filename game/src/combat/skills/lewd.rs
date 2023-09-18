#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LewdSkill {
	recovery_ms: i64,
	charge_ms: i64,
	acc_mode: ACCMode,
	dmg: DMGMode,
	crit: CRITMode,
	effects_self: Vec<SelfApplier>,
	effects_target: Vec<TargetApplier>,
	target_positions: PositionSetup,
	multi_target: bool,
	use_counter: UseCounter,
	allowed_targets_keys: Vec<String>,
}

impl LewdSkill {
	pub fn calc_dmg(&self, caster: &Character, target: &Character, crit: bool) -> Option<Range> {
		let (power, toughness_reduction) = match self.dmg {
			DMGMode::Power { power, toughness_reduction } => { (power, toughness_reduction) }
			DMGMode::NoDamage => { return None; }
		};

		let (mut dmg_min, mut dmg_max) = (caster.damage.min, caster.damage.max);

		let base_toughness = target.stat(ModifiableStat::TOUGHNESS);
		let min_toughness = isize::min(base_toughness, 0);
		let final_toughness = isize::max(min_toughness, base_toughness - toughness_reduction);

		let total_power = power * caster.stat(ModifiableStat::POWER) * (100 - final_toughness);

		dmg_max = (dmg_max * total_power) / 1000000;
		dmg_min = isize::min((dmg_min * total_power) / 1000000, dmg_max);

		if crit {
			dmg_max = (dmg_max * 150) / 100;
			dmg_min = (dmg_min * 150) / 100;
		}

		return Some(Range::new(dmg_min, dmg_max));
	}

	pub fn calc_hit_chance(&self, caster: &Character, target: &Character) -> Option<isize> {
		let acc = match self.acc_mode {
			ACCMode::CanMiss { acc } => { acc }
			ACCMode::NeverMiss => { return None; }
		};

		return Some(acc + caster.stat(ModifiableStat::ACC) - target.stat(ModifiableStat::DODGE));
	}

	pub fn calc_crit_chance(&self, caster: &Character, target: &Character) -> Option<isize> {
		let crit = match self.crit {
			CRITMode::CanCrit { crit } => { crit }
			CRITMode::NeverCrit => { return None; }
		};

		return Some(crit + caster.stat(ModifiableStat::CRIT));
	}
}