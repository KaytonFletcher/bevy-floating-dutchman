use bevy::prelude::*;

use super::{Damage, Health, Motion};

pub struct Weapon {
    pub fire_rate: Timer,
    pub spread: f32,
    pub pos_offset: f32,
    pub projectile: ProjectileBundle,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            fire_rate: Timer::from_seconds(0.4, false),
            spread: 0.0,
            pos_offset: 0.0,
            projectile: ProjectileBundle::default(),
        }
    }
}

#[derive(Clone)]
pub struct Projectile {
    pub time_to_live: Timer,
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            time_to_live: Timer::from_seconds(2.0, false),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub motion: Motion,
    pub health: Health,
    pub damage: Damage,

    #[bundle]
    pub sprite: SpriteBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            health: Health::new(0.01),
            projectile: Projectile::default(),
            motion: Motion::default(),
            damage: Damage::default(),
            sprite: SpriteBundle::default(),
        }
    }
}
