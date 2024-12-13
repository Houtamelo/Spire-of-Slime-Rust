use crate::entity::{
	character::{MaybeGrappled, effects::IntervalMS},
	data::girls::{ethel::perks::*, nema::perks::*},
};
#[allow(unused_imports)]
use crate::prelude::*;

impl TargetEffectApplier {
	//noinspection RsLift
	/// returns target if it's still standing
	#[must_use]
	pub fn apply_on_target(
		&self,
		caster: &mut CombatCharacter,
		mut target: CombatCharacter,
		ctx: &mut ActorContext,
		rng: &mut Xoshiro256PlusPlus,
		is_crit: bool,
	) -> Option<CombatCharacter> {
		match self {
			//todo!(
			TargetEffectApplier::Tempt(TemptApplier { base_intensity }) => {
				let Some(girl) = &mut target.girl_stats
				else {
					godot_warn!(
						"{}():Trying to apply tempt to actors {target:?}, but it's not a girl.",
						full_fn_name(&Self::apply_on_target)
					);
					return Some(target);
				};

				let lust_f64 = *girl.lust as f64;
				let lust_squared = lust_f64 * lust_f64;
				let extra_intensity_from_lust = lust_squared / 500.0;
				let multiplier_from_lust = 1.0 + (lust_squared / 80000.0);

				let intensity_f64 =
					(*base_intensity as f64 + extra_intensity_from_lust) * multiplier_from_lust;
				let composure_f64 = *girl.composure as f64;

				let dividend = 10.0
					* (intensity_f64 + (intensity_f64 * intensity_f64 / 500.0)
						- composure_f64 - (composure_f64 * composure_f64 / 500.0));
				let divisor = 125.0
					+ (intensity_f64 * 0.25)
					+ (composure_f64 * 0.25)
					+ (intensity_f64 * composure_f64 * 0.0005);

				let temptation_delta_f64 = dividend / divisor;

				let temptation_delta: u8 = if temptation_delta_f64 > 0. {
					let temptation_delta_f64 = f64::clamp(temptation_delta_f64, 1., u8::MAX as f64);
					temptation_delta_f64.round() as u8
				} else {
					0
				};

				if temptation_delta == 0 {
					return Some(target);
				}

				*girl.temptation += temptation_delta;
				if girl.temptation < 100 {
					return Some(target);
				}

				let CharacterDataVariant::NPC(_) = caster.data
				// making sure caster is a npc (required for grappling)
				else {
					godot_warn!(
						"{}(): Trying to apply tempt to actors {target:?}, but caster {caster:?} isn't an NPC.",
						full_fn_name(&Self::apply_on_target)
					);
					return Some(target);
				};

				match target.maybe_grappled() {
					MaybeGrappled::Yes(victim) => {
						caster.state = CharacterState::Grappling(GrapplingState {
							victim,
							lust_per_interval: unsafe { Int::new_unchecked(45) },
							temptation_per_interval: unsafe { Int::new_unchecked(-5) },
							duration_ms: 5000.into(),
							accumulated_ms: 0.into(),
						});
						return None;
					}
					MaybeGrappled::No(target) => {
						return Some(target);
					}
				}
			}
		}
	}
}
