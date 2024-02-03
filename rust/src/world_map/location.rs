use serde::{Deserialize, Serialize};
use crate::save::affairs::{Happened, RescuedByMistressTender};
use crate::save::file::SaveFile;

#[repr(u8)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorldLocation {
	Chapel = 0,
	Grove = 1,
	Forest = 2,
	Cave = 3,
}

impl WorldLocation {
	pub fn connections(&self, save: &SaveFile) -> &'static [WorldLocation] {
		return match self {
			WorldLocation::Chapel => {
				if Happened::Yes == save.affairs().get::<RescuedByMistressTender>() {
					&[WorldLocation::Grove]
				} else {
					&[]
				}
			}
			WorldLocation::Grove => { 
				if Happened::Yes == save.affairs().get::<RescuedByMistressTender>() {
					&[WorldLocation::Chapel]
				} else {
					&[]
				} 
			}
			WorldLocation::Forest => { &[] }
			WorldLocation::Cave => { &[] }
		}
	}
}
