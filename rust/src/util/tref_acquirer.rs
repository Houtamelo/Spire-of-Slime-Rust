use gdnative::object::bounds::{AssumeSafeLifetime, LifetimeConstraint};
use gdnative::prelude::*;

pub fn assert_tref_if_sane<T>(input: &Option<Ref<T, Shared>>) -> Option<TRef<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	if let Some(obj_ref) = input {
		if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
			return Some(obj_tref);
		}
	}

	godot_error!("Failed to get object through sane assertion, input: {input:?}");
	return None;
}

pub fn assert_tref<T>(input: &Option<Ref<T, Shared>>) -> Option<TRef<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	if let Some(obj_ref) = input {
		return Some(unsafe { obj_ref.assume_safe() });
	}

	godot_error!("Failed to get object, input: {input:?}");
	return None;
}

pub fn try_tref_if_sane<T>(input: &Option<Ref<T, Shared>>) -> Option<TRef<T, Shared>> where T : GodotObject<Memory = ManuallyManaged> {
	if let Some(obj_ref) = input {
		if let Some(obj_tref) = unsafe { obj_ref.assume_safe_if_sane() } {
			return Some(obj_tref);
		}
	}

	return None;
}

pub fn try_tref<T>(input: &Option<Ref<T, Shared>>) -> Option<TRef<T, Shared>> where T : GodotObject<Memory = RefCounted> {
	if let Some(obj_ref) = input {
		return Some(unsafe { obj_ref.assume_safe() });
	}

	return None;
}

pub fn assert_tinstance<'a, 'r, T>(input: &'r Option<Instance<T>>) -> Option<TInstance<'a, T, Shared>>
                                                                where T : NativeClass,
                                                                      AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::Memory> {
	if let Some(obj_ref) = input {
		return Some(unsafe { obj_ref.assume_safe() });
	}

	godot_error!("Failed to get instance.");
	return None;
}