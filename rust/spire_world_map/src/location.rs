#[allow(unused_imports)]
use crate::*;
use gdnative::prelude::{FromVariant, ToVariant};
use serde::{Deserialize, Serialize};
use strum_macros::VariantArray;

#[repr(u8)]
#[derive(Serialize, Deserialize)]
#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(ToVariant, FromVariant)]
#[derive(VariantArray)]
pub enum WorldLocation {
	Chapel = 0,
	Grove = 1,
	Forest = 2,
	Cave = 3,
}

impl WorldLocation {
	/*
	pub fn available_connections(&self, save: &SaveFile) -> &'static [WorldLocation] {
		return match self {
			WorldLocation::Chapel =>
				if Happened::Yes == save.affairs().get::<RescuedByMistressTender>() {
					&[WorldLocation::Grove]
				} else {
					&[]
				},
			WorldLocation::Grove =>
				if Happened::Yes == save.affairs().get::<RescuedByMistressTender>() {
					&[WorldLocation::Chapel]
				} else {
					&[]
				},
			WorldLocation::Forest => &[],
			WorldLocation::Cave => &[],
		};
	}
	
	pub fn is_unlocked(&self, save: &SaveFile) -> bool {
		return match self {
			WorldLocation::Chapel => true,
			WorldLocation::Grove =>
				if Happened::Yes == save.affairs().get::<RescuedByMistressTender>() {
					true
				} else {
					false
				},
			WorldLocation::Forest => false,
			WorldLocation::Cave => false,
		};
	}
	*/
}
