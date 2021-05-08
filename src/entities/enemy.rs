use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_rapier2d::{
    physics::RapierConfiguration,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

use crate::{
    components::{Follow, Motion, Track},
    resources::Game,
};

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
    const ENEMY_SPEED: f32 = 2.0;
    const ENEMY_ACCEL: f32 = 0.1;
    const ENEMY_ROTATE_SPEED: f32 = 3.0;

    let texture_handle = asset_server.load("sprites/green_fighter.png");

    let mut tracker = Track::new(ENEMY_ROTATE_SPEED, -PI / 2.0);
    tracker.with_entity(game.player.unwrap());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                ..Default::default()
            },
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .insert(Motion::new(ENEMY_SPEED, ENEMY_ACCEL))
        .insert(tracker)
        .insert(Follow::new(game.player.unwrap()))
        .insert(RigidBodyBuilder::new_dynamic().translation(20.0, 20.0))
        .insert(ColliderBuilder::cuboid(
            ENEMY_WIDTH / rapier_config.scale / 2.0,
            ENEMY_HEIGHT / rapier_config.scale / 2.0,
        ));
}
