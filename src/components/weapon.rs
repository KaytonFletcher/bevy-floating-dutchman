use bevy::{asset::HandleId, prelude::*};

pub struct Weapon {
    pub fire_rate: Timer,
    pub spread: f32,
    pub projectile: ProjectileBundle,
}

pub struct Projectile {
    pub time_to_live: Timer,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    projectile: Projectile,

    #[bundle]
    sprite: SpriteBundle,
}

impl ProjectileBundle {
    pub fn new(texture: Handle<ColorMaterial>) -> Self {
        Self {
            projectile: Projectile {
                time_to_live: Timer::from_seconds(0.2, true),
            },
            sprite: SpriteBundle {
                material: texture,
                ..Default::default()
            },
        }
    }
}
