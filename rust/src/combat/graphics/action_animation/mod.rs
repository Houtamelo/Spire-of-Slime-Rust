#[allow(unused_imports)]
use crate::*;

use crate::combat::entity::position::Position;
use crate::combat::graphics::entity_anim::character_node::CharacterNode;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;

pub mod camera;
pub mod speed_lines;
pub mod splash_screen;
pub mod initial_position;
pub mod character_movement;
pub mod skills;
mod test;

const ACTION_PARTICIPANTS_Y: f64 = 115.;

pub struct ActionParticipant {
	pub godot: CharacterNode,
	pub guid: Uuid,
	pub pos: Position,
}

pub struct ActionToAnimate {
	caster: ActionParticipant,
	kind: ActionKind,
}

pub enum ActionKind {
	OnSelf { skill: DefensiveSkill },
	OnAllies { skill: DefensiveSkill, allies: CountOrMore<1, ActionParticipant> },
	OnEnemies { skill: OffensiveSkill, enemies: CountOrMore<1, ActionParticipant> },
	Lewd { skill: LewdSkill, enemies: CountOrMore<1, ActionParticipant> },
}

impl ActionToAnimate {
	fn participants(&self) -> impl Iterator<Item = &ActionParticipant> {
		std::iter::from_coroutine(|| {
			yield &self.caster;
			
			match &self.kind {
				ActionKind::OnSelf { .. } => {},
				ActionKind::OnAllies { allies, .. } => {
					for _ally in allies.iter() {
						yield _ally;
					}
				},
				ActionKind::OnEnemies { enemies, .. } => {
					for _enemy in enemies.iter() {
						yield _enemy;
					}
				},
				ActionKind::Lewd { enemies, .. } => {
					for _enemy in enemies.iter() {
						yield _enemy;
					}
				},
			}
		})
	}
}

const POP_DURATION: f64 = 0.2;

pub struct ActionTweens {
	camera: TweenID<TweenProperty_Vector2>,
	fade_hide_outsiders: TweenID<TweenProperty_f64>,
	fade_show_participants: TweenID<TweenProperty_f64>,
	infinite_move_splash_screen: TweenID<TweenProperty_f64>,
}

pub struct AnimationNodes {
	combat_root: Ref<Node2D>,
	outside_modulate: Ref<CanvasModulate>,
	camera: Ref<Camera2D>,
	splash_screen: Ref<Node2D>,
	splash_screen_local_start_pos: Vector2,
	characters_container: Ref<Node2D>,
	in_action_container: Ref<Node2D>,
	inside_modulate: Ref<CanvasModulate>,
}

impl AnimationNodes {
	fn switch_participants_parent(&self, participants: impl Iterator<Item = &ActionParticipant>) -> Result<()> {
		let origin =
			unsafe { self.characters_container.assume_safe_if_sane() }
				.ok_or_else(|| anyhow!("switch_participants_parent(): characters_container is not sane."))?;
		
		let destination =
			unsafe { self.in_action_container.assume_safe_if_sane() }
				.ok_or_else(|| anyhow!("switch_participants_parent(): in_action_container is not sane."))?;

		participants
			.into_iter()
			.try_for_each(|part| unsafe { 
				part.godot
				    .node()
					.assume_safe_if_sane()
					.map(|owner| {
						origin.remove_child(owner);
						destination.add_child(owner, false);
					}).ok_or_else(|| anyhow!("switch_participants_parent(): owner is not sane."))
			})
	}
	
	pub fn animate_action(&self, action: ActionToAnimate) -> Result<ActionTweens> {
		self.switch_participants_parent(action.participants())?;

		let _zoom_in_camera = {
			let participants_height =
				action.participants()
				      .map(|p| p.godot.sprite_height())
					  .collect::<Vec<_>>();

			let zoom_scale =
				camera::height_based_zoom_value(participants_height.into_iter());
			
			let end_zoom = Vector2::ONE * zoom_scale as f32;
			
			self.camera
				.do_zoom(end_zoom, POP_DURATION)
				.register()?
		};

		let _fade_hide_outsiders =
			self.outside_modulate
				.do_fade(0., POP_DURATION)
				.register()?;

		let _fade_show_participants =
			self.inside_modulate
			    .do_fade(1., POP_DURATION)
				.register()?;
		
		return match action.kind {
			ActionKind::OnSelf { .. } => { todo!() },
			ActionKind::OnAllies { skill, allies } =>
				self.animate_allies_skill(action.caster, skill, allies),
			ActionKind::OnEnemies { .. } => { todo!() },
			ActionKind::Lewd { .. } => { todo!() },
		};
	}

	fn animate_allies_skill(&self,
	                        caster: ActionParticipant,
	                        skill: DefensiveSkill,
	                        allies: CountOrMore<1, ActionParticipant>)
	                        -> Result<ActionTweens> {
		const MOVE_SPEED: f64 = 50.0; // todo! test this value
		let _infinite_move_splash_screen =
			splash_screen::animate_movement(self.splash_screen, self.splash_screen_local_start_pos, MOVE_SPEED)?;
		
		initial_position::do_positions(skill.padding(), &caster, allies.iter(), POP_DURATION, ACTION_PARTICIPANTS_Y)?;
		
		todo!()
	}
}

