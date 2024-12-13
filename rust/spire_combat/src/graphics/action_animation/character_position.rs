use super::*;

#[derive(Debug, Copy, Clone)]
pub struct DefensivePadding {
	center_to_allies: f64,
	between_allies:   f64,
}

impl Default for DefensivePadding {
	fn default() -> Self {
		DefensivePadding {
			center_to_allies: 2.,
			between_allies:   2.,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct OffensivePadding {
	center_to_caster:  f64,
	center_to_enemies: f64,
	between_enemies:   f64,
}

impl Default for OffensivePadding {
	fn default() -> Self {
		OffensivePadding {
			center_to_caster:  2.,
			center_to_enemies: 2.,
			between_enemies:   2.,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum SkillPadding {
	Defensive(DefensivePadding),
	OffensiveSkill(OffensivePadding),
}

impl From<DefensivePadding> for SkillPadding {
	fn from(padding: DefensivePadding) -> Self { SkillPadding::Defensive(padding) }
}

impl From<OffensivePadding> for SkillPadding {
	fn from(padding: OffensivePadding) -> Self { SkillPadding::OffensiveSkill(padding) }
}

#[allow(unused)]
type Order = Int;

// returns absolute values, characters on left side need to have their positions negated
fn calc_defensive_positions<'a>(
	padding: DefensivePadding,
	caster: &'a ActorScreenData,
	allies: impl Iterator<Item = &'a ActorScreenData>,
) -> impl Iterator<Item = (&'a ActorScreenData, f64)> {
	let participants_by_position = {
		let mut temp = iter::once(caster).chain(allies).collect::<Vec<_>>();

		temp.sort_by(|lhs, rhs| lhs.pos_before.cmp(&rhs.pos_before));

		temp.into_iter()
	};

	participants_by_position.scan(int!(0), move |size_sum, participant| {
		let size = participant.godot.ident().position_size();

		let abs_pos_x = (0..*size).fold(0., |sum, i| {
			let position = *size_sum + i;
			sum + padding.center_to_allies + (position as f64 * padding.between_allies)
		});

		*size_sum += size;

		Some((participant, abs_pos_x))
	})
}

// returns absolute values, characters on left side need to have their positions negated
fn calc_offensive_positions<'a>(
	padding: OffensivePadding,
	caster: &'a ActorScreenData,
	enemies: impl Iterator<Item = &'a ActorScreenData>,
) -> impl Iterator<Item = (&'a ActorScreenData, f64)> {
	let enemies_by_position = {
		let mut temp = enemies.collect::<Vec<_>>();

		temp.sort_by(|lhs, rhs| lhs.pos_before.cmp(&rhs.pos_before));

		temp.into_iter()
	};

	enemies_by_position
		.scan(int!(0), move |size_sum, participant| {
			let size = participant.godot.ident().position_size();

			let abs_pos_x = (0..*size).fold(0., |sum, i| {
				let position = *size_sum + i;
				sum + padding.center_to_enemies + (position as f64 * padding.between_enemies)
			});

			*size_sum += size;

			Some((participant, abs_pos_x))
		})
		.chain(iter::once((caster, padding.center_to_caster)))
}

pub fn do_anim_positions<'a>(
	padding: impl Into<SkillPadding>,
	caster: &'a ActorScreenData,
	ctx: impl Iterator<Item = &'a ActorScreenData>,
	duration: f64,
	pos_y: f64,
) -> HashMap<Uuid, SpireTween<Property<Vector2>>> {
	let padding = padding.into();

	let positions: Vec<_> = match padding {
		SkillPadding::Defensive(padding) => {
			calc_defensive_positions(padding, caster, ctx).collect()
		}
		SkillPadding::OffensiveSkill(padding) => {
			calc_offensive_positions(padding, caster, ctx).collect()
		}
	};

	positions
		.into_iter()
		.map(|(part, abs_pos_x)| {
			let pos_x = match part.team {
				Team::Left => -abs_pos_x,
				Team::Right => abs_pos_x,
			};

			let target_pos = Vector2::new(pos_x as f32, pos_y as f32);
			(part.godot.guid(), part.godot.node().do_move(target_pos, duration))
		})
		.collect()
}

pub fn do_default_positions(
	padding: StagePadding,
	characters: impl Iterator<Item = ActorScreenData>,
	duration: f64,
) -> HashMap<Uuid, SpireTween<Property<Vector2>>> {
	let default_positions = calc_default_positions(padding, characters);

	default_positions
		.into_iter()
		.map(|(actor, pos)| (actor.godot.guid(), actor.godot.node().do_move(pos, duration)))
		.collect()
}
