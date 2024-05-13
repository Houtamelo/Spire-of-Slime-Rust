use crate::prelude::*;

pub fn hide_idle_anim(owner: &Node2D) {
	owner.inspect_node("anims/idle", |idle: &Node2D| idle.hide());
}

pub fn node_emit_particles(owner: &Node2D, node_path: &'static str) {
	owner.inspect_node(node_path, |particles: &Particles2D| {
		particles.set_emitting(true);
	});
}

pub fn node_maybe_emit_particles(owner: &Node2D, node_path: &'static str) {
	owner.try_get_node(node_path)
	     .map(|particles: TRef<Particles2D>| {
		     particles.set_emitting(true);
	     }).log_if_err();
}

pub fn node_stop_emit_particles(owner: &Node2D, node_path: &'static str) {
	owner.inspect_node(node_path, |particles: &Particles2D| {
		particles.set_emitting(false);
	});
}

pub fn node_fade_show(owner: &Node2D, node_path: &'static str, duration: f64) {
	owner.inspect_node(node_path, |node: &Node2D| {
		node.show();

		node.do_fade(1., duration)
		    .bound_to(owner)
		    .register()
		    .log_if_err();
	})
}

pub fn node_fade_hide(owner: &Node2D, node_path: &'static str, duration: f64) {
	owner.inspect_node(node_path, |node: &Node2D| {
		node.do_fade(0., duration)
		    .bound_to(owner)
		    .register()
		    .log_if_err();
	})
}

pub fn node_show(owner: &Node2D, node_path: &'static str) {
	owner.inspect_node(node_path, |node: &Node2D| {
		node.show();
	})
}

pub fn node_hide(owner: &Node2D, node_path: &'static str) {
	owner.inspect_node(node_path, |node: &Node2D| {
		node.hide();
	})
}

pub fn node_play_sound(owner: &Node2D, node_path: &'static str) {
	owner.inspect_node(node_path, |node: &Node| unsafe {
		node.call("_play_custom", &[]);
	})
}



