use anyhow::{anyhow, Result};
use gdnative::api::*;
use gdnative::prelude::*;
use util::fn_name;

pub fn fade(node_ref: Ref<Node2D>,
            duration: f64,
            final_val: f64)
            -> Result<Ref<SceneTreeTween>> {
	unsafe { node_ref.assume_safe_if_sane() }
		.ok_or_else(|| anyhow!("{}(): Splash screen is not sane.", fn_name(&fade)))
		.and_then(|node| {
			node.create_tween()
			    .ok_or_else(|| anyhow!("{}(): Failed to create tween.", fn_name(&fade)))
			    .and_then(|tween_ref| unsafe {
				    tween_ref.assume_safe()
				             .tween_property(node_ref, "modulate:a", final_val, duration)
				             .ok_or_else(|| anyhow!("{}(): Could not create `modulate:a` property tweener.", fn_name(&fade)))?;

				    Ok(tween_ref)
			    })
		})
}

/// The returned tween only ends when killed manually.
pub fn animate_movement(node_ref: Ref<Node2D>,
                        start_local_pos: Vector2,
                        speed: f64)
                        -> Result<Ref<SceneTreeTween>> {
	unsafe { node_ref.assume_safe_if_sane() }
		.ok_or_else(|| anyhow!("{}(): Splash screen is not sane.", fn_name(&animate_movement)))
		.and_then(|node| {
			node.set_position(start_local_pos);

			node.create_tween()
			    .ok_or_else(|| anyhow!("{}(): Failed to create tween.", fn_name(&animate_movement)))
			    .and_then(|tween_ref| {
				    let tween = unsafe { tween_ref.assume_safe() };
				    tween.set_loops(0);
				    tween.tween_property(node_ref, "position:x", speed, 1.0)
				         .ok_or_else(|| anyhow!("{}(): Could not create `position:x` property tweener.", fn_name(&animate_movement)))
				         .map(|tweener| unsafe {
					         tweener.assume_safe().as_relative();
				         })?;

				    Ok(tween_ref)
			    })
		})
}