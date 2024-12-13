use super::*;

#[derive(
	strum_macros::FromRepr, strum_macros::EnumIter, Clone, Copy, Debug, Default, GodotConvert,
)]
#[godot(via = GString)]
#[repr(i32)]
pub enum Language {
	#[default]
	English = 0,
}

impl Language {
	pub(crate) fn display_name(&self) -> GString { self.to_godot() }

	pub(crate) fn index(&self) -> i32 { *self as i32 }
}
