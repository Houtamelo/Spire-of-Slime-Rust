use gdnative::prelude::Instance;

pub(super) mod main_menu;

pub(super) enum GameState {
    StartScreen,
    MainMenu(Instance<crate::main_menu::MainMenu>, main_menu::MainMenuState),
}

impl Default for GameState {
    fn default() -> Self { return GameState::StartScreen; }
}