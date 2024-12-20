#![allow(unused_imports)]

pub use character_stats::CharacterStatsUI;
pub use speed_buttons::{Speed, SpeedButtons, SpeedSetting};
pub use targeting_tooltip::TargetingTooltip;

#[allow(unused_imports)]
use crate::prelude::*;

mod character_stats;
mod speed_buttons;
mod targeting_tooltip;
mod skill_button;

macro_rules! get_ref_or_bail {
    ($root_node: expr, $path: literal, $node_ty: ty) => {
	    unsafe {
			$root_node.get_node_as::<$node_ty>($path)
					  .ok_or_else(|| anyhow::anyhow!("Failed to get {} from {}", $path, $root_node.name()))
					  .map(|tref| tref.assume_shared())
		}
    };
}

macro_rules! get_tref_or_bail {
    ($root_node: ident, $path: literal, $node_ty: ty) => {
	    unsafe {
			$root_node.get_node_as::<$node_ty>($path)
					  .ok_or_else(|| anyhow::anyhow!("Failed to get {} from {}", $path, $root_node.name()))
		}
    };
}

pub(crate) use {get_ref_or_bail, get_tref_or_bail};