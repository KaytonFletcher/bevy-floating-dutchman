use bevy::{asset::HandleId, prelude::*};

pub struct Weapon {
    fire_rate: f32,
    spread: f32,
    projectile: Projectile,
}

pub struct Projectile {
    texture_id: HandleId,
    damage: f32,
    time_to_live: f32,
}
