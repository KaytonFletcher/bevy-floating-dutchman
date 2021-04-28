use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::WindowId,
};

use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

use crate::{components::Player, entities::Motion};

const PLAYER_SCALE: f32 = 0.33;
const PLAYER_SPRITE_DIM: f32 = 549.;
const PLAYER_WIDTH: f32 = PLAYER_SPRITE_DIM - 350.;
const PLAYER_HEIGHT: f32 = PLAYER_SPRITE_DIM - 80.;
const PLAYER_SPEED: f32 = 200.;
const PLAYER_ACCEL: f32 = 4.5;

pub fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {

    rapier_config.scale = 20.0;

    let window = windows.get(WindowId::default()).unwrap();

    let player_spawn = (window.width() * 0.5, window.height() * 0.5);

    print!("{:?}", player_spawn);

    // let mut collider = Collider::new(
    //     PLAYER_WIDTH * PLAYER_SCALE,
    //     PLAYER_HEIGHT * PLAYER_SCALE,
    //     player_spawn.0,
    //     player_spawn.1,
    // );
    // collider.hit_box.update_vertices();

    let texture_handle = asset_server.load("sprites/pirate_ship.png");

    let collider_size_x = PLAYER_WIDTH * PLAYER_SCALE / rapier_config.scale;
    let collider_size_y = PLAYER_HEIGHT * PLAYER_SCALE / rapier_config.scale;

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                ..Default::default()
            },
            material: materials.add(texture_handle.into()),
            sprite: Sprite::new(Vec2::new(PLAYER_SPRITE_DIM, PLAYER_SPRITE_DIM)),
            ..Default::default()
        })
        .insert(Player::new())
        .insert(Motion::new(PLAYER_SPEED, PLAYER_ACCEL))
        .insert(RigidBodyBuilder::new_dynamic())
        .insert(ColliderBuilder::cuboid(
            collider_size_x / 2.0,
            collider_size_y / 2.0,
        ));
}
