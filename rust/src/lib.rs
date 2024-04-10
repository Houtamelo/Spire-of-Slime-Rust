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

pub use audio::bus;
#[allow(unused_imports)]
pub(crate) use internal_prelude::*;
pub use world_map::WorldLocation;

mod audio;
mod combat;
mod misc;
mod main_menu;
mod settings_menu;
mod save;
mod world_map;
mod local_map;
mod game_manager;
mod start_screen;
pub mod gdnative_macros;

pub const MAX_CHARACTERS_PER_TEAM: u8 = 4;
pub static CONFIG_PATH: &str = "user://config.cfg";

fn init_panic_hook() {
	let old_hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |panic_info| {
		let loc_string;
		if let Some(location) = panic_info.location() {
			loc_string = format!("file '{}' at line {}", location.file(), location.line());
		} else {
			loc_string = own!("unknown location")
		}

		let error_message;
		if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
			error_message = format!("[RUST] {}: panic occurred: {:?}", loc_string, s);
		} else {
			error_message = format!("[RUST] {}: unknown panic occurred", loc_string);
		}
		godot_error!("{}", error_message);
		(*(old_hook.as_ref()))(panic_info);

		unsafe {
			if let Some(gd_panic_hook) = autoload::<Node>("rust_panic_hook") {
				gd_panic_hook.call("rust_panic_hook", &[GodotString::from_str(error_message).to_variant()]);
			}
		}
	}));
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
	init_panic_hook();
	handle.add_class::<start_screen::StartScreenController>();
	handle.add_class::<combat::ui::SpeedButtons>();
	handle.add_class::<combat::ui::UI_CharacterStats>();
	handle.add_class::<combat::ui::TargetingTooltip>();
	handle.add_class::<game_manager::GameManager>();
	handle.add_class::<misc::panel_are_you_sure::PanelAreYouSure>();
	handle.add_class::<main_menu::MainMenuController>();
	handle.add_class::<main_menu::LoadButton>();
	handle.add_class::<world_map::WorldMapController>();
	handle.add_class::<local_map::generation::generator_ui::MapGeneratorUI>();
	handle.add_class::<local_map::generation::generator_ui::BiomeDataResource>();
	handle.add_class::<PitchRandomizer>();
	handle.add_class::<PlayOnClickAndPitchRandomizer>();
	handle.add_class::<PlayOnHoverAndPitchRandomizer>();
	handle.add_class::<DisallowClickFocusOnParent>();
	handle.add_class::<AutoTextResize>();
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
	pub use crate::misc::{SaturatedI64, SaturatedU8, SaturatedU64, ToSaturatedU64, ToSaturatedI64, ToU8Percentage,
	                      Base100ChanceGenerator, TrackedTicks, PercentageU8, Percentage_0_100};

	pub trait Inherits<T: GodotObject> {
		unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where T: SubClass<Base>;
	}

	impl<TSelf, Origin, Inherited> Inherits<Inherited> for TSelf
		where TSelf: ChangeRef<Inner = Origin>,
		      Origin: GodotObject + SubClass<Inherited>,
		      Inherited: GodotObject {
		unsafe fn base<Base: GodotObject>(&self) -> Ref<Base> where Inherited: SubClass<Base> {
			self.change_ref::<Inherited>().change_ref::<Base>()
		}
	}

	pub(crate) trait ChangeRef {
		type Inner: GodotObject;

		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base>;
	}

	#[allow(unused_qualifications)]
	impl<T, Own> ChangeRef for Ref<T, Own>
		where T: GodotObject,
		      Own: gdnative::object::ownership::Ownership {
		type Inner = T;

		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where T: SubClass<Base> {
			Ref::<Base>::from_sys(std::ptr::NonNull::new_unchecked(self.as_ptr()))
		}
	}

	#[allow(unused_qualifications)]
	impl<T, Own> ChangeRef for TRef<'_, T, Own>
		where T: GodotObject,
		      Own: gdnative::object::ownership::Ownership {
		type Inner = T;

		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base> {
			self.upcast().assume_shared()
		}
	}

	impl<T: GodotObject> ChangeRef for &T {
		type Inner = T;

		unsafe fn change_ref<Base: GodotObject>(&self) -> Ref<Base> where Self::Inner: SubClass<Base> {
			self.upcast().assume_shared()
		}
	}

	#[allow(unused)]
	unsafe fn test(obj: &impl Inherits<Node>) {
		let obj_ref: Ref<Object> = obj.base();
	}

	#[allow(unused)]
	unsafe fn test_1(obj: &PathFollow2D, obj_ref: Ref<Node2D>, obj_tref: TRef<'_, Sprite>) {
		test(&obj);
		test(&obj_ref);
		test(&obj_tref);
	}

	#[allow(unused)]
	unsafe fn test_2(obj: &impl Inherits<Resource>) {
		let obj_ref: Ref<Object> = obj.base();
	}

	#[allow(unused)]
	unsafe fn test_3(obj: &PackedScene, obj_ref: Ref<Texture>, obj_tref: TRef<'_, DynamicFontData>) {
		test_2(&obj);
		test_2(&obj_ref);
		test_2(&obj_tref);
	}
}