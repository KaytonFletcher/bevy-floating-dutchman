use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MainSet {
    GamePlay,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GamePlaySet {
    DespawnEntities,
    PlayerInput,
    Simulation, // Most game logic (queries modifying components)
    Physics,    // Apply forces using rapier based on simulation
    Collision,  // Finally detect collisions using rapier based on forces applied
}
