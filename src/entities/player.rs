use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::Motion,
    components::{Damage, Health, Player, ProjectileBundle, Track, Weapon},
    resources::SpriteAssets,
};

pub fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    const SCALE: f32 = 0.33;
    const SPRITE_DIM: f32 = 549.;
    const WIDTH: f32 = SPRITE_DIM - 350.;
    const HEIGHT: f32 = SPRITE_DIM - 80.;
    const SPEED: f32 = 900.0;
    const ACCEL: f32 = 600.0;
    const ROTATE_ACCEL: f32 = 10.0;
    const SPRITE_OFFSET: f32 = std::f32::consts::PI / 2.0;

    let player_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                texture: sprites.cannonball.clone(),
                ..Default::default()
            },
            // motion: Motion {
            //     acceleration: 200.0,
            //     max_vel: 40.0,
            //     ..Default::default()
            // },
            ..Default::default()
        },
        pos_offset: 100.0,
        ..Default::default()
    };

    let mut player_builder = commands.spawn_empty();

    player_builder
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(10.0, 10.0, 1.0),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            texture: sprites.player.clone(),
            ..Default::default()
        })
        .insert(Player { score: 0 })
        .insert(Health::new(4.0))
        .insert(Track::new(ROTATE_ACCEL, SPRITE_OFFSET))
        .insert(Damage { amount: 4.0 })
        .insert(Motion {
            acceleration: ACCEL,
            max_vel: SPEED,
            ..Default::default()
        })
        .insert(player_weapon)
        .insert(Collider::cuboid(WIDTH / 2.0, HEIGHT / 2.0))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2))
        .insert(ColliderMassProperties::default())
        .insert(ReadMassProperties::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce::default())
        .insert(Velocity::zero())
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.0,
        });
}
