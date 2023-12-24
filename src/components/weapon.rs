use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Damage, Health};

#[derive(Component)]
pub struct Weapon {
    pub should_fire: bool,
    pub fire_rate: Timer,
    pub spread: f32,
    pub pos_offset: f32,
    pub projectile: ProjectileBundle,
}

impl Default for Weapon {
    fn default() -> Self {
        Weapon {
            should_fire: false,
            fire_rate: Timer::from_seconds(0.4, TimerMode::Once),
            spread: 0.0,
            pos_offset: 0.0,
            projectile: ProjectileBundle::default(),
        }
    }
}

#[derive(Clone, Component)]
pub struct Projectile {
    pub time_to_live: Timer,
    max_hits: i32,
    current_hits: i32,
}

impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            time_to_live: Timer::from_seconds(2.0, TimerMode::Once),
            max_hits: 1,
            current_hits: 0,
        }
    }
}

impl Projectile {
    /**
     * When a projectile collides we check to see if it has already
     */
    pub fn add_hit(&mut self) -> bool {
        if self.current_hits == self.max_hits {
            println!("TRIED TO GO FOR ANOTHER HIT");
            return false;
        } else {
            self.current_hits = self.current_hits + 1;
            return true;
        }
    }
}

#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub collision_group: CollisionGroups,
    //pub motion: Motion,
    pub health: Health,
    pub damage: Damage,

    pub sprite: SpriteBundle,
}

impl Default for ProjectileBundle {
    fn default() -> Self {
        ProjectileBundle {
            collision_group: CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
            health: Health::new(0.01),
            projectile: Projectile::default(),
            //motion: Motion::default(),
            damage: Damage::default(),
            sprite: SpriteBundle::default(),
        }
    }
}
