use super::*;

mod camera;
mod character_movement;
mod character_position;
mod skills;
mod speed_lines;
mod splash_screen;
mod test;

pub use character_movement::*;
pub use character_position::*;
pub use skills::*;
pub use speed_lines::*;

const ACTION_PARTICIPANTS_Y: f64 = 115.;
const POP_DURATION: f64 = 0.2;
const STAY_DURATION: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct ActorScreenData {
	pub godot: ActorNode,
	pub team: Team,
	pub pos_before: Position,
	pub pos_after: Position,
}

pub struct SkillAnimation {
	pub caster: ActorScreenData,
	pub kind:   ActionKind,
}

pub enum ActionKind {
	OnSelf {
		skill: DefensiveSkill,
	},
	OnAllies {
		skill:  DefensiveSkill,
		allies: CountOrMore<1, ActorScreenData>,
	},
	OnEnemies {
		skill:   Box<dyn OffensiveAnim>,
		enemies: CountOrMore<1, (ActorScreenData, OffensiveResult)>,
	},
	Lewd {
		skill:   LewdSkill,
		enemies: CountOrMore<1, ActorScreenData>,
	},
}

impl SkillAnimation {
	fn participants(&self) -> impl Iterator<Item = &ActorScreenData> {
		enumerator!({
			yield &self.caster;

			match &self.kind {
				ActionKind::OnSelf { .. } => {}
				ActionKind::OnAllies { allies, .. } => {
					for ally in allies.iter() {
						yield ally;
					}
				}
				ActionKind::OnEnemies { enemies, .. } => {
					for (enemy, _) in enemies.iter() {
						yield enemy;
					}
				}
				ActionKind::Lewd { enemies, .. } => {
					for enemy in enemies.iter() {
						yield enemy;
					}
				}
			}
		})
	}
}

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct AnimationNodes {
	base: Base<Node2D>,
	#[init(node = "../")]
	combat_root: OnReady<Gd<Node2D>>,
	#[init(node = "camera")]
	camera: OnReady<Gd<Camera2D>>,
	#[init(node = "canvas-layer_default/canvas-modulate")]
	default_modulate: OnReady<Gd<CanvasModulate>>,
	#[init(node = "canvas-layer_default/characters")]
	characters_container: OnReady<Gd<Node2D>>,
	#[init(node = "canvas-layer_skill-anim/canvas-modulate")]
	action_modulate: OnReady<Gd<CanvasModulate>>,
	#[init(node = "canvas-layer_skill-anim/characters")]
	action_container: OnReady<Gd<Node2D>>,
	#[init(node = "canvas-layer_skill-anim/splash-screen")]
	splash_screen: OnReady<Gd<Node2D>>,
	splash_screen_local_start_pos: Vector2,
	#[export]
	stage: CombatBG,
}

impl AnimationNodes {
	fn switch_participants_parent(&mut self, participants: impl Iterator<Item = &ActorScreenData>) {
		participants.into_iter().for_each(|part| {
			let node = &part.godot.node();
			self.characters_container.remove_child(node);
			self.action_container.add_child(node);
		});
	}

	pub fn animate_skill(
		&mut self,
		mut animation: SkillAnimation,
		outsiders: Vec<ActorScreenData>,
	) -> Result<SpireSequence> {
		self.switch_participants_parent(animation.participants());

		let mut seq = SpireSequence::new().bound_to(&*self.combat_root);

		seq.append({
			let participants_height = animation
				.participants()
				.map(|p| p.godot.ident().required_height())
				.collect::<Vec<_>>();

			let zoom_scale = camera::height_based_zoom_value(participants_height.into_iter());

			let end_zoom = Vector2::ONE * zoom_scale as f32;

			self.camera.do_zoom(end_zoom, POP_DURATION)
		});

		seq.join(self.default_modulate.do_fade(0., POP_DURATION));
		seq.join(self.action_modulate.do_fade(1., POP_DURATION));

		match animation.kind {
			ActionKind::OnSelf { .. } => {
				todo!()
			}
			ActionKind::OnAllies { skill, allies } => {
				self.animate_allies_skill(&mut seq, animation.caster, skill, allies)
			}
			ActionKind::OnEnemies { skill, enemies } => {
				self.animate_enemies_skill(
					&mut seq,
					&mut animation.caster,
					skill,
					enemies,
					outsiders,
				)?
			}
			ActionKind::Lewd { .. } => {
				todo!()
			}
		}

		Ok(seq)
	}

	fn join_end_skill_anim(&self, sequence: &mut SpireSequence) {
		sequence.join(self.camera.do_zoom(Vector2::ONE, POP_DURATION));
		sequence.join(self.default_modulate.do_fade(1., POP_DURATION));
		sequence.join(self.action_modulate.do_fade(0., POP_DURATION));
	}

	fn join_characters_to_default_positions(
		&self,
		seq: &mut SpireSequence,
		participants: impl Iterator<Item = ActorScreenData>,
		outsiders: impl Iterator<Item = ActorScreenData>,
	) {
		let all_characters = participants.chain(outsiders);

		do_default_positions(self.stage.padding(), all_characters, POP_DURATION)
			.into_iter()
			.for_each(|(_, tween)| {
				seq.join(tween);
			});
	}

	fn append_switch_to_idles(
		&self,
		seq: &mut SpireSequence,
		participants: impl Iterator<Item = ActorScreenData>,
	) {
		let participants = participants.collect::<Vec<_>>();

		seq.append_call(move || {
			participants.iter().for_each(|part| {
				part.godot.ident().to_idle_anim(part.godot.clone());
			});
		});
	}

	fn animate_enemies_skill(
		&mut self,
		seq: &mut SpireSequence,
		caster: &mut ActorScreenData,
		skill: Box<dyn OffensiveAnim + 'static>,
		mut enemies: CountOrMore<1, (ActorScreenData, OffensiveResult)>,
		outsiders: Vec<ActorScreenData>,
	) -> Result<()> {
		const MOVE_SPEED: f64 = 50.0;
		let _infinite_move_splash_screen = splash_screen::animate_movement(
			&mut self.splash_screen,
			self.splash_screen_local_start_pos,
			MOVE_SPEED,
		);

		let padding = skill.padding();
		let mut enemy_nodes = enemies
			.iter()
			.cloned()
			.map(|(part, result)| (part.godot, result))
			.collect::<Vec<_>>();

		let positions = do_anim_positions(
			padding,
			&caster,
			enemies.iter().map(pluck!(&.0)),
			POP_DURATION,
			ACTION_PARTICIPANTS_Y,
		);

		for tween in positions.into_iter().map(pluck!(.1)) {
			seq.join(tween);
		}

		seq.append_interval(POP_DURATION);

		let caster_movement = skill.caster_movement();
		seq.append(caster_movement.animate(caster, STAY_DURATION));

		let enemies_movement = skill.enemies_movement();
		for (enemy, _) in enemies.iter_mut() {
			seq.join(enemies_movement.animate(enemy, STAY_DURATION));
		}

		seq.append(skill.offensive_anim(&mut caster.godot, enemy_nodes.as_mut_slice()));

		seq.append_interval(0.01);

		let is_caster_dead = enemies.iter().any(|(_, result)| result.caster_died());

		let alive_participants = {
			let mut temp = Vec::new();

			if !is_caster_dead {
				temp.push(caster.clone());
			}

			temp.extend(enemies.iter().filter_map(|(target, result)| {
				if result.target_died() {
					None
				} else {
					Some(target.clone())
				}
			}));

			temp
		};

		self.join_characters_to_default_positions(
			seq,
			alive_participants.iter().cloned(),
			outsiders.into_iter(),
		);

		self.join_end_skill_anim(seq);

		if !is_caster_dead {
			seq.append_call({
				let mut caster = caster.clone();
				move || skill.reset(&mut caster.godot)
			});
		}

		self.append_switch_to_idles(seq, alive_participants.into_iter());

		Ok(())
	}

	fn animate_allies_skill(
		&mut self,
		_seq: &mut SpireSequence,
		caster: ActorScreenData,
		skill: DefensiveSkill,
		allies: CountOrMore<1, ActorScreenData>,
	) {
		const MOVE_SPEED: f64 = 50.0; // todo! test this value
		let _infinite_move_splash_screen = splash_screen::animate_movement(
			&mut self.splash_screen,
			self.splash_screen_local_start_pos,
			MOVE_SPEED,
		);

		let _positions = do_anim_positions(
			skill.padding(),
			&caster,
			allies.iter(),
			POP_DURATION,
			ACTION_PARTICIPANTS_Y,
		);

		todo!()
	}
}
