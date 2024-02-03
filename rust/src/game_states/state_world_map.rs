#[derive(Debug)]
pub enum WorldMapState {
	Idle,
	SettingsMenu,
	CharacterMenu,
	Event(String),
	Combat { scene_on_win: String, scene_on_loss: String },
	LoadingLocalMap,
}