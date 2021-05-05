use bevy::prelude::*;

use bevy_rapier2d::{
    physics::RapierConfiguration,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

use crate::{components::Track, entities::Motion};

pub fn spawn_follow_enemy(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    const ENEMY_SPRITE_DIM: f32 = 549.;
    const ENEMY_WIDTH: f32 = ENEMY_SPRITE_DIM - 350.;
    const ENEMY_HEIGHT: f32 = ENEMY_SPRITE_DIM - 80.;
    const ENEMY_SPEED: f32 = 15.0;
    const ENEMY_ACCEL: f32 = 0.9;
    const ENEMY_ROTATE_SPEED: f32 = 60.0;

    let texture_handle = asset_server.load("sprites/green_fighter.png");

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(100.0, 100.0, 1.0),
                ..Default::default()
            },
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .insert(Motion::new(ENEMY_SPEED, ENEMY_ACCEL))
        .insert(Track::new(ENEMY_ROTATE_SPEED, 0.0))
        .insert(RigidBodyBuilder::new_dynamic())
        .insert(ColliderBuilder::cuboid(
            ENEMY_WIDTH / rapier_config.scale / 2.0,
            ENEMY_HEIGHT / rapier_config.scale / 2.0,
        ));
}
