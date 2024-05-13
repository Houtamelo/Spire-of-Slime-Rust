#[allow(unused_imports)]
use crate::*;
use serde::{Deserialize, Serialize};
use combat::state::CombatState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) enum SaveState {
	WorldMap,
	WorldMap_Event { event: String },
	LocalMap, //todo! add local map state
	LocalMap_Event { event: String }, //todo! add local map state
	Combat_LocalMap { combat: CombatState }, //todo! add current combat scene, add local map state
	Combat_WorldMap { combat: CombatState, scene_on_win: String, scene_on_loss: String }, //todo! add current combat scene
}