use gdnative::prelude::Instance;
use crate::{main_menu, world_map};

pub(super) mod state_main_menu;
pub(super) mod state_world_map;

#[derive(Debug)]
pub(super) enum GameState {
    StartScreen,
    MainMenu(Instance<main_menu::MainMenuController>, state_main_menu::MainMenuState),
	WorldMap(Instance<world_map::WorldMapController>, state_world_map::WorldMapState),
}

impl Default for GameState {
    fn default() -> Self { return GameState::StartScreen; }
}