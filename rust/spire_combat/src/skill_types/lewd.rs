#[allow(unused_imports)]
use crate::prelude::*;
use crate::effects::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LewdSkill {
	pub skill_name: SkillVariant,
	pub recovery_ms: SaturatedU64,
	pub charge_ms  : SaturatedU64,
	pub acc_mode : ACCMode,
	pub dmg_mode : DMGMode,
	pub crit_mode: CRITMode,
	pub effects_self    : DynamicArray<SelfApplier>,
	pub effects_target  : DynamicArray<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target    : bool,
	pub use_counter     : UseCounter,
}

impl LewdSkill {
	pub fn calc_dmg(&self, caster: &CombatCharacter, target: &CombatCharacter, is_crit: bool) -> Option<CheckedRange> {
		let DMGMode::Power { power, toughness_reduction } = self.dmg_mode 
			else { return None; };

		let toughness = {
			let base = target.dyn_stat::<Toughness>().squeeze_to_i64();
			let min = i64::min(base, 0);
			i64::max(min, base - toughness_reduction.squeeze_to_i64())
		};

		let total_power = { 
			let mut temp = power.to_sat_i64();
			temp *= caster.dyn_stat::<Power>().get();
			temp *= 100 - toughness;
			temp /= 10000;
			temp
		};
		
		let (final_min, final_max) = {
			let dmg_range = caster.dmg;
			
			let mut temp_min = dmg_range.bound_lower().to_sat_i64();
			let mut temp_max = dmg_range.bound_upper().to_sat_i64();
			temp_min *= total_power;
			temp_min /= 100;
			temp_max *= total_power;
			temp_max /= 100;
			
			if is_crit {
				temp_min *= 15;
				temp_min /= 10;
				temp_max *= 15;
				temp_max /= 10;
			}
			
			(temp_min, temp_max)
		};
		
		return Some(CheckedRange::floor(final_min.squeeze_to(), final_max.squeeze_to()));
	}

	pub fn calc_hit_chance(&self, caster: &CombatCharacter, target: &CombatCharacter) -> Option<PercentageU8> {
		return match self.acc_mode {
			ACCMode::CanMiss { acc } => {
				let final_acc = {
					let mut temp = acc.to_sat_i64();
					temp += caster.dyn_stat::<Accuracy>().get();
					temp -= target.dyn_stat::<Dodge>().get();
					temp.to_percent_u8()
				};

				Some(final_acc)
			}
			ACCMode::NeverMiss => None,
		};
	}
	
	pub fn calc_crit_chance(&self, caster: &CombatCharacter) -> Option<PercentageU8> {
		return match self.crit_mode {
			CRITMode::CanCrit { chance } => {
				let final_chance = {
					let mut temp = chance.to_sat_i64();
					temp += caster.dyn_stat::<CritRate>().get();
					temp.to_percent_u8()
				};

				Some(final_chance)
			},
			CRITMode::NeverCrit => None,
		};
	}
}

impl SkillData for LewdSkill {
	fn variant(&self) -> SkillVariant { return self.skill_name  ; }
	fn recovery_ms(&self) -> &SaturatedU64 { return &self.recovery_ms; }
	fn charge_ms  (&self) -> &SaturatedU64 { return &self.charge_ms  ; }
	fn crit(&self) -> &CRITMode { return &self.crit_mode; }
	fn effects_self    (&self) -> &[SelfApplier]   { return &self.effects_self    ; }
	fn effects_target  (&self) -> &[TargetApplier] { return &self.effects_target  ; }
	fn caster_positions(&self) -> &PositionMatrix  { return &self.caster_positions; }
	fn target_positions(&self) -> &PositionMatrix  { return &self.target_positions; }
	fn multi_target(&self) -> &bool { return &self.multi_target; }
	fn use_counter (&self) -> &UseCounter { return &self.use_counter ; }
}