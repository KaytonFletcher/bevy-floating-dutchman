use bevy::prelude::{Component, Entity};

#[derive(Clone, Component)]
pub struct Score {
    pub score: i32,
}

#[derive(Clone, Component)]
pub struct Scorer {
    // Scorer used to mark an entity that can "score" on behalf of a player.
    // This can be applied to projectiles, the player itself, etc...
    pub player: Entity,
}
