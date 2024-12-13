use super::*;

pub fn hide_idle_anim(root: &Gd<Node2D>) {
	root.try_get_node_as::<Node2D>("anims/idle")
		.map(|mut idle| idle.hide())
		.ok_or_else(|| {
			anyhow!("Node of type `Node2D` not found at path `{}/anims/idle`", root.get_path())
		})
		.log_if_err();
}

pub fn node_emit_particles(root: &Gd<Node2D>, node_path: &'static str) {
	root.try_get_node_as::<CpuParticles2D>(node_path)
		.map(|mut particles| particles.set_emitting(true))
		.ok_or_else(|| {
			anyhow!(
				"Node of type `CpuParticles2D` not found at path `{}/{}`",
				root.get_path(),
				node_path
			)
		})
		.log_if_err();
}

pub fn node_maybe_particles(root: &Gd<Node2D>, node_path: &'static str) {
	root.try_get_node_as::<CpuParticles2D>(node_path)
		.map(|mut particles| particles.set_emitting(true));
}

pub fn node_stop_emit_particles(root: &Gd<Node2D>, node_path: &'static str) {
	root.try_get_node_as::<CpuParticles2D>(node_path)
		.map(|mut particles| particles.set_emitting(false))
		.ok_or_else(|| {
			anyhow!(
				"Node of type `CpuParticles2D` not found at path `{}/{}`",
				root.get_path(),
				node_path
			)
		})
		.log_if_err();
}

pub fn node_fade_show(root: &Gd<Node2D>, node_path: &'static str, duration: f64) {
	root.try_get_node_as::<Node2D>(node_path)
		.map(|mut node| {
			node.show();

			node.do_fade(1., duration).bound_to(root).register();
		})
		.ok_or_else(|| {
			anyhow!("Node of type `Node2D` not found at path `{}/{}`", root.get_path(), node_path)
		})
		.log_if_err();
}

pub fn node_fade_hide(root: &Gd<Node2D>, node_path: &'static str, duration: f64) {
	root.try_get_node_as::<Node2D>(node_path)
		.map(|node| {
			node.do_fade(0., duration).bound_to(root).register();
		})
		.ok_or_else(|| {
			anyhow!("Node of type `Node2D` not found at path `{}/{}`", root.get_path(), node_path)
		})
		.log_if_err();
}

pub fn node_show(owner: &Gd<Node2D>, node_path: &'static str) {
	owner
		.try_get_node_as::<CanvasItem>(node_path)
		.map(|mut node| node.show())
		.ok_or_else(|| {
			anyhow!(
				"Node of type `CanvasItem` not found at path `{}/{}`",
				owner.get_path(),
				node_path
			)
		})
		.log_if_err();
}

pub fn node_hide(root: &Gd<Node2D>, node_path: &'static str) {
	root.try_get_node_as::<CanvasItem>(node_path)
		.map(|mut node| node.hide())
		.ok_or_else(|| {
			anyhow!(
				"Node of type `CanvasItem` not found at path `{}/{}`",
				root.get_path(),
				node_path
			)
		})
		.log_if_err();
}

pub fn node_play_sound(root: &Gd<Node2D>, node_path: &'static str) {
	root.try_get_node_as::<Node>(node_path)
		.map(|mut node| unsafe {
			node.call(fn_name(&PitchRandomizer::_play_custom), &[]);
		})
		.ok_or_else(|| anyhow!("Node not found at path `{}/{}`", root.get_path(), node_path))
		.log_if_err();
}
