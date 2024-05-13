#[allow(unused_imports)]
use crate::prelude::*;
use crate::graphics::stages::CombatBG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedRngMode {
	Switch { on: bool },
	Props { chosens: Vec<String> },
	MutuallyExclusive { chosen: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedBGTree {
	pub rng_mode: Option<SerializedRngMode>,
	pub randomized_children: Vec<(String, SerializedBGTree)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedBG {
	pub stage: CombatBG,
	pub tree: SerializedBGTree,
}

fn deserialize_rng_mode(mode: SerializedRngMode, node: &Node2D) -> Result<()> {
	match mode {
		SerializedRngMode::Switch { on } => {
			node.set_visible(on);
		}
		SerializedRngMode::Props { chosens } => {
			node.get_children()
				.iter()
				.filter_map(|node| node.to_object::<Node2D>())
				.for_each(|child_ref| {
					let child = unsafe { child_ref.assume_safe() };
					let child_name = child.name().to_string();
					child.set_visible(chosens.contains(&child_name));
				});
		}
		SerializedRngMode::MutuallyExclusive { chosen } => {
			node.get_children()
				.iter()
				.filter_map(|node| node.to_object::<Node2D>())
				.find(|child_ref| {
					let child = unsafe { child_ref.assume_safe() };
					if child.name().to_string() == chosen {
						child.set_visible(true);
						true
					} else {
						false
					}
				});
		}
	}
	
	Ok(())
}

fn deserialize_tree(name: String, tree: SerializedBGTree, parent: &Node2D) -> Result<()> {
	let node = parent.try_get_node::<Node2D>(&name)?;
	
	if let Some(rng_mode) = tree.rng_mode {
		deserialize_rng_mode(rng_mode, &node)?;
	}
	
	for (child_name, child_tree) in tree.randomized_children {
		deserialize_tree(child_name, child_tree, &node)?;
	}
	
	Ok(())
}

impl SerializedBG {
	pub fn deserialize(self, parent: &Node2D) -> Result<Ref<Node2D>> {
		let bg_ref = self.stage.spawn(parent)?;
		let bg = unsafe { 
			bg_ref.assume_safe_if_sane()
				  .ok_or_else(|| anyhow!("Spawned bg is not sane."))?
		};
		
		deserialize_tree(bg.name().to_string(), self.tree, &bg)?;
		
		Ok(bg_ref)
	}
}