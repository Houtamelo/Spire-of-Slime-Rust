use anyhow::{anyhow, Result};
use gdnative::api::*;
use gdnative::prelude::*;
use util::fn_name;
use util_gdnative::prelude::{GodotRefCountedSomeInspector, IntoSharedArray};

use crate::combat::entity::Entity;

const REF_HEIGHT: f64 = 3.6; 
const REF_ZOOM_OFFSET: f64 = 0.3;

fn calc_zoom_in(max_npc_height: f64) -> f64 {
	let inv_zoom = 1.0 + REF_ZOOM_OFFSET * (REF_HEIGHT / max_npc_height);
	return 1.0 / inv_zoom;
}

fn lerp_zoom(camera_ref: Ref<Camera2D>,
             scene_tree: TRef<SceneTree>,
             final_val: f64,
             lerp_duration: f64)
             -> Result<Ref<SceneTreeTween>> {
	let zoom_vec = Vector2::new(final_val as f32, final_val as f32);
	
	let tween_ref =
		scene_tree.create_tween()
		          .ok_or_else(|| anyhow!("{}: Failed to create tween.", fn_name(&lerp_zoom)))?;
	
	tween_ref.touch_assert_safe(|tween| {
		tween.tween_property(camera_ref, "zoom", zoom_vec.to_shared_array(), lerp_duration);
	});
	
	return Ok(tween_ref);
}

pub fn animate_zoom_in<'a, EntityIter>(camera_ref: Ref<Camera2D>,
                                       scene_tree: TRef<SceneTree>,
                                       participants: EntityIter,
                                       lerp_duration: f64)
                                       -> Result<Ref<SceneTreeTween>>
                                       where EntityIter: Iterator<Item = &'a Entity> {
	let max_height = 
		participants.map(|entity| entity.sprite_height())
					.max_by(f64::total_cmp)
					.unwrap_or(REF_HEIGHT);
	
	let zoom = calc_zoom_in(max_height);
	return lerp_zoom(camera_ref, scene_tree, zoom, lerp_duration);
}

pub fn animate_zoom_out(camera_ref: Ref<Camera2D>,
                        scene_tree: TRef<SceneTree>,
                        lerp_duration: f64)
                        -> Result<Ref<SceneTreeTween>> {
	return lerp_zoom(camera_ref, scene_tree, 1.0, lerp_duration);
}