use super::*;

pub fn offensive_anim(
	caster: ActorNode,
	enemies: Vec<(ActorNode, OffensiveResult)>,
) -> SpireSequence {
	let mut sequence = SpireSequence::new().bound_to(&caster.node());

	sequence.append_call({
		let caster = caster.clone();
		move || {
			let node = &caster.node();
			hide_idle_anim(node);
			node_show(node, "anims/pierce");
			node_emit_particles(node, "anims/pierce/trail");
			node_fade_show(node, "anims/pierce/slash_sprite", 0.1);
		}
	});

	sequence.append_interval(0.1);

	sequence.append_call({
		let caster = caster.clone();
		move || {
			node_emit_particles(&caster.node(), "anims/pierce/slash_particles");
		}
	});

	sequence.append_interval(0.05);

	sequence.append_call({
		let caster = caster.clone();
		move || {
			let node = &caster.node();
			node_stop_emit_particles(node, "anims/pierce/trail");
			node_fade_hide(node, "anims/pierce/slash_sprite", 0.4);

			play_attackeds_anim(&caster, &enemies);
		}
	});

	sequence
}

fn reset(caster: ActorNode) {
	let node = &caster.node();
	node_hide(node, "anims/pierce");
	node_stop_emit_particles(node, "anims/pierce/trail");
	node_stop_emit_particles(node, "anims/pierce/slash_particles");
	node.map_node::<Node2D, _>("anims/pierce/slash_sprite", |slash| {
		slash.hide();
		slash.set_indexed("modulate:a", &0.0.to_variant());
	})
	.log_if_err();
}

fn padding() -> OffensivePadding { todo!() }

fn caster_movement() -> CharacterMovement { todo!() }

fn enemies_movement() -> CharacterMovement { todo!() }
