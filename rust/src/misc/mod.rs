#[allow(unused_imports)]
use crate::*;

pub mod panel_are_you_sure;
mod int_conversions;

use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

pub type SaturatedU64 = Bound_u64<0, {u64::MAX}>;
pub type SaturatedI64 = Bound_i64<{i64::MIN}, {i64::MAX}>;
pub use int_conversions::*;
pub type PercentageU8 = Bound_u8<0, 100>;

pub fn any_cancel_input(event: &TRef<InputEvent>) -> bool {
	return (event.is_action("ui_cancel", false))
		|| (event.cast::<InputEventMouseButton>().is_some_and(|mouse_event| mouse_event.button_index() == 2))
		|| (event.cast::<InputEventKey>().is_some_and(|key_event| key_event.scancode() == 16777217)); // Escape key
}

pub fn is_confirm_input(event: Ref<InputEvent>) -> bool {
	let safe_event = unsafe { event.assume_safe() };
	
	if safe_event.cast::<InputEventMouseButton>().is_some_and(|mouse_event|
		mouse_event.is_pressed() && mouse_event.button_index() == GlobalConstants::BUTTON_LEFT)
	{
		return true;
	}
	
	if safe_event.is_action_pressed("ui_accept", false, true) {
		return true;
	}
	
	return false;
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackedTicks {
	pub remaining_ms: SaturatedU64,
	pub initial_ms: SaturatedU64,
}

impl TrackedTicks {
	pub fn from_milliseconds(milliseconds: SaturatedU64) -> TrackedTicks {
		return TrackedTicks {
			remaining_ms: milliseconds,
			initial_ms: milliseconds,
		};
	}
}

pub trait Base100ChanceGenerator {
	fn base100_chance(&mut self, chance: PercentageU8) -> bool;
}

impl Base100ChanceGenerator for Xoshiro256PlusPlus {
	fn base100_chance(&mut self, chance: Bound_u8<0, 100>) -> bool {
		return self.gen_ratio(chance.get() as u32, 100);
	}
}


#[derive(Debug, Copy, Clone)]
pub struct Percentage_0_100 {
	inner_value: f64,
}

bound_f64_impl!(Percentage_0_100, 0.0, 100.0);

pub fn load_prefab(path: &str) -> Result<Ref<PackedScene>> {
	ResourceLoader::godot_singleton()
		.load(path, "PackedScene", false)
		.map(|res| res.cast::<PackedScene>())
		.flatten()
		.ok_or_else(|| anyhow!("Failed to load prefab at path: {path}"))
}

pub fn spawn_prefab_as<T: GodotObject + SubClass<Node>>(path: &str) -> Result<TRef<T>> {
	let prefab_ref = load_prefab(path)?;
	let prefab = unsafe { prefab_ref.assume_safe() };

	unsafe {
		prefab.instance(0)
			  .ok_or_else(|| anyhow!("Failed to instance prefab at path: {path}"))?
			  .assume_safe()
			  .cast()
			  .ok_or_else(|| anyhow!("Failed to cast prefab instance to {}", type_name::<T>()))
	}
}

pub fn spawn_prefab_as_inst<T: NativeClass<Base: SubClass<Node>>>(path: &str) -> Result<TInstance<T>> {
	let node = spawn_prefab_as::<T::Base>(path)?;
	node.cast_instance()
	    .ok_or_else(|| anyhow!("Failed to cast prefab instance to {}", type_name::<T>()))
}

pub fn load_resource_as<T: GodotObject<Memory = RefCounted> + SubClass<Resource>>(path: &str) -> Result<Ref<T>> {
	ResourceLoader::godot_singleton()
		.load(path, T::class_name(), false)
		.ok_or_else(|| anyhow!("Failed to load resource at path: {path}"))
		.and_then(|res| {
			res.cast::<T>().ok_or_else(|| anyhow!("Failed to cast resource to {}", type_name::<T>()))
		})
}