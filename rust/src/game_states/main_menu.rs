#[derive(Debug)]
pub enum MainMenuState {
    Idle,
    LoadingSave { save_name: String },
    Settings,
}