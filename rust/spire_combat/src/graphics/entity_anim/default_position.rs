use util::prelude::pluck;
use crate::graphics::stages::StagePadding;
use crate::prelude::*;
use super::EntityAnim;

const SCREEN_WIDTH: f64 = 1920.;
const LEFT_EDGE: f64 = 0.;

fn add_positions(
	padding: StagePadding,
	width_mult: f64,
	current_x: &mut f64,
	computed_positions: &mut Vec<(CharacterNode, Vector2)>,
	characters: impl Iterator<Item = CharacterNode>) {
	for character in characters {
		let required_width = character.name().required_width() * width_mult;
		*current_x += required_width;
		computed_positions.push((character, Vector2::new((*current_x - (required_width / 2.)) as f32, padding.entity_y() as f32)));
	}
}

fn compute_positions_with_mult(
	left_characters: &[(CharacterNode, Position)],
	right_characters: &[(CharacterNode, Position)],
	padding: StagePadding,
	width_mult: f64) -> (Vec<(CharacterNode, Vector2)>, f64) {
	let mut current_x = LEFT_EDGE;
	
	let mut computed_positions = Vec::new();

	add_positions(padding, width_mult, &mut current_x, &mut computed_positions,
	              left_characters.iter().rev().map(pluck!(.0)));

	current_x += padding.center_to_left() + padding.center_to_right();

	add_positions(padding, width_mult, &mut current_x, &mut computed_positions,
	              right_characters.iter().map(pluck!(.0)));
	
	(computed_positions, current_x)
}

#[must_use]
pub fn calc_default_positions(
	padding: StagePadding,
	characters: impl Iterator<Item = (CharacterNode, Position)>,
) -> Vec<(CharacterNode, Vector2)> {
	let (left_characters, right_characters) = {
		let (mut temp_lefties, mut temp_righties) =
			characters.partition::<Vec<_>, fn(&(CharacterNode, Position)) -> bool>(|(_, pos)| pos.side.is_left());
		
		temp_lefties.sort_unstable_by(|(_, pos_a), (_, pos_b)| {
			pos_a.order.cmp(&pos_b.order)
		});
		
		temp_righties.sort_unstable_by(|(_, pos_a), (_, pos_b)| {
			pos_a.order.cmp(&pos_b.order)
		});
		
		(temp_lefties, temp_righties)
	};
	
	let (computed_positions, occupied_width) = 
		compute_positions_with_mult(&left_characters, &right_characters, padding, 1.);
	
	let occupied_ratio = occupied_width / SCREEN_WIDTH;

	if occupied_ratio <= 1. {
		let remaining_width = SCREEN_WIDTH - occupied_width;
		let shift = (remaining_width / 2.) as f32;

		(computed_positions.into_iter().map(|(chara, pos)| {
			(chara, Vector2 { x: pos.x + shift, y: pos.y })
		})).collect()
	} else {
		let downscale_mult = 1. / occupied_ratio;
		compute_positions_with_mult(&left_characters, &right_characters, padding, downscale_mult).0
	}
}