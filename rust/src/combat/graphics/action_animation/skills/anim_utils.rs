use crate::*;

pub trait InspectNode {
	unsafe fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T));
}

impl<TSelf: SubClass<Node>> InspectNode for &TSelf {
	unsafe fn inspect_node<T: SubClass<Node>>(self, node_path: &'static str, f: impl FnOnce(&T)) {
		let owner: &Node = &self.upcast();

		owner.get_node_as::<T>(node_path)
		     .map(|node| f(&node))
		     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`", 
			     owner.name(), node_path)).log_if_err();
	}
}

pub trait MapNode {
	unsafe fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap>;
}

impl<TSelf: SubClass<Node>> MapNode for &TSelf {
	unsafe fn map_node<T: SubClass<Node>, TMap>(self, node_path: &'static str, f: impl FnOnce(&T) -> TMap) -> Result<TMap> {
		let owner: &Node = &self.upcast();

		owner.get_node_as::<T>(node_path)
		     .map(|node| f(&node))
		     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`", 
			     owner.name(), node_path))
	}
}

pub trait TryGetNode {
	unsafe fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>>;
}

impl<TSelf: SubClass<Node>> TryGetNode for &TSelf {
	unsafe fn try_get_node<T: SubClass<Node>>(self, node_path: &str) -> Result<TRef<T>> {
		let owner: &Node = &self.upcast();

		owner.get_node_as::<T>(node_path)
		     .ok_or_else(|| anyhow!("Node `{}` does not have child node at path `{}`", 
			     owner.name(), node_path))
	}
}

pub fn hide_idle_anim(owner: &Node2D) {
	unsafe { owner.inspect_node("anims/idle", |idle: &Node2D| idle.hide()) }; 
}

pub fn node_emit_particles(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.inspect_node(node_path, |particles: &Particles2D| {
			particles.set_emitting(true);
		});
	}
}

pub fn node_maybe_emit_particles(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.get_node_as(node_path)
			 .map(|particles: TRef<Particles2D>| { 
				 particles.set_emitting(true); 
			 });
	}
}

pub fn node_stop_emit_particles(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.inspect_node(node_path, |particles: &Particles2D| {
			particles.set_emitting(false);
		});
	}
}

pub fn node_fade_show(owner: &Node2D, node_path: &'static str, duration: f64) {
	unsafe {
		owner.inspect_node(node_path, |node: &Node2D| {
			node.show();
			
			node.do_fade(1., duration)
				.bound_to(owner)
			    .register()
				.log_if_err();
		})
	}
}

pub fn node_fade_hide(owner: &Node2D, node_path: &'static str, duration: f64) {
	unsafe {
		owner.inspect_node(node_path, |node: &Node2D| {
			node.do_fade(0., duration)
			    .bound_to(owner)
			    .register()
			    .log_if_err();
		})
	}
}

pub fn node_show(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.inspect_node(node_path, |node: &Node2D| {
			node.show();
		})
	}
}

pub fn node_hide(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.inspect_node(node_path, |node: &Node2D| {
			node.hide();
		})
	}
}

pub fn node_play_sound(owner: &Node2D, node_path: &'static str) {
	unsafe {
		owner.inspect_node(node_path, |node: &Node| {
			node.call("_play_custom", &[]);
		})
	}
}



