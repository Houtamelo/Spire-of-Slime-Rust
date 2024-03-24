use anyhow::{anyhow, Result};
use gdnative::api::*;
use gdnative::prelude::*;
use util::fn_name;
use util::prelude::CountOrMore;
use uuid::Uuid;

use crate::combat::entity::position::Position;
use crate::combat::entity_node::CharacterNode;
use crate::combat::skill_types::defensive::DefensiveSkill;
use crate::combat::skill_types::lewd::LewdSkill;
use crate::combat::skill_types::offensive::OffensiveSkill;

pub mod camera;
pub mod speed_lines;
pub mod splash_screen;
pub mod initial_position;
pub mod character_movement;

pub struct ActionParticipant {
	pub script: Instance<CharacterNode>,
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

const POP_DURATION: f64 = 0.1;

pub struct ActionTweens {
	camera: Ref<SceneTreeTween>,
	fade_hide_outsiders: Ref<SceneTreeTween>,
	fade_show_splash_screen: Ref<SceneTreeTween>,
	infinite_move_splash_screen: Ref<SceneTreeTween>,
}

pub struct AnimationNodes {
	combat_root: Ref<Node2D>,
	camera: Ref<Camera2D>,
	splash_screen: Ref<Node2D>,
	splash_screen_local_start_pos: Vector2,
	characters_container: Ref<Node2D>,
	in_action_container: Ref<Node2D>,
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
			.try_for_each(|p|
				unsafe { p.script.assume_safe() }
					.map(|script, _| {
						origin.remove_child(script.owner());
						destination.add_child(script.owner(), false);
					}).map_err(|err| anyhow!("switch_participants_parent(): {err}.")))
	}
	
	pub fn animate_action(&self, action: ActionToAnimate) -> Result<ActionTweens> {
		self.switch_participants_parent(action.participants())?;

		let zoom_in_camera = {
			let participants_height =
				action.participants()
				      .map(|p| unsafe {
					      p.script
					       .assume_safe()
					       .map(|s, _|
						       s.sprite_height())
				      }).try_collect::<Vec<_>>()?;

			let zoom =
				camera::height_based_zoom_in(participants_height.into_iter());

			camera::lerp_zoom(self.camera, zoom, POP_DURATION)?
		};

		let fade_hide_outsiders =
			unsafe { self.combat_root.assume_safe_if_sane() }
				.ok_or_else(|| anyhow!("{}(): combat_root is not sane.", fn_name(&Self::animate_action)))
				.and_then(|node| {
					node.create_tween()
					    .ok_or_else(|| anyhow!("{}(): Failed to create combat root tween.", fn_name(&Self::animate_action)))
					    .and_then(|tween| unsafe {
						    tween.assume_safe()
						         .tween_property(self.combat_root, "modulate:a", 0., POP_DURATION)
						         .ok_or_else(|| anyhow!("{}(): Could not create `modulate:a` property tweener on combat root.",
								     fn_name(&Self::animate_action)))?;

						    Ok(tween)
					    })
				})?;
		
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
		let fade_show_splash_screen =
			splash_screen::fade(self.splash_screen, POP_DURATION, 1.)?;

		const MOVE_SPEED: f64 = 50.0; // todo! test this value
		let infinite_move_splash_screen =
			splash_screen::animate_movement(self.splash_screen, self.splash_screen_local_start_pos, MOVE_SPEED)?;
		
		// todo! move characters to their initial positions
		
		return Err(anyhow!(""));
	}
}

