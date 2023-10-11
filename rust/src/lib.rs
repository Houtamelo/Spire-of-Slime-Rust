#![allow(dead_code)]
#![allow(nonstandard_style)]
#![allow(clippy::needless_return)]
#![allow(clippy::large_enum_variant)]
#![feature(step_trait)]
#![allow(clippy::len_zero)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::neg_multiply)]
#![allow(clippy::enum_variant_names)]


mod combat;
mod util;

extern crate bounded_integer;

pub const MAX_CHARACTERS_PER_TEAM: usize = 4;
pub const STANDARD_INTERVAL_MS: i64 = 1000;
pub const STANDARD_INTERVAL_S: i64 = STANDARD_INTERVAL_MS / 1000;
pub const CONVERT_STANDARD_INTERVAL_TO_UNITCOUNT: i64 = STANDARD_INTERVAL_MS / 1000;

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