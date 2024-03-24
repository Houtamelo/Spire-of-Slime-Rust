use std::iter::once;

use anyhow::{anyhow, Result};
use gdnative::api::*;
use gdnative::prelude::*;
use util::fn_name;
use util_gdnative::prelude::IntoSharedArray;

const REF_HEIGHT: f64 = 3.6; 
const REF_ZOOM_OFFSET: f64 = 0.3;

pub fn height_based_zoom_in(participants_height: impl Iterator<Item = f64>) -> f64 {
	let max_height =
		participants_height
			.chain(once(REF_HEIGHT))
			.max_by(f64::total_cmp)
			.unwrap(); // SOUNDNESS: `chain(once(REF_HEIGHT))` two lines above ensures that there is at least one element.

	let inv_zoom = 1.0 + REF_ZOOM_OFFSET * (REF_HEIGHT / max_height);
	return 1.0 / inv_zoom;
}

pub fn lerp_zoom(camera: Ref<Camera2D>,
                 final_val: f64,
                 lerp_duration: f64)
                 -> Result<Ref<SceneTreeTween>> {
	let zoom_vec = Vector2::new(final_val as f32, final_val as f32);

	let tween =
		unsafe { camera.assume_safe_if_sane() }
			.ok_or_else(|| anyhow!("{}: Camera is not sane.", fn_name(&lerp_zoom)))?
			.create_tween()
			.ok_or_else(|| anyhow!("{}: Failed to create tween.", fn_name(&lerp_zoom)))?;

	unsafe { tween.assume_safe() }
		.tween_property(camera, "zoom", zoom_vec.to_shared_array(), lerp_duration)
		.ok_or_else(|| anyhow!("{}: Could not create `zoom` property tweener.", fn_name(&lerp_zoom)))?;
	
	return Ok(tween);
}