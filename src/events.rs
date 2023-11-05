use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct WeaponFired(pub Entity);

#[derive(Event)]
pub struct EntityKilled(pub Entity);

#[derive(Event)]
pub struct PlayerKilled(pub Entity);
