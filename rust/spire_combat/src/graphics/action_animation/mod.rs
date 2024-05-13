#[allow(unused_imports)]
use crate::prelude::*;

use std::iter;
use character_position::{do_anim_positions, do_default_positions};

use crate::graphics::action_animation::skills::offensive::{AttackResult, CounterResult, OffensiveAnim};
use crate::graphics::entity_anim::EntityAnim;
use crate::graphics::stages::CombatBG;

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
	pub caster: ActionParticipant,
	pub kind: ActionKind,
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

pub struct AnimationNodes {
	combat_root: Ref<Node2D>,
	camera: Ref<Camera2D>,
	stage: CombatBG,
	default_modulate: Ref<CanvasModulate>,
	characters_container: Ref<Node2D>,
	action_modulate: Ref<CanvasModulate>,
	action_container: Ref<Node2D>,
	splash_screen: Ref<Node2D>,
	splash_screen_local_start_pos: Vector2,
}

impl AnimationNodes {
	pub unsafe fn from_combat_root(root: &Node2D, stage: CombatBG) -> Result<AnimationNodes> {
		let combat_root =
			root.assume_shared();
		let camera = root
			.try_get_node::<Camera2D>("camera")?.assume_shared();
		let outside_modulate = root
			.try_get_node::<CanvasModulate>("canvas-layer_default/canvas-modulate")?.assume_shared();
		
		let (splash_screen, splash_screen_local_start_pos) = { 
			let node = root
				.try_get_node::<Node2D>("canvas-layer_skill-anim/splash-screen")?;
			let pos = node.position();
			(node.assume_shared(), pos)
		};
		
		let characters_container = root
			.try_get_node::<Node2D>("canvas-layer_default/characters")?.assume_shared();
		let in_action_container = root
			.try_get_node::<Node2D>("canvas-layer_skill-anim/characters")?.assume_shared();
		let inside_modulate = root
			.try_get_node::<CanvasModulate>("canvas-layer_skill-anim/canvas-modulate")?.assume_shared();
		
		Ok(Self {
			combat_root,
			camera,
			default_modulate: outside_modulate,
			splash_screen,
			splash_screen_local_start_pos,
			characters_container,
			action_container: in_action_container,
			action_modulate: inside_modulate,
			stage,
		})
	}
	
	fn switch_participants_parent(&self, participants: impl Iterator<Item = &ActionParticipant>) -> Result<()> {
		let origin =
			unsafe { self.characters_container.assume_safe_if_sane() }
				.ok_or_else(|| anyhow!("switch_participants_parent(): characters_container is not sane."))?;
		
		let destination =
			unsafe { self.action_container.assume_safe_if_sane() }
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

		seq.join(self.default_modulate.do_fade(0., POP_DURATION));
		seq.join(self.action_modulate.do_fade(1., POP_DURATION));
		
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
		sequence.join(self.default_modulate.do_fade(1., POP_DURATION));
		sequence.join(self.action_modulate.do_fade(0., POP_DURATION));
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
		
		seq.join_many(do_default_positions(self.stage.padding(), all_characters, POP_DURATION).into_iter().map(pluck!(.1)));
	}
	
	fn append_switch_to_idles(
		&self, 
		seq: &mut Sequence,
		participants: impl Iterator<Item = ActionParticipant>,
	) {
		let participants = participants.collect::<Vec<_>>();
		
		seq.append_call(move || {
			participants.iter().for_each(|part| {
				part.godot.name().to_idle_anim(part.godot);
			});
		});
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
		let enemy_nodes = enemies.iter()
			.map(|(part, result)| { 
				(part.godot, *result) 
			}).collect();

		let positions =
			do_anim_positions(padding, &caster, enemies.iter().map(pluck!(&.0)),
			                  POP_DURATION, ACTION_PARTICIPANTS_Y);

		seq.join_many(positions.into_iter().map(pluck!(.1)));

		seq.append_interval(POP_DURATION);

		let caster_movement = skill.caster_movement();
		seq.append(caster_movement.animate(&caster, STAY_DURATION));

		let enemies_movement = skill.enemies_movement();
		for (enemy, _) in enemies.iter() {
			seq.join(enemies_movement.animate(enemy, STAY_DURATION));
		}

		seq.append_sequence(skill.offensive_anim(caster.godot, enemy_nodes));
		
		seq.append_interval(0.01);
		
		let is_caster_dead = enemies.iter()
			.any(|(_, result)| { 
				matches!(result, AttackResult::Counter(_, CounterResult::Killed))
			});
		
		let alive_participants: Vec<ActionParticipant> = 
			if is_caster_dead {
				enemies.iter()
				       .filter_map(|(part, result)| {
					       if matches!(result, AttackResult::Killed) {
						       None
					       } else {
						       Some(*part)
					       }
				       }).collect()
			} else {
				iter::once(caster.clone())
					.chain(enemies.iter().filter_map(|(part, result)| {
						if matches!(result, AttackResult::Killed) {
							None
						} else {
							Some(*part)
						}
					})).collect()
			};

		self.join_characters_to_default_positions(seq, alive_participants.iter().cloned(), outsiders.into_iter());
		
		self.join_end_skill_anim(seq);
		
		if !is_caster_dead {
			seq.append_call(move || {
				skill.reset(caster.godot);
			});
		}

		self.append_switch_to_idles(seq, alive_participants.into_iter());
		
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
		
		let _positions = do_anim_positions(skill.padding(), &caster, allies.iter(),
		                                                       POP_DURATION, ACTION_PARTICIPANTS_Y);
		
		todo!()
	}
}

