#[allow(unused_imports)]
use crate::prelude::*;
use serialization::SerializedBG;

pub mod randomizer;
pub mod serialization;
mod grove;
mod forest;
mod cave;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CombatBG {
	Grove,
	Forest,
	Cave,
}

impl CombatBG {
	pub fn padding(&self) -> StagePadding {
		match self {
			CombatBG::Grove => default(),
			CombatBG::Forest => default(),
			CombatBG::Cave => default(),
		}
	}
	
	fn path(&self) -> &'static str {
		match self {
			CombatBG::Grove => "res://Core/Combat/Backgrounds/Grove/grove.tscn",
			CombatBG::Forest => "res://Core/Combat/Backgrounds/Forest/forest.tscn",
			CombatBG::Cave => "res://Core/Combat/Backgrounds/Cave/cave.tscn",
		}
	}
	
	fn randomize(&self, rng: &mut impl Rng, parent: &Node2D, name: &str) -> Result<SerializedBG> {
		let bg_tree =
			match self {
				CombatBG::Grove => grove::GROVE_NODE,
				CombatBG::Forest => forest::FOREST_NODE,
				CombatBG::Cave => cave::CAVE_NODE,
			};

		let tree = unsafe { bg_tree.randomize_recursive(rng, name, parent)? };
		Ok(SerializedBG { stage: *self, tree })
	}
	
	fn spawn(&self, parent: &Node2D) -> Result<Ref<Node2D>> {
		let bg = spawn_prefab_as::<Node2D>(self.path())?;

		let bg_ref = unsafe { bg.assume_shared() };
		parent.add_child(bg, false);

		Ok(bg_ref)
	}
	
	pub fn spawn_randomized(&self, parent: &Node2D, rng: &mut impl Rng) -> Result<(Ref<Node2D>, SerializedBG)> {
		let bg = spawn_prefab_as::<Node2D>(self.path())?;
		
		let bg_ref = unsafe { bg.assume_shared() };
		parent.add_child(bg, false);
		
		let serial = 
			self.randomize(rng, parent, bg.name().to_string().as_str())?;
		
		Ok((bg_ref, serial))
	}
	
	pub fn deserialize(serial: SerializedBG, parent: &Node2D) -> Result<Ref<Node2D>> {
		serial.deserialize(parent)
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
