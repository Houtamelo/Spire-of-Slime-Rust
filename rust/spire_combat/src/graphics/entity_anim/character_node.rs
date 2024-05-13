#[allow(unused_imports)]
use crate::prelude::*;
use crate::graphics::entity_anim::EntityAnim;

#[derive(Debug, Clone, Copy)]
pub struct CharacterNode {
	node: Ref<Node2D>,
	guid: Uuid,
	name: CharacterVariant,
}

impl CharacterNode {
	pub fn new(node: Ref<Node2D>, name: CharacterVariant, guid: Uuid) -> Self {
		Self { node, guid, name }
	}
	
	pub fn node(&self) -> Ref<Node2D> { self.node }
	pub fn guid(&self) -> Uuid { self.guid }
	pub fn name(&self) -> CharacterVariant { self.name }

	pub fn spawn(parent: &Node, prefab: &PackedScene, name: CharacterVariant, guid: Uuid) -> Result<CharacterNode> {
		let node_ref = 
			prefab.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
				  .ok_or_else(|| anyhow!("Failed to instance prefab"))?;
		
		let node_t = unsafe {
			node_ref.assume_safe()
			        .cast::<Node2D>()
			        .ok_or_else(|| anyhow!("Prefab is not a Node2D"))?
		};
		
		let script = CharacterNode {
			node: unsafe { node_t.assume_shared() },
			guid,
			name,
		};
		
		parent.add_child(node_t, false);
		
		Ok(script)
	}
	
	pub fn load_spawn(parent: &Node, name: CharacterVariant, guid: Uuid) -> Result<CharacterNode> {
		let node_t = 
			spawn_prefab_as::<Node2D>(name.prefab_path())?;
		
		let script = CharacterNode {
			node: unsafe { node_t.assume_shared() },
			guid,
			name,
		};
		
		parent.add_child(node_t, false);
		Ok(script)
	}
}
