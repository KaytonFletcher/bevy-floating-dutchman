use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct WeaponFired(pub Entity);

#[derive(Event)]
pub struct EntityKilled(pub Entity, pub Entity);

#[derive(Event)]
pub struct PlayerScored {
    pub player: Entity,
    pub score: i32,
}
