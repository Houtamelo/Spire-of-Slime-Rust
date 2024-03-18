pub use speed_buttons::{Speed, SpeedButtons, SpeedSetting};

mod character_stats;
mod speed_buttons;

macro_rules! get_or_bail {
    ($root_node: ident, $path: literal, $node_ty: ty) => {
	    unsafe {
			$root_node.get_node_as::<$node_ty>($path)
				.ok_or_else(|| anyhow::anyhow!("Failed to {} from {}", $path, $root_node.name()))
				.map(|tref| tref.assume_shared())
		}
    };
}

pub(crate) use get_or_bail;