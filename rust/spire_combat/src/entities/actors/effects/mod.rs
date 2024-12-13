pub use actor_extensions::*;
pub use common::*;
pub use girl::*;

use super::*;

mod actor_extensions;
mod common;
mod girl;
mod inner_conversions;

pub use inner_conversions::*;

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum CasterApplierEnum {
			OnCommon(CasterApplier),
			OnGirl(CasterGirlApplier),
		}
	}

	DELEGATES: {}
}

delegated_enum! {
	ENUM_OUT: {
		#[derive(Clone, Serialize, Deserialize)]
		pub enum TargetApplierEnum {
			OnCommon(TargetApplier),
			OnGirl(TargetGirlApplier),
		}
	}
	DELEGATES: {}
}

const CRIT_DURATION_MULTIPLIER: u64 = 150;
const CRIT_EFFECT_MULTIPLIER: u64 = 150;
const CRIT_CHANCE_MODIFIER: u16 = 50;

fn clamp_tick_ms(delta_ms: Int, duration_ms: Int) -> Int {
	if delta_ms <= duration_ms {
		delta_ms
	} else {
		godot_warn!(
			"Tick ms is greater than duration_ms. \n\
			 This should not happen. \
			 Tick ms: {delta_ms:?}, duration_ms: {duration_ms:?}"
		);

		duration_ms
	}
}

fn are_allies(caster: &Ptr<Actor>, target: &Ptr<Actor>) -> bool { caster.team == target.team }

fn are_enemies(caster: &Ptr<Actor>, target: &Ptr<Actor>) -> bool { !are_allies(caster, target) }

impl CasterApplierEnum {
	pub fn apply(&self, ctx: &mut ActorContext, caster: &mut Ptr<Actor>, is_crit: bool) {
		match self {
			CasterApplierEnum::OnCommon(fx) => fx.apply_on_caster(ctx, caster, is_crit),
			CasterApplierEnum::OnGirl(fx) => {
				if let Some(girl) = &mut caster.girl.clone() {
					fx.apply_on_caster_girl(ctx, caster, girl, is_crit);
				}
			}
		}
	}
}

impl TargetApplierEnum {
	pub fn apply(
		&self,
		ctx: &mut ActorContext,
		caster: &mut Ptr<Actor>,
		target: &mut Ptr<Actor>,
		is_crit: bool,
	) {
		match self {
			TargetApplierEnum::OnCommon(fx) => fx.apply_on_target(ctx, caster, target, is_crit),
			TargetApplierEnum::OnGirl(fx) => {
				if let Some(girl) = &mut target.girl.clone() {
					fx.apply_on_target_girl(ctx, caster, target, girl, is_crit);
				}
			}
		}
	}
}
