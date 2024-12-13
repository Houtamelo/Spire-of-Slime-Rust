use super::*;

const DISPLAY_SKILL_OVERLAY_MODE_AUTO: &str = "Auto";

const DISPLAY_SKILL_OVERLAY_MODE_WAIT: &str = "Wait for Input";

#[derive(Clone, Copy, Debug)]
pub enum SkillOverlayMode {
	Auto { delay_ms: i64 },
	WaitForInput,
}

impl GodotConvert for SkillOverlayMode {
	type Via = Dictionary;
}

impl FromGodot for SkillOverlayMode {
	fn try_from_godot(dict: Self::Via) -> Result<Self, ConvertError> {
		let is_auto = dict
			.get("is_auto")
			.ok_or_else(|| ConvertError::new("No key `is_auto` in dictionary"))?
			.try_to::<bool>()?;

		if is_auto {
			let delay_ms = dict
				.get("delay_ms")
				.ok_or_else(|| {
					ConvertError::new("No key `delay_ms` in dictionary but `is_auto` == true")
				})?
				.try_to::<i64>()?;

			Ok(SkillOverlayMode::Auto { delay_ms })
		} else {
			Ok(SkillOverlayMode::WaitForInput)
		}
	}
}

impl ToGodot for SkillOverlayMode {
	type ToVia<'v> = Self::Via;

	fn to_godot(&self) -> Self::Via {
		match self {
			SkillOverlayMode::Auto { delay_ms } => {
				dict! {
					"is_auto": true,
					"delay_ms": *delay_ms
				}
			}
			SkillOverlayMode::WaitForInput => {
				dict! {
					"is_auto": false
				}
			}
		}
	}
}

pub(crate) const ALL_OVERLAY_MODES: &[SkillOverlayMode] = &[
	SkillOverlayMode::Auto { delay_ms: 3000 },
	SkillOverlayMode::WaitForInput,
];

impl Default for SkillOverlayMode {
	fn default() -> Self { SkillOverlayMode::Auto { delay_ms: 3000 } }
}

// Pending Translation Hook
impl SkillOverlayMode {
	pub(crate) fn display_name(&self) -> &'static str {
		match self {
			SkillOverlayMode::Auto { .. } => DISPLAY_SKILL_OVERLAY_MODE_AUTO,
			SkillOverlayMode::WaitForInput => DISPLAY_SKILL_OVERLAY_MODE_WAIT,
		}
	}

	pub(crate) fn index(&self) -> i32 {
		match self {
			SkillOverlayMode::Auto { .. } => 0,
			SkillOverlayMode::WaitForInput => 1,
		}
	}
}
