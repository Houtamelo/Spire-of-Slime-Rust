#[allow(unused_imports)]
use crate::*;
macro_rules! seek_tree_and_create_tween {
    ($owner: ident) => {{
	    let scene_tree_option = $owner.get_tree();
		let scene_tree = scene_tree_option.unwrap_manual();
		scene_tree.create_tween().unwrap()
    }};
}

pub(crate) use seek_tree_and_create_tween;
