use rand::Rng;
#[allow(unused_imports)]
use crate::*;
use crate::combat::graphics::action_animation::AnimationNodes;
use crate::combat::graphics::stages::CombatBG;
use crate::combat::graphics::stages::serialization::SerializedBG;

pub mod action_animation;
pub mod entity_anim;
pub mod stages;

#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constructor]
pub struct CombatScene {
	animation_nodes: AnimationNodes,
	bg: Ref<Node2D>,
	bg_serial: SerializedBG,
}

#[methods]
impl CombatScene {
	pub fn load(parent: &Node, stage: CombatBG, rng: &mut impl Rng) -> Result<Instance<Self>> {
		let scene = spawn_prefab_as::<Node2D>("res://Core/Combat/scene_combat.tscn")?;

		let (bg, bg_serial) = stage.spawn_randomized(&scene, rng)?;
		let animation_nodes = unsafe { AnimationNodes::from_combat_root(&scene, stage)? };
		
		let _self = Self {
			animation_nodes,
			bg,
			bg_serial,
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