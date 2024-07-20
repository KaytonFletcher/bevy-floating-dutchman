use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct WeaponFired(pub Entity);

#[derive(Event)]
pub struct DealDamage {
    pub damage: f32,
    pub cause: Entity,
}

#[derive(Event)]
pub struct EntityDeath {
    pub cause: Entity,
}

#[derive(Event)]
pub struct PlayerDeath;

#[derive(Event)]
pub struct PlayerScored {
    pub score: i32,
}
