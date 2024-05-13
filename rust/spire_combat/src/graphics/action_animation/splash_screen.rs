#[allow(unused_imports)]
use crate::prelude::*;

/// The returned tween only ends when killed manually.
pub fn animate_movement(node: Ref<Node2D>,
                        start_local_pos: Vector2,
                        speed: f64)
                        -> Result<TweenID<TweenProperty_f64>> {
	let tween_ref = unsafe {
		node.assume_safe_if_sane()
		    .ok_or_else(|| anyhow!("{}(): Splash screen is not sane.", fn_name(&animate_movement)))
		    .and_then(|tref| {
			        tref.set_position(start_local_pos);

			        tref.create_tween()
			            .ok_or_else(|| anyhow!("{}(): Failed to create tween.", fn_name(&animate_movement)))
		        })?
	};
	
	unsafe {
		let tween = tween_ref.assume_safe();
		tween.set_loops(0);
		tween.tween_property(node, "position:x", speed, 1.0)
		     .ok_or_else(|| anyhow!("{}(): Could not create `position:x` property tweener.", fn_name(&animate_movement)))
		     .map(|tweener| { 
			     tweener.assume_safe().as_relative();
		     })?;
	}

	node.do_move_x(speed, 1.0)
		.as_relative(0.)
		.register()
}