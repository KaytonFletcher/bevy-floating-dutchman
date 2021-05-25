use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::{ColliderBuilder, InteractionGroups},
};

use crate::{
    components::{Damage, Follow, Health, Motion, Player, ProjectileBundle, Track, Weapon},
    resources::SpriteAssets,
};

pub fn spawn_follow_enemy(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Res<SpriteAssets>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 12.0 * ENEMY_SCALE;
    const ENEMY_HEIGHT: f32 = 20.0 * ENEMY_SCALE;
    const ENEMY_SPEED: f32 = 250.0;
    const ENEMT_ACCEL: f32 = 100.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    // let player_id = player_query.single().unwrap();
    println!("BRUHHHH");

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    // tracker.with_entity(player_id);

    let mut enemy_builder = commands.spawn();

    let enemy_entity = enemy_builder.id();

    enemy_builder
        .insert(Motion {
            max_vel: ENEMY_SPEED,
            acceleration: ENEMT_ACCEL,
            ..Default::default()
        })
        .insert(tracker)
        .insert(Health::new(4.0))
        // .insert(Follow::new(player_id))
        .insert(Damage { amount: 0.5 })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            material: materials.add(sprites.follow_enemy.clone().into()),
            ..Default::default()
        })
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(1.0)
                .angular_damping(6.0)
                .translation(300.0, 300.0)
                .user_data(enemy_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::cuboid(ENEMY_WIDTH / 2.0, ENEMY_HEIGHT / 2.0)
                .collision_groups(InteractionGroups::new(0x00001, 0x00110)),
        );
        println!("BRUHHHH2");

}

pub fn spawn_shoot_enemy(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sprites: Res<SpriteAssets>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 32.0 * ENEMY_SCALE;
    const ENEMY_HEIGHT: f32 = 32.0 * ENEMY_SCALE;
    const ENEMY_SPEED: f32 = 250.0;
    const ENEMT_ACCEL: f32 = 100.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    let player_id = player_query.single().unwrap();

    println!("spawn enemy");

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(player_id);

    let enemy_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                material: materials.add(sprites.enemy_bullet.clone().into()),
                ..Default::default()
            },
            motion: Motion {
                acceleration: 10000.0,
                max_vel: 300.0,
                ..Default::default()
            },
            ..Default::default()
        },
        pos_offset: 70.0,
        fire_rate: Timer::from_seconds(0.8, false),
        ..Default::default()
    };

    let mut enemy_builder = commands.spawn();

    let enemy_entity = enemy_builder.id();

    enemy_builder
        .insert(Motion {
            max_vel: ENEMY_SPEED,
            acceleration: ENEMT_ACCEL,
            ..Default::default()
        })
        .insert(tracker)
        .insert(Health::new(4.0))
        .insert(Follow {
            entity: player_id,
            space: Some(300.0),
        })
        .insert(Damage { amount: 0.5 })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            material: materials.add(sprites.shoot_enemy.clone().into()),
            ..Default::default()
        })
        .insert(enemy_weapon)
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(1.0)
                .angular_damping(6.0)
                .translation(-600.0, -300.0)
                .user_data(enemy_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::cuboid(ENEMY_WIDTH / 2.0, ENEMY_HEIGHT / 2.0)
                .collision_groups(InteractionGroups::new(0x00001, 0x00110)),
        );
}
