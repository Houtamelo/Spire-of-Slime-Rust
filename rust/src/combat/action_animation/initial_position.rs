use std::collections::HashMap;
use std::iter::once;

use anyhow::{anyhow, Result};
use comfy_bounded_ints::prelude::Bound_u8;
use gdnative::api::*;
use gdnative::prelude::*;
use util_gdnative::prelude::IntoSharedArray;
use uuid::Uuid;

use crate::combat::action_animation::ActionParticipant;
use crate::combat::entity::position::Side;
use crate::misc::SaturatedU8;

const DEFAULT_DEFENSIVE_PADDING: DefensivePadding = DefensivePadding {
	center_to_allies: 2.,
	between_allies: 2.,
};

pub struct DefensivePadding {
	center_to_allies: f64,
	between_allies: f64,
}

const DEFAULT_OFFENSIVE_PADDING: OffensivePadding = OffensivePadding {
	center_to_caster: 2.,
	center_to_enemies: 2.,
	between_enemies: 2.,
};

pub struct OffensivePadding {
	center_to_caster: f64,
	center_to_enemies: f64,
	between_enemies: f64,
}

pub enum SkillPadding {
	Defensive(DefensivePadding),
	OffensiveSkill(OffensivePadding),
}

type Order = SaturatedU8;
type Size = Bound_u8<1, { u8::MAX }>;

// returns absolute values, characters on left side need to have their positions negated
fn calc_defensive_positions<'a>(padding: &'a DefensivePadding,
                                caster: &'a ActionParticipant,
                                allies: impl Iterator<Item = &'a ActionParticipant>)
                                -> impl Iterator<Item = (&'a ActionParticipant, f64)> {
	let participants_by_position = {
		let mut temp =
			once(caster).chain(allies).collect::<Vec<_>>();
		
		temp.sort_by(|lhs, rhs| 
			lhs.pos.order.get().cmp(&rhs.pos.order.get()));
		
		temp.into_iter()
	};
	
	participants_by_position
		.scan(SaturatedU8::new(0), |size_sum, participant| {
			let abs_pos_x = 
				(0..participant.pos.size.get())
					.fold(0., |sum, i| {
						let position = size_sum.get() + i;
						sum + padding.center_to_allies + (position as f64 * padding.between_allies)
					});
			
			*size_sum += participant.pos.size;
			
			Some((participant, abs_pos_x))
		})
}

// returns absolute values, characters on left side need to have their positions negated
fn calc_offensive_positions<'a>(padding: &'a OffensivePadding,
                                caster: &'a ActionParticipant,
                                enemies: impl Iterator<Item = &'a ActionParticipant>)
                                -> impl Iterator<Item = (&'a ActionParticipant, f64)> {
	let enemies_by_position = {
		let mut temp =
			enemies.collect::<Vec<_>>();

		temp.sort_by(|lhs, rhs|
			lhs.pos.order.get().cmp(&rhs.pos.order.get()));

		temp.into_iter()
	};

	enemies_by_position
		.scan(SaturatedU8::new(0), |size_sum, participant| {
			let abs_pos_x =
				(0..participant.pos.size.get())
					.fold(0., |sum, i| {
						let position = size_sum.get() + i;
						sum + padding.center_to_enemies + (position as f64 * padding.between_enemies)
					});

			*size_sum += participant.pos.size;

			Some((participant, abs_pos_x))
		})
		.chain(once((caster, padding.center_to_caster)))
}

pub fn lerp_positions<'a>(padding: &'a SkillPadding,
                          caster: &'a ActionParticipant,
                          others: impl Iterator<Item = &'a ActionParticipant>,
                          duration: f64,
                          pos_y: f64)
                          -> Result<HashMap<Uuid, Ref<SceneTreeTween>>> {
	let positions: Vec<_> =
		match padding {
			SkillPadding::Defensive(padding) =>
				calc_defensive_positions(padding, caster, others).collect(),
			SkillPadding::OffensiveSkill(padding) =>
				calc_offensive_positions(padding, caster, others).collect(),
		};
	
	positions
		.into_iter()
		.map(|(participant, abs_pos_x)| {
			unsafe { participant.script.assume_safe() }
				.map(|script, _| {
					let pos_x =
						match participant.pos.side {
							Side::Left => -abs_pos_x,
							Side::Right => abs_pos_x,
						};
					
					unsafe { script.owner().assume_safe_if_sane() }
						.ok_or_else(|| anyhow!("lerp_positions(): script.owner() is not sane."))
						.and_then(|owner| {
							owner.create_tween()
								 .ok_or_else(|| anyhow!("lerp_positions(): Failed to create tween."))
								 .and_then(|tween_ref| {
									 let target_pos = Vector2::new(pos_x as f32, pos_y as f32);
									 
									 unsafe { tween_ref.assume_safe() }
										 .tween_property(owner, "position", target_pos.to_shared_array(), duration)
										 .ok_or_else(|| anyhow!("lerp_positions(): Could not create `position` property tweener."))?;
									 
									 Ok((script.guid(), tween_ref))
								 })
						})
				}).map_err(|err| anyhow!("lerp_positions(): {err}"))
				.flatten()
		}).try_collect()
}