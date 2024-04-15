#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CombatStage {
	Grove,
	Forest,
	Cave,
}

impl CombatStage {
	pub fn padding(&self) -> StagePadding {
		match self {
			CombatStage::Grove => default(),
			CombatStage::Forest => default(),
			CombatStage::Cave => default(),
		}
	}
	
	pub fn load_background(&self, parent: &Node2D) -> Result<Ref<Node2D>> {
		let path =
			match self {
				CombatStage::Grove => "res://Core/Combat/Backgrounds/Grove/grove.tscn",
				CombatStage::Forest => "res://Core/Combat/Backgrounds/Forest/forest.tscn",
				CombatStage::Cave => "res://Core/Combat/Backgrounds/Cave/cave.tscn",
			};
		
		let node = spawn_prefab_as::<Node2D>(path)?;
		let bg_ref = unsafe { node.assume_shared() };
		parent.add_child(node, false);
		Ok(bg_ref)
	}
}

#[derive(Debug, Copy, Clone)]
pub struct StagePadding {
	center_to_left: f64,
	center_to_right: f64,
	entity_y: f64,
}

fn default() -> StagePadding {
	StagePadding {
		center_to_left: 150.,
		center_to_right: 150.,
		entity_y: 120.,
	}
}

impl StagePadding {
	pub fn center_to_left(&self) -> f64 { self.center_to_left }
	pub fn center_to_right(&self) -> f64 { self.center_to_right }
	pub fn entity_y(&self) -> f64 { self.entity_y }
}
