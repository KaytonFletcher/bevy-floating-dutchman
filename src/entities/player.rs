use bevy::{prelude::*, window::WindowId};

use bevy_rapier2d::{
    physics::RapierConfiguration,
    rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, InteractionGroups},
    },
};

use crate::{
    components::{Player, Track},
    entities::Motion,
};

const PLAYER_SCALE: f32 = 0.33;
const PLAYER_SPRITE_DIM: f32 = 549.;
const PLAYER_WIDTH: f32 = PLAYER_SPRITE_DIM - 350.;
const PLAYER_HEIGHT: f32 = PLAYER_SPRITE_DIM - 80.;
const PLAYER_SPEED: f32 = 3.0;
const PLAYER_ACCEL: f32 = 0.9;

pub fn init_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    let texture_handle = asset_server.load("sprites/pirate_ship.png");

    let collider_size_x = PLAYER_WIDTH * PLAYER_SCALE / rapier_config.scale;
    let collider_size_y = PLAYER_HEIGHT * PLAYER_SCALE / rapier_config.scale;

    let collider_builder = ColliderBuilder::cuboid(collider_size_x / 2.0, collider_size_y / 2.0);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                ..Default::default()
            },
            material: materials.add(texture_handle.into()),
            sprite: Sprite::new(Vec2::new(PLAYER_SPRITE_DIM, PLAYER_SPRITE_DIM)),
            ..Default::default()
        })
        .insert(Player::new())
        .insert(Motion::new(PLAYER_SPEED, PLAYER_ACCEL))
        .insert(Track::new())
        .insert(RigidBodyBuilder::new_dynamic())
        .insert(collider_builder);
}
