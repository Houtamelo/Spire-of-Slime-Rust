use super::*;

#[derive(Debug, Clone)]
pub struct ActorNode {
	node: Gd<Node2D>,
	guid: Uuid,
	name: CharacterVariant,
}

impl ActorNode {
	pub fn new(node: Gd<Node2D>, name: CharacterVariant, guid: Uuid) -> Self {
		Self { node, guid, name }
	}

	pub fn node(&self) -> Gd<Node2D> { self.node.clone() }
	pub fn guid(&self) -> Uuid { self.guid }
	pub fn ident(&self) -> CharacterVariant { self.name }

	pub fn spawn(
		parent: &mut Gd<Node>,
		prefab: &PackedScene,
		name: CharacterVariant,
		guid: Uuid,
	) -> Result<ActorNode> {
		let node = prefab
			.instantiate()
			.ok_or_else(|| anyhow!("Failed to instance prefab"))?;

		let node = node.try_cast::<Node2D>().map_err(|node| {
			node.free();
			anyhow!("Prefab is not a Node2D")
		})?;

		parent.add_child(&node);

		let script = ActorNode { node, guid, name };

		Ok(script)
	}

	pub fn load_spawn(
		parent: &mut Gd<Node>,
		name: CharacterVariant,
		guid: Uuid,
	) -> Result<ActorNode> {
		let node = spawn_prefab_as::<Node2D>(name.prefab_path())?;
		parent.add_child(&node);

		let script = ActorNode { node, guid, name };

		Ok(script)
	}
}
