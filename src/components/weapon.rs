use bevy::prelude::*;

use super::Motion;

pub struct Weapon {
    pub fire_rate: Timer,
    pub spread: f32,
    pub projectile: ProjectileBundle,
}

#[derive(Clone)]
pub struct Projectile {
    pub time_to_live: Timer,
}

#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub motion: Motion,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl ProjectileBundle {
    pub fn new(texture: Handle<ColorMaterial>) -> Self {
        Self {
            projectile: Projectile {
                time_to_live: Timer::from_seconds(3.0, true),
            },
            motion: Motion::new(800.0, 800.0),
            sprite: SpriteBundle {
                material: texture,
                ..Default::default()
            },
        }
    }
}
