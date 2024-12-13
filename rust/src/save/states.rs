use combat::prelude::CombatState;

use crate::internal_prelude::*;

#[allow(non_camel_case_types)]
#[derive(Clone, Serialize, Deserialize)]
pub enum SaveState {
	WorldMap,
	WorldMap_Event {
		event: String,
	},
	LocalMap, //todo! add local map state
	LocalMap_Event {
		event: String,
	}, //todo! add local map state
	Combat_LocalMap {
		combat: CombatState,
	}, //todo! add current combat scene, add local map state
	Combat_WorldMap {
		combat: CombatState,
		scene_on_win: String,
		scene_on_loss: String,
	}, //todo! add current combat scene
}
