use std::iter;
use character_position::do_default_positions;
#[allow(unused_imports)]
use crate::*;

use crate::combat::entity::position::Position;
use crate::combat::graphics::action_animation::skills::offensive::{AttackResult, CounterResult, OffensiveAnim};
use crate::combat::graphics::entity_anim::character_node::CharacterNode;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::graphics::entity_anim::EntityAnim;
use crate::combat::graphics::stages::CombatStage;

pub mod camera;
pub mod speed_lines;
pub mod splash_screen;
pub mod character_position;
pub mod character_movement;
pub mod skills;
pub mod test;

const ACTION_PARTICIPANTS_Y: f64 = 115.;
const POP_DURATION: f64 = 0.2;
const STAY_DURATION: f64 = 1.0;

#[derive(Debug, Copy, Clone)]
pub struct ActionParticipant {
	pub godot: CharacterNode,
	pub pos_before: Position,
	pub pos_after: Position,
}

#[derive(Debug, Copy, Clone)]
pub struct Outsider {
	pub godot: CharacterNode,
	pub pos_after: Position,
}

pub struct SkillAnimation {
	caster: ActionParticipant,
	kind: ActionKind,
}

pub enum ActionKind {
	OnSelf { skill: DefensiveSkill },
	OnAllies { skill: DefensiveSkill, allies: CountOrMore<1, ActionParticipant> },
	OnEnemies { skill: Box<dyn OffensiveAnim>, enemies: CountOrMore<1, (ActionParticipant, AttackResult)> },
	Lewd { skill: LewdSkill, enemies: CountOrMore<1, ActionParticipant> },
}

impl SkillAnimation {
	fn participants(&self) -> impl Iterator<Item = &ActionParticipant> {
		iter::from_coroutine(|| {
			yield &self.caster;
			
			match &self.kind {
				ActionKind::OnSelf { .. } => {},
				ActionKind::OnAllies { allies, .. } => {
					for _ally in allies.iter() {
						yield _ally;
					}
				},
				ActionKind::OnEnemies { enemies, .. } => {
					for (_enemy, _) in enemies.iter() {
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
	stage: CombatStage,
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
	
	pub fn animate_skill(&self, animation: SkillAnimation, outsiders: Vec<Outsider>) -> Result<Sequence> {
		self.switch_participants_parent(animation.participants())?;
		
		let mut seq = Sequence::new().bound_to(&self.combat_root);
		
		seq.append({
			let participants_height =
				animation.participants()
				         .map(|p| p.godot.name().required_height())
				         .collect::<Vec<_>>();

			let zoom_scale =
				camera::height_based_zoom_value(participants_height.into_iter());

			let end_zoom = Vector2::ONE * zoom_scale as f32;

			self.camera
			    .do_zoom(end_zoom, POP_DURATION)
		});

		seq.join(self.outside_modulate.do_fade(0., POP_DURATION));
		seq.join(self.inside_modulate.do_fade(1., POP_DURATION));
		
		match animation.kind {
			ActionKind::OnSelf { .. } => { todo!() },
			ActionKind::OnAllies { skill, allies } =>
				self.animate_allies_skill(&mut seq, animation.caster, skill, allies),
			ActionKind::OnEnemies { skill, enemies } => 
				self.animate_enemies_skill(&mut seq, animation.caster, skill, enemies, outsiders)?,
			ActionKind::Lewd { .. } => { todo!() },
		}
		
		Ok(seq)
	}

	fn join_end_skill_anim(&self, sequence: &mut Sequence) {
		sequence.join(self.camera.do_zoom(Vector2::ONE, POP_DURATION));
		sequence.join(self.outside_modulate.do_fade(1., POP_DURATION));
		sequence.join(self.inside_modulate.do_fade(0., POP_DURATION));
	}
	
	fn join_characters_to_default_positions(
		&self,
		seq: &mut Sequence,
		participants: impl Iterator<Item = ActionParticipant>,
		outsiders: impl Iterator<Item = Outsider>,
	) {
		let all_characters = 
			participants
				.map(|part| (part.godot, part.pos_after))
				.chain(outsiders.map(|out| (out.godot, out.pos_after)));
		
		for (_, tween) in do_default_positions(self.stage.padding(), all_characters, POP_DURATION) {
			seq.join(tween);
		}
	}

	fn animate_enemies_skill(
		&self,
		seq: &mut Sequence,
		caster: ActionParticipant,
		skill: Box<dyn OffensiveAnim + 'static>,
		enemies: CountOrMore<1, (ActionParticipant, AttackResult)>,
		outsiders: Vec<Outsider>
	) -> Result<()> {
		const MOVE_SPEED: f64 = 50.0;
		let _infinite_move_splash_screen =
			splash_screen::animate_movement(self.splash_screen, self.splash_screen_local_start_pos, MOVE_SPEED)?;

		let padding = skill.padding();
		let enemy_nodes = enemies.iter().map(|(part, result)| (part.godot, *result)).collect();

		let positions =
			character_position::do_anim_positions(padding, &caster, enemies.iter().map(pluck!(&.0)),
			                                      POP_DURATION, ACTION_PARTICIPANTS_Y);

		for (_, tween) in positions {
			seq.join(tween);
		}

		seq.append_interval(POP_DURATION);

		let caster_movement = skill.caster_movement();
		seq.append(caster_movement.animate(&caster, STAY_DURATION));

		let enemies_movement = skill.enemies_movement();
		for (enemy, _) in enemies.iter() {
			seq.join(enemies_movement.animate(enemy, STAY_DURATION));
		}

		seq.append_sequence(skill.offensive_anim(caster.godot, enemy_nodes));
		
		let is_caster_dead = enemies.iter().any(|(_, result)| matches!(result, AttackResult::Counter(_, CounterResult::Killed)));
		if is_caster_dead {
			let participants_to_move =
				enemies.into_iter()
				       .filter_map(|(part, result)| {
					       if matches!(result, AttackResult::Killed) {
						       None
					       } else {
						       Some(part)
					       }
				       });
			
			self.join_characters_to_default_positions(seq, participants_to_move, outsiders.into_iter());
		} else {
			let participants_to_move =
				iter::once(caster)
					.chain(enemies.into_iter().filter_map(|(part, result)| {
						if matches!(result, AttackResult::Killed) {
							None
						} else {
							Some(part)
						}
					}));
			
			self.join_characters_to_default_positions(seq, participants_to_move, outsiders.into_iter());
		}
		
		self.join_end_skill_anim(seq);
		
		Ok(())
	}
	
	fn animate_allies_skill(&self,
	                        _seq: &mut Sequence,
	                        caster: ActionParticipant,
	                        skill: DefensiveSkill,
	                        allies: CountOrMore<1, ActionParticipant>) {
		const MOVE_SPEED: f64 = 50.0; // todo! test this value
		let _infinite_move_splash_screen =
			splash_screen::animate_movement(self.splash_screen, self.splash_screen_local_start_pos, MOVE_SPEED);
		
		let _positions = character_position::do_anim_positions(skill.padding(), &caster, allies.iter(),
		                                                       POP_DURATION, ACTION_PARTICIPANTS_Y);
		
		todo!()
	}
}

