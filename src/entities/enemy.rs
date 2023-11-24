use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Damage, EnemyBuilder, Follow, Motion, Player, ProjectileBundle, Track, Weapon},
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
    const ENEMY_SIZE: Vec2 = Vec2::new(12.0, 20.0);
    const START_POS: Vec2 = Vec2::new(300.0, 300.0);
    const ENEMY_SPEED: f32 = 700.0;
    const ENEMT_ACCEL: f32 = 500.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    let player_id = player_query.single();

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(player_id);

    let mut enemy_builder = commands.spawn_empty();

    enemy_builder
        .insert(tracker)
        .insert(Follow::new(player_id))
        .insert(Damage { amount: 0.5 })
        .insert(Damping {
            angular_damping: 1.0,
            linear_damping: 0.5,
        })
        .insert(
            EnemyBuilder::new(
                ENEMY_SIZE,
                ENEMY_SCALE,
                START_POS,
                sprites.follow_enemy.clone(),
            )
            .with_health(2.0)
            .with_motion(Motion {
                max_vel: ENEMY_SPEED,
                acceleration: ENEMT_ACCEL,
                ..Default::default()
            })
            .with_score(10)
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
    const ENEMY_SIZE: Vec2 = Vec2::new(32.0, 32.0);
    const START_POS: Vec2 = Vec2::new(800.0, 200.0);

    const ENEMY_SPEED: f32 = 250.0;
    const ENEMT_ACCEL: f32 = 100.0;
    const ENEMY_ROTATE_ACCEL: f32 = 300.0;

    let player_id = player_query.single();

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(player_id);

    let enemy_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                texture: sprites.enemy_bullet.clone(),
                ..Default::default()
            },
            collision_group: CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            ..Default::default()
        },
        pos_offset: 70.0,
        fire_rate: Timer::from_seconds(0.8, TimerMode::Once),
        ..Default::default()
    };

    let mut enemy_builder = commands.spawn_empty();

    enemy_builder
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
        .insert(enemy_weapon)
        .insert(
            EnemyBuilder::new(
                ENEMY_SIZE,
                ENEMY_SCALE,
                START_POS,
                sprites.shoot_enemy.clone(),
            )
            .with_health(2.0)
            .with_motion(Motion {
                max_vel: ENEMY_SPEED,
                acceleration: ENEMT_ACCEL,
                ..Default::default()
            })
            .with_score(20)
            .build(),
        );
}
