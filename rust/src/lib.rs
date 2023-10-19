#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(clippy::needless_return)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::len_zero)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::never_loop)]
#![allow(clippy::clone_on_copy)]
#![feature(step_trait)]
#![feature(const_trait_impl)]

mod combat;
mod util;

pub const MAX_CHARACTERS_PER_TEAM: usize = 4;

pub use crate::util::bounded_isize::*;
pub use crate::util::bounded_u32::*;

use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	handle.add_class::<GameManager>();
}

godot_init!(init);

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameManager {
}

#[methods]
impl GameManager {
	fn new(_owner: &Node) -> Self {
		Self { }
	}
	
	#[method]
	fn time_planner_button_pressed(&mut self) {
		
	}
}