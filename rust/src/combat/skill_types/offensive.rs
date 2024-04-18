#[allow(unused_imports)]
use crate::*;
use crate::combat::shared::*;
use crate::combat::effects::onSelf::SelfApplier;
use crate::combat::effects::onTarget::TargetApplier;
use crate::combat::effects::persistent::PersistentEffect;
use crate::combat::entity::character::*;
use crate::combat::entity::data::girls::nema::perks::NemaPerk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffensiveSkill {
	pub skill_name: SkillName,
	pub recovery_ms: SaturatedU64,
	pub charge_ms  : SaturatedU64,
	pub can_be_riposted : bool,
	pub acc_mode: ACCMode,
	pub dmg_mode: DMGMode,
	pub crit_mode: CRITMode,
	pub custom_modifiers: DynamicArray<CustomOffensiveModifier>,
	pub effects_self    : DynamicArray<SelfApplier>,
	pub effects_target  : DynamicArray<TargetApplier>,
	pub caster_positions: PositionMatrix,
	pub target_positions: PositionMatrix,
	pub multi_target    : bool,
	pub use_counter     : UseCounter,
}

// todo! This is used as data but the calculations don't use it
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomOffensiveModifier {
	BonusVsMarked {
		power: i16,
		acc: i16,
		crit: i16,
	}
}

impl OffensiveSkill {
	pub fn calc_dmg(&self, caster: &CombatCharacter, target: &CombatCharacter, is_crit: bool) -> Option<CheckedRange> {
		let DMGMode::Power { power, toughness_reduction} = self.dmg_mode 
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
	
	pub fn calc_dmg_independent(skill_power: Power, toughness_reduction: ToughnessReduction,
	                            caster: &CombatCharacter, target: &CombatCharacter, is_crit: bool) 
		-> CheckedRange {
		let toughness = {
			let base = target.dyn_stat::<Toughness>().squeeze_to_i64();
			let min = i64::min(base, 0);
			i64::max(min, base - toughness_reduction.squeeze_to_i64())
		};

		let total_power = {
			let mut temp = skill_power.to_sat_i64();
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
		
		return CheckedRange::floor(final_min.squeeze_to(), final_max.squeeze_to());
	}
	
	pub fn final_hit_chance(&self, caster: &CombatCharacter, target: &CombatCharacter) -> Option<PercentageU8> {
		return match self.acc_mode {
			ACCMode::CanMiss { acc } => Some(OffensiveSkill::final_hit_chance_independent(acc, caster, target)),
			ACCMode::NeverMiss => None,
		};
	}

	pub fn final_hit_chance_independent(skill_acc: Accuracy, caster: &CombatCharacter, target: &CombatCharacter) -> PercentageU8 {
		let final_hit_chance = {
			let mut temp = skill_acc.to_sat_i64();
			temp += caster.dyn_stat::<Accuracy>().get();
			temp -= target.dyn_stat::<Dodge>().get();

			if has_perk!(target, Perk::Nema(NemaPerk::Poison_Disbelief))
				&& any_matches!(caster.persistent_effects, PersistentEffect::Poison {..}) {
				temp -= 20;
			}
			
			temp.to_percent_u8()
		};
		
		return final_hit_chance;
	}
	
	pub fn final_crit_chance(&self, caster: &CombatCharacter) -> Option<PercentageU8> {
		return match self.crit_mode {
			CRITMode::CanCrit { chance } => Some(OffensiveSkill::final_crit_chance_independent(chance, caster)),
			CRITMode::NeverCrit => None,
		};
	}
	
	pub fn final_crit_chance_independent(skill_crit: CritRate, caster: &CombatCharacter) -> PercentageU8 {
		let final_crit_chance = {
			let mut temp = skill_crit.to_sat_i64();
			temp += caster.dyn_stat::<CritRate>().get();
			temp.to_percent_u8()
		};
		
		return final_crit_chance;
	}
}

impl SkillData for OffensiveSkill {
	fn name(&self) -> SkillName { return self.skill_name  ; }
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
