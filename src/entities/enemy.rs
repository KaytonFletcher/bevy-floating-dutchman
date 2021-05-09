use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_rapier2d::{
    physics::RapierConfiguration,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

use crate::{components::{Damage, Follow, Motion, Track}, resources::Game};

pub fn spawn_follow_enemy(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<Game>,
    asset_server: Res<AssetServer>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    const ENEMY_SCALE: f32 = 2.5;
    const ENEMY_WIDTH: f32 = 12.0 * ENEMY_SCALE;
    const ENEMY_HEIGHT: f32 = 20.0 * ENEMY_SCALE;
    const ENEMY_SPEED: f32 = 80.0;
    const ENEMT_ACCEL: f32 = 20.0;
    const ENEMY_ROTATE_ACCEL: f32 = 3.0;

    let texture_handle = asset_server.load("sprites/green_fighter.png");

    let mut tracker = Track::new(ENEMY_ROTATE_ACCEL, -PI / 2.0);
    tracker.with_entity(game.player.unwrap());

    let mut enemy_builder = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
            ..Default::default()
        },
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let enemy_entity = enemy_builder.id();

    enemy_builder
        .insert(Motion::new(ENEMY_SPEED, ENEMT_ACCEL))
        .insert(tracker)
        .insert(Follow::new(game.player.unwrap()))
        .insert(Damage { amount: 1.0 } )
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(1.0)
                .angular_damping(6.0)
                .translation(20.0, 20.0)
                .user_data(enemy_entity.to_bits() as u128),
        )
        .insert(ColliderBuilder::cuboid(
            ENEMY_WIDTH / rapier_config.scale / 2.0,
            ENEMY_HEIGHT / rapier_config.scale / 2.0,
        ));
}
