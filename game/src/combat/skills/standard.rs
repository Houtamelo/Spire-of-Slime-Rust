use std::cell::{BorrowMutError, Ref, RefCell, RefMut};
use std::rc::Rc;
use crate::combat::{Character, ModifiableStat};
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::MAX_CHARACTERS_PER_TEAM;
use crate::util::Range;

pub struct OffensiveSkill {
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
}

impl OffensiveSkill {
	pub fn calc_dmg(&self, caster: &Character, target: &Character) -> Option<Range> {
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
	}
}