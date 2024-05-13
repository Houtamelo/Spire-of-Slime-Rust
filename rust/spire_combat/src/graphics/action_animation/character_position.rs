#[allow(unused_imports)]
use crate::prelude::*;

use std::iter::once;
use entity_anim::default_position::calc_default_positions;
use crate::graphics::action_animation::ActionParticipant;
use crate::graphics::entity_anim;
use crate::graphics::stages::StagePadding;

#[derive(Debug, Copy, Clone)]
pub struct DefensivePadding {
	center_to_allies: f64,
	between_allies: f64,
}

impl Default for DefensivePadding {
	fn default() -> Self {
		DefensivePadding {
			center_to_allies: 2.,
			between_allies: 2.,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct OffensivePadding {
	center_to_caster: f64,
	center_to_enemies: f64,
	between_enemies: f64,
}

impl Default for OffensivePadding {
	fn default() -> Self {
		OffensivePadding {
			center_to_caster: 2.,
			center_to_enemies: 2.,
			between_enemies: 2.,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum SkillPadding {
	Defensive(DefensivePadding),
	OffensiveSkill(OffensivePadding),
}

impl From<DefensivePadding> for SkillPadding {
	fn from(padding: DefensivePadding) -> Self {
		SkillPadding::Defensive(padding)
	}
}

impl From<OffensivePadding> for SkillPadding {
	fn from(padding: OffensivePadding) -> Self {
		SkillPadding::OffensiveSkill(padding)
	}
}

#[allow(unused)]
type Order = SaturatedU8;
#[allow(unused)]
type Size = Bound_u8<1, { u8::MAX }>;

// returns absolute values, characters on left side need to have their positions negated
fn calc_defensive_positions<'a>(padding: DefensivePadding,
                                caster: &'a ActionParticipant,
                                allies: impl Iterator<Item = &'a ActionParticipant>)
                                -> impl Iterator<Item = (&'a ActionParticipant, f64)> {
	let participants_by_position = {
		let mut temp =
			once(caster).chain(allies).collect::<Vec<_>>();
		
		temp.sort_by(|lhs, rhs| 
			lhs.pos_before.order.get().cmp(&rhs.pos_before.order.get()));
		
		temp.into_iter()
	};
	
	participants_by_position
		.scan(SaturatedU8::new(0), move |size_sum, participant| {
			let abs_pos_x = 
				(0..participant.pos_before.size.get())
					.fold(0., |sum, i| {
						let position = size_sum.get() + i;
						sum + padding.center_to_allies + (position as f64 * padding.between_allies)
					});
			
			*size_sum += participant.pos_before.size;
			
			Some((participant, abs_pos_x))
		})
}

// returns absolute values, characters on left side need to have their positions negated
fn calc_offensive_positions<'a>(padding: OffensivePadding,
                                caster: &'a ActionParticipant,
                                enemies: impl Iterator<Item = &'a ActionParticipant>)
                                -> impl Iterator<Item = (&'a ActionParticipant, f64)> {
	let enemies_by_position = {
		let mut temp =
			enemies.collect::<Vec<_>>();

		temp.sort_by(|lhs, rhs|
			lhs.pos_before.order.get().cmp(&rhs.pos_before.order.get()));

		temp.into_iter()
	};

	enemies_by_position
		.scan(SaturatedU8::new(0), move |size_sum, participant| {
			let abs_pos_x =
				(0..participant.pos_before.size.get())
					.fold(0., |sum, i| {
						let position = size_sum.get() + i;
						sum + padding.center_to_enemies + (position as f64 * padding.between_enemies)
					});

			*size_sum += participant.pos_before.size;

			Some((participant, abs_pos_x))
		})
		.chain(once((caster, padding.center_to_caster)))
}

pub fn do_anim_positions<'a>(
	padding: impl Into<SkillPadding>,
	caster: &'a ActionParticipant,
	others: impl Iterator<Item = &'a ActionParticipant>,
	duration: f64,
	pos_y: f64)
	-> HashMap<Uuid, TweenProperty_Vector2> {
	let padding = padding.into();
	
	let positions: Vec<_> =
		match padding {
			SkillPadding::Defensive(padding) =>
				calc_defensive_positions(padding, caster, others).collect(),
			SkillPadding::OffensiveSkill(padding) =>
				calc_offensive_positions(padding, caster, others).collect(),
		};
	
	positions
		.into_iter()
		.map(|(part, abs_pos_x)| {
			let pos_x =
				match part.pos_before.side {
					Side::Left => -abs_pos_x,
					Side::Right => abs_pos_x,
				};

			let target_pos = Vector2::new(pos_x as f32, pos_y as f32);
			let tween = 
				part.godot
					.node()
					.do_move(target_pos, duration);
			
			(part.godot.guid(), tween)
		}).collect()
}

pub fn do_default_positions(
	padding: StagePadding,
	characters: impl Iterator<Item = (CharacterNode, Position)>,
	duration: f64,
) -> HashMap<Uuid, TweenProperty_Vector2> {
	let default_positions = 
		calc_default_positions(padding, characters);
	
	default_positions
		.into_iter()
		.map(|(character, pos)| {
			(character.guid(), character.node().do_move(pos, duration))
		}).collect()
}