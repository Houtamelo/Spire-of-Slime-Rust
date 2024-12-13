use super::*;

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
	pub tree:  SerializedBGTree,
}

fn deserialize_rng_mode(mode: SerializedRngMode, node: &mut Gd<Node2D>) -> Result<()> {
	match mode {
		SerializedRngMode::Switch { on } => {
			node.set_visible(on);
			Ok(())
		}
		SerializedRngMode::Props { mut chosens } => {
			node.get_children()
				.iter_shared()
				.filter_map(|child| child.try_cast::<Node2D>().ok())
				.for_each(|mut child| {
					let child_name = child.get_name().to_string();
					let visible = if let Some((idx, _)) = chosens
						.iter()
						.enumerate()
						.find(|(_, name)| *name == &child_name)
					{
						chosens.remove(idx);
						true
					} else {
						false
					};

					child.set_visible(visible);
				});

			if chosens.is_empty() {
				Ok(())
			} else {
				Err(anyhow!("Chosens not found in props mode: {:?}", chosens))
			}
		}
		SerializedRngMode::MutuallyExclusive { chosen } => {
			let option = node
				.get_children()
				.iter_shared()
				.filter_map(|child| child.try_cast::<Node2D>().ok())
				.find(|mut child| child.get_name().to_string() == chosen);

			if let Some(mut child) = option {
				child.set_visible(true);
				Ok(())
			} else {
				Err(anyhow!(
					"Chosen not found in mutually exclusive mode: {:?}",
					chosen
				))
			}
		}
	}
}

fn deserialize_tree(name: String, tree: SerializedBGTree, parent: &Node2D) -> Result<()> {
	let mut node = parent
		.try_get_node_as::<Node2D>(&name)
		.ok_or_else(|| anyhow!("Node not found in tree: {:?}", name))?;

	if let Some(rng_mode) = tree.rng_mode {
		deserialize_rng_mode(rng_mode, &mut node)?;
	}

	for (child_name, child_tree) in tree.randomized_children {
		deserialize_tree(child_name, child_tree, &node)?;
	}

	Ok(())
}

impl SerializedBG {
	pub fn deserialize(self, parent: &mut Gd<Node2D>) -> Result<Gd<Node2D>> {
		let bg = self.stage.spawn(parent)?;
		deserialize_tree(bg.get_name().to_string(), self.tree, &bg)?;
		Ok(bg)
	}
}
