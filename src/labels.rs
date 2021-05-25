use bevy::prelude::StageLabel;
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum CustomStages {
    Physics,
    Debug,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
  AssetLoading,
  InitialSpawn,
  Playing,
}

