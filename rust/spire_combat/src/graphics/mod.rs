use super::*;

mod action_animation;
mod entity_anim;
mod stages;
mod ui;

pub use action_animation::*;
pub use entity_anim::*;
pub use stages::*;
pub use ui::*;

#[allow(unused)]
#[derive(GodotClass)]
#[class(no_init, base = Node2D)]
pub struct CombatScene {
	animation_nodes: AnimationNodes,
	bg: Gd<Node2D>,
	bg_serial: SerializedBG,
	character_stats_left: ActorStatsUI,
	character_stats_right: ActorStatsUI,
	targeting_tooltip: Gd<TargetingTooltip>,
	speed_buttons: Gd<SpeedButtons>,
}

#[godot_api]
impl CombatScene {
	pub fn load(parent: &Node, stage: CombatBG, rng: &mut impl Rng) -> Result<Gd<Self>> {
		todo!()
		/*
		let scene = spawn_prefab_as::<Node2D>("res://Core/Combat/scene_combat.tscn")?;

		let (bg, bg_serial) = stage.spawn_randomized(scene, rng)?;
		let animation_nodes = scene.try_cast::<AnimationNodes>()?;

		let character_stats_left = {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/actors-stats/left-side")?;
			CharacterStatsUI::new(&node)?
		};

		character_stats_left.hide();

		let character_stats_right = {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/actors-stats/right-side")?;
			CharacterStatsUI::new(&node)?
		};

		character_stats_right.hide();

		let targeting_tooltip = {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/targeting_tooltip")?;
			TargetingTooltip::build_in(&node)?
		};

		targeting_tooltip.touch_assert_safe_mut(|tooltip, _| {
			tooltip.hide()
		});

		let speed_buttons = {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/speed-buttons")?;
			SpeedButtons::build_in(&node, SpeedSetting::UnPaused { speed: Speed::X1 })?
		};

		let _self = Self {
			animation_nodes,
			bg,
			bg_serial,
			character_stats_left,
			character_stats_right,
			targeting_tooltip,
			speed_buttons,
		}.emplace();

		let base = _self.base();

		for child in scene.get_children().iter().map(|node| node.to_object::<Node>().unwrap()) {
			scene.remove_child(child);
			base.add_child(child, false);
		}

		scene.queue_free();
		parent.add_child(unsafe { base.assume_shared() }, false);

		Ok(_self.into_shared())
		*/
	}
}
