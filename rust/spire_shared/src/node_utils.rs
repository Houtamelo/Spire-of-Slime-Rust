#![allow(clippy::missing_safety_doc)]

use util::prelude::*;
use util_gdnative::prelude::*;

pub trait InspectNode {
	fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T));
}

impl<TSelf: SubClass<Node>> InspectNode for &TSelf {
	fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T)) {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .map(|node| f(&node))
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`",
				     owner.name(), node_path))
				 .log_if_err();
		}
	}
}

pub trait MapNode {
	fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap>;
}

impl<TSelf: SubClass<Node>> MapNode for &TSelf {
	fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .map(|node| f(&node))
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`",
				     owner.name(), node_path))
		}
	}
}

pub trait TryGetNode {
	fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>>;
}

impl<TSelf: SubClass<Node>> TryGetNode for &TSelf {
	fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>> {
		let owner: &Node = self.upcast();

		unsafe {
			owner.get_node_as::<T>(node_path)
			     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`",
				     owner.name(), node_path))
		}
	}
}

pub trait SpawnAs<'a> {
	fn spawn_as<T: SubClass<Node> + GodotObject<Memory = ManuallyManaged>>(self) -> Result<TRef<'a, T>>;
}

impl<'a, TSelf: SubClass<PackedScene>> SpawnAs<'a> for &'a TSelf {
	fn spawn_as<T: SubClass<Node> + GodotObject<Memory = ManuallyManaged>>(self) -> Result<TRef<'a, T>> {
		let scene: &PackedScene = self.upcast();
		
		scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED)
			 .ok_or_else(|| anyhow!("Failed to instance scene `{}`", scene.name()))
			 .and_then(|node| unsafe { 
				 node.assume_safe()
					 .cast()
				     .ok_or_else(|| anyhow!("Node is not of type `{}`", T::class_name()))
			 })
	}
}

pub trait SpawnAsInst<'a> {
	fn spawn_as_inst<T>(self) -> Result<TInstance<'a, T>> 
		where T: NativeClass,
			  T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged>;
}

impl<'a, TSelf: SubClass<PackedScene>> SpawnAsInst<'a> for &'a TSelf {
	fn spawn_as_inst<T>(self) -> Result<TInstance<'a, T>>
		where T: NativeClass,
			  T::Base: SubClass<Node> + GodotObject<Memory = ManuallyManaged> {
		self.spawn_as::<T::Base>()
			.and_then(|node| {
				node.cast_instance()
					.ok_or_else(|| anyhow!("Node is not an instance of type `{}`", type_name::<T>()))
			})
	}
}