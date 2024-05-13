#[allow(unused_imports)]
use crate::prelude::*;

use crate::graphics::action_animation::AnimationNodes;
use crate::graphics::action_animation::skills::anim_utils::TryGetNode;
use crate::graphics::stages::CombatBG;
use crate::graphics::stages::serialization::SerializedBG;
use crate::graphics::ui::{SpeedButtons, TargetingTooltip, CharacterStatsUI, SpeedSetting, Speed};

pub mod action_animation;
pub mod entity_anim;
pub mod stages;
pub mod ui;

#[allow(unused)]
#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constructor]
pub struct CombatScene {
	animation_nodes: AnimationNodes,
	bg: Ref<Node2D>,
	bg_serial: SerializedBG,
	character_stats_left: CharacterStatsUI,
	character_stats_right: CharacterStatsUI,
	targeting_tooltip: Instance<TargetingTooltip>,
	speed_buttons: Instance<SpeedButtons>,
}

#[methods]
impl CombatScene {
	pub fn load(parent: &Node, stage: CombatBG, rng: &mut impl Rng) -> Result<Instance<Self>> {
		let scene = spawn_prefab_as::<Node2D>("res://Core/Combat/scene_combat.tscn")?;

		let (bg, bg_serial) = stage.spawn_randomized(&scene, rng)?;
		let animation_nodes = unsafe { AnimationNodes::from_combat_root(&scene, stage)? };
		
		let character_stats_left = unsafe {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/character-stats/left-side")?;
			CharacterStatsUI::new(&node)?
		};
		
		character_stats_left.hide();
		
		let character_stats_right = unsafe {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/character-stats/right-side")?;
			CharacterStatsUI::new(&node)?
		};

		character_stats_right.hide();
		
		let targeting_tooltip = unsafe {
			let node = scene.try_get_node::<Control>("canvas-layer_default/ui/targeting_tooltip")?;
			TargetingTooltip::build_in(&node)?
		};
		
		targeting_tooltip.touch_assert_safe_mut(|tooltip, _| {
			tooltip.hide()
		});
		
		let speed_buttons = unsafe {
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
	}
}
