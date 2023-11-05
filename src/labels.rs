use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MainSet {
    MainMenu,
    GamePlay,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GamePlaySet {
    Input,
}

#[derive(States, Clone, Eq, PartialEq, Default, Debug, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    SpawnPlayer,
    SpawnEnemies,
    Playing,
}
