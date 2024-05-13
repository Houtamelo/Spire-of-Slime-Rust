#[allow(unused_imports)]
use crate::prelude::*;

#[allow(unused)]
const MAX_ALPHA: f64 = 160. / 255.;

#[allow(unused)]
pub struct SpeedLines {
	owner: Ref<Node2D>,
	default_pos: Vector2,
}

impl SpeedLines {
	pub fn new(owner: TRef<Node2D>) -> Self {
		let owner_ref = unsafe { owner.assume_shared() };
		let default_pos = owner.position();
		
		SpeedLines {
			owner: owner_ref,
			default_pos,
		}
	}
	
	pub fn animate(&self, _fade_duration: f64, _stay_duration: f64, _speed: f64) -> Result<(TweenProperty_f64, TweenProperty_f64)> {
		todo!();
		/*
		self.owner
		    .map_if_sane(|owner| {
			    owner.set_position(self.default_pos);
			    
			    let fade_tween = owner.do_fade(MAX_ALPHA, fade_duration)?;

			    let total_duration = fade_duration * 2. + stay_duration;
			    let move_tween = 
				    owner.do_move_x(speed * total_duration, total_duration)
					     .map(|t| t.as_relative())?;
			    
			    let tween_ref =
				    owner.create_tween()
				         .ok_or_else(|| anyhow!("{}: Failed to create tween.", fn_name(&Self::animate)))?;

			    let tween = unsafe { tween_ref.assume_safe() };
			    tween.tween_property(self.owner, "modulate:a", MAX_ALPHA, fade_duration)
			         .ok_or_else(|| anyhow!("{}: Could not create `modulate:a` property tweener.", fn_name(&Self::animate)))?;
			    
			    let total_duration = fade_duration * 2. + stay_duration;
			    tween.parallel();

			    tween.tween_property(self.owner, "position:x", speed * total_duration, total_duration)
			         .ok_or_else(|| anyhow!("{}: Could not create `position:x` property tweener.", fn_name(&Self::animate)))
			         .map(|tweener| unsafe {
				         tweener.assume_safe().as_relative();
			         })?;

			    tween.tween_property(self.owner, "modulate:a", 0., fade_duration)
				     .ok_or_else(|| anyhow!("{}: Could not create `modulate:a` property tweener.", fn_name(&Self::animate)))?;
			    
			    return Ok(tween_ref);
		    })
		    .ok_or_else(|| anyhow!("{}: Owner is not sane.", fn_name(&Self::animate)))
		    .flatten()
		 */
	}
}