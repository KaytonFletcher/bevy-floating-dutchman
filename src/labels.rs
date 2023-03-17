use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CustomSets {
    Physics,
    Player,
    Movement,
}

#[derive(Clone, Eq, PartialEq, Default, Debug, Hash, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    SpawnPlayer,
    SpawnEnemies,
    Playing,
}
