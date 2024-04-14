#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CombatStage {
	BellPlantGrove,
	Forest,
	Cave,
}

#[derive(Debug, Copy, Clone)]
pub struct StagePadding {
	center_to_left: f64,
	center_to_right: f64,
	entity_y: f64,
}

fn default() -> StagePadding {
	StagePadding {
		center_to_left: 150.,
		center_to_right: 150.,
		entity_y: 120.,
	}
}

impl StagePadding {
	pub fn center_to_left(&self) -> f64 { self.center_to_left }
	pub fn center_to_right(&self) -> f64 { self.center_to_right }
	pub fn entity_y(&self) -> f64 { self.entity_y }
}

impl CombatStage {
	pub fn padding(&self) -> StagePadding {
		match self {
			CombatStage::BellPlantGrove => default(),
			CombatStage::Forest => default(),
			CombatStage::Cave => default(),
		}
	}
}