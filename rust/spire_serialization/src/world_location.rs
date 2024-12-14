use super::*;

#[repr(i64)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, GodotConvert)]
#[godot(via = i64)]
pub enum WorldLocation {
	Chapel = 0,
	Grove = 1,
	Forest = 2,
	Cave = 3,
}

impl WorldLocation {
	pub fn available_connections(&self, save: &SaveFile) -> &'static [WorldLocation] {
		match self {
			WorldLocation::Chapel => {
				if &Happened::Yes == save.affairs.get::<RescuedByMistressTender>() {
					&[WorldLocation::Grove]
				} else {
					&[]
				}
			}
			WorldLocation::Grove => {
				if &Happened::Yes == save.affairs.get::<RescuedByMistressTender>() {
					&[WorldLocation::Chapel]
				} else {
					&[]
				}
			}
			WorldLocation::Forest => &[],
			WorldLocation::Cave => &[],
		}
	}

	pub fn is_unlocked(&self, save: &SaveFile) -> bool {
		match self {
			WorldLocation::Chapel => true,
			WorldLocation::Grove => &Happened::Yes == save.affairs.get::<RescuedByMistressTender>(),
			WorldLocation::Forest => false,
			WorldLocation::Cave => false,
		}
	}
}
