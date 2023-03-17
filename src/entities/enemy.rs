use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        Damage, EnemyBuilder, EnemyBundle, Follow, Health, Motion, Player, ProjectileBundle, Track,
        Weapon,
    },
    labels::GameState,
    resources::SpriteAssets,
};

pub fn spawn_follow_enemy(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
    sprites: Res<SpriteAssets>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 12.0;
    const ENEMY_HEIGHT: f32 = 20.0;
    const ENEMY_SPEED: f32 = 700.0;
    const ENEMT_ACCEL: f32 = 500.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    let player_id = player_query.single();

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(player_id);

    let mut enemy_builder = commands.spawn_empty();

    enemy_builder
        .insert(Motion {
            max_vel: ENEMY_SPEED,
            acceleration: ENEMT_ACCEL,
            ..Default::default()
        })
        .insert(tracker)
        .insert(Follow::new(player_id))
        .insert(Damage { amount: 0.5 })
        .insert(Damping {
            angular_damping: 1.0,
            linear_damping: 0.5,
        })
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            texture: sprites.follow_enemy.clone(),
            ..Default::default()
        })
        .insert(
            EnemyBuilder::new(ENEMY_WIDTH, ENEMY_HEIGHT)
                .with_health(2.0)
                .build(),
        );

    game_state.set(GameState::Playing)
}

pub fn spawn_shoot_enemy(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    sprites: Res<SpriteAssets>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 32.0 * ENEMY_SCALE;
    const ENEMY_HEIGHT: f32 = 32.0 * ENEMY_SCALE;
    const ENEMY_SPEED: f32 = 250.0;
    const ENEMT_ACCEL: f32 = 100.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    let player_id = player_query.single();

    println!("spawn enemy");

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(player_id);

    let enemy_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                texture: sprites.enemy_bullet.clone(),
                ..Default::default()
            },
            // motion: Motion {
            //     acceleration: 10000.0,
            //     max_vel: 300.0,
            //     ..Default::default()
            // },
            ..Default::default()
        },
        pos_offset: 70.0,
        fire_rate: Timer::from_seconds(0.8, TimerMode::Once),
        ..Default::default()
    };

    let mut enemy_builder = commands.spawn_empty();

    enemy_builder
        .insert(Motion {
            max_vel: ENEMY_SPEED,
            acceleration: ENEMT_ACCEL,
            ..Default::default()
        })
        .insert(tracker)
        .insert(Follow {
            entity: player_id,
            space: Some(700.0),
        })
        .insert(Damping {
            angular_damping: 3.0,
            linear_damping: 1.0,
        })
        .insert(Damage { amount: 0.5 })
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(300.0, 300.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            texture: sprites.shoot_enemy.clone(),
            ..Default::default()
        })
        // .insert(enemy_weapon)
        .insert(
            EnemyBuilder::new(ENEMY_WIDTH, ENEMY_HEIGHT)
                .with_health(2.0)
                .build(),
        );
}
