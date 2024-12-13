use super::*;

const SCREEN_WIDTH: f64 = 1920.;
const LEFT_EDGE: f64 = 0.;

fn add_positions(
	padding: StagePadding,
	width_mult: f64,
	current_x: &mut f64,
	computed_positions: &mut Vec<(ActorScreenData, Vector2)>,
	characters: impl Iterator<Item = ActorScreenData>,
) {
	for actor in characters {
		let required_width = actor.godot.ident().required_width() * width_mult;
		*current_x += required_width;
		computed_positions.push((
			actor,
			Vector2::new((*current_x - (required_width / 2.)) as f32, padding.entity_y() as f32),
		));
	}
}

fn compute_positions_with_mult(
	left_characters: &[ActorScreenData],
	right_characters: &[ActorScreenData],
	padding: StagePadding,
	width_mult: f64,
) -> (Vec<(ActorScreenData, Vector2)>, f64) {
	let mut current_x = LEFT_EDGE;

	let mut computed_positions = Vec::new();

	add_positions(
		padding,
		width_mult,
		&mut current_x,
		&mut computed_positions,
		left_characters.iter().rev().cloned(),
	);

	current_x += padding.center_to_left() + padding.center_to_right();

	add_positions(
		padding,
		width_mult,
		&mut current_x,
		&mut computed_positions,
		right_characters.iter().cloned(),
	);

	(computed_positions, current_x)
}

#[must_use]
pub fn calc_default_positions(
	padding: StagePadding,
	characters: impl Iterator<Item = ActorScreenData>,
) -> Vec<(ActorScreenData, Vector2)> {
	let (left_characters, right_characters) = {
		let (mut lefties, mut righties) =
			characters.partition::<Vec<_>, _>(|actor| actor.team.is_left());

		lefties.sort_unstable_by_key(|actor| actor.pos_after);
		righties.sort_unstable_by_key(|actor| actor.pos_after);

		(lefties, righties)
	};

	let (computed_positions, occupied_width) =
		compute_positions_with_mult(&left_characters, &right_characters, padding, 1.);

	let occupied_ratio = occupied_width / SCREEN_WIDTH;

	if occupied_ratio <= 1. {
		let remaining_width = SCREEN_WIDTH - occupied_width;
		let shift = (remaining_width / 2.) as f32;

		(computed_positions.into_iter().map(|(actor, pos)| {
			let shifted_pos = Vector2 {
				x: pos.x + shift,
				y: pos.y,
			};

			(actor, shifted_pos)
		}))
		.collect()
	} else {
		let downscale_mult = 1. / occupied_ratio;
		compute_positions_with_mult(&left_characters, &right_characters, padding, downscale_mult).0
	}
}
