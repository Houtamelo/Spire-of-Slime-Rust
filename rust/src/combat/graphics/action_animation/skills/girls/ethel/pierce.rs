#[allow(unused_imports)]
use crate::*;
use crate::combat::entity::data::girls::ethel::skills::Pierce;
use crate::combat::graphics::action_animation::character_movement::CharacterMovement;
use crate::combat::graphics::action_animation::character_position::OffensivePadding;
use crate::combat::graphics::action_animation::skills::anim_utils::*;
use crate::combat::graphics::action_animation::skills::offensive::{AttackResult, OffensiveAnim, play_attackeds_anim};
use crate::combat::shared::*;

impl OffensiveAnim for Pierce {
	fn offensive_anim(&self, caster: CharacterNode, enemies: Vec<(CharacterNode, AttackResult)>) -> Sequence {
		let mut sequence = Sequence::new().bound_to(&caster.node());

		sequence.append_call(move || {
			caster.node().touch_assert_sane(|node| {
				hide_idle_anim(node);
				node_show(node, "anims/pierce");
				node_emit_particles(node, "anims/pierce/trail");
				node_fade_show(node, "anims/pierce/slash_sprite", 0.1);
			});
		});

		sequence.append_interval(0.1);

		sequence.append_call(move || {
			caster.node().touch_assert_sane(|node| {
				node_emit_particles(node, "anims/pierce/slash_particles");
			});
		});

		sequence.append_interval(0.05);

		sequence.append_call(move || {
			caster.node().touch_assert_sane(|node| {
				node_stop_emit_particles(node, "anims/pierce/trail");
				node_fade_hide(node, "anims/pierce/slash_sprite", 0.4);
			});
			
			play_attackeds_anim(caster, &enemies);
		});

		sequence
	}

	fn reset(&self, caster: CharacterNode) -> Result<()> {
		caster.node().touch_assert_sane(|node| unsafe {
			node_hide(node, "anims/pierce");
			node_stop_emit_particles(node, "anims/pierce/trail");
			node_stop_emit_particles(node, "anims/pierce/slash_particles");
			node.inspect_node("anims/pierce/slash_sprite", |slash: &Node2D| {
				slash.hide();
				slash.set_indexed("modulate:a", 0.);
			});
		});

		Ok(())
	}

	fn padding(&self) -> OffensivePadding {
		todo!()
	}

	fn caster_movement(&self) -> CharacterMovement {
		todo!()
	}

	fn enemies_movement(&self) -> CharacterMovement {
		todo!()
	}
}