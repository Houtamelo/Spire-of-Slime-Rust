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
#![warn(clippy::missing_const_for_fn)]
#![feature(step_trait)]
#![feature(let_chains)]
#![feature(const_type_name)]
#![feature(const_option)]
#![feature(hash_extract_if)]
#![feature(ascii_char)]
#![feature(variant_count)]
#![feature(result_flattening)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(iterator_try_collect)]
#![feature(exclusive_range_pattern)]
#![feature(inline_const_pat)]
#![feature(arbitrary_self_types)]

#[allow(unused_imports)]
pub(crate) use internal_prelude::*;

mod save;
mod game_manager;
mod start_screen;

mod panic_hook;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	panic_hook::init_panic_hook();
	
	handle.add_class::<start_screen::StartScreenController>();
	handle.add_class::<combat::graphics::CombatScene>();
	handle.add_class::<combat::graphics::ui::TargetingTooltip>();
	handle.add_class::<combat::graphics::ui::SpeedButtons>();
	handle.add_class::<combat::graphics::action_animation::test::AnimTester>();
	handle.add_class::<game_manager::GameManager>();
	handle.add_class::<shared::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenuController>();
	handle.add_class::<main_menu::LoadButton>();
	handle.add_class::<world_map::WorldMapController>();
	handle.add_class::<local_map::MapGeneratorUI>();
	handle.add_class::<local_map::BiomeDataResource>();
	handle.add_class::<settings_menu::SettingsMenuController>();
	handle.add_class::<PitchRandomizer>();
	handle.add_class::<PlayOnClickAndPitchRandomizer>();
	handle.add_class::<PlayOnHoverAndPitchRandomizer>();
	handle.add_class::<DisallowClickFocusOnParent>();
	handle.add_class::<AutoTextResize>();
	handle.add_class::<TweensController>();
}

godot_init!(init);

#[allow(unused_imports)]
mod internal_prelude {
	pub use comfy_bounded_ints::prelude::*;
	pub use gdnative_tweener::prelude::*;
	pub use util::prelude::*;
	pub use util_gdnative::prelude::*;
	pub use uuid::Uuid;
	pub use serde::{Deserialize, Serialize};
	pub use shared::num::*;
	pub use shared::rand_utils::*;
	pub use rand_xoshiro::Xoshiro256PlusPlus;
}