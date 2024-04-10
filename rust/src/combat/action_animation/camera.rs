use std::iter::once;

#[allow(unused_imports)]
use crate::*;

const REF_HEIGHT: f64 = 3.6; 
const REF_ZOOM_OFFSET: f64 = 0.3;

pub fn height_based_zoom_value(participants_height: impl Iterator<Item = f64>) -> f64 {
	let max_height =
		participants_height
			.chain(once(REF_HEIGHT))
			.max_by(f64::total_cmp)
			.unwrap(); // SOUNDNESS: `chain(once(REF_HEIGHT))` two lines above ensures that there is at least one element.

	let inv_zoom = 1.0 + REF_ZOOM_OFFSET * (REF_HEIGHT / max_height);
	return 1.0 / inv_zoom;
}