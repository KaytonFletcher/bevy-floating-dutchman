use bevy::{prelude::*, window::WindowId};

use bevy_rapier2d::{
    na::Vector2,
    physics::RapierConfiguration,
    rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, InteractionGroups},
    },
};

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
    rapier_config.gravity = Vector2::<f32>::zeros();

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

    let mut collider_builder =
        ColliderBuilder::cuboid(collider_size_x / 2.0, collider_size_y / 2.0)
            .collision_groups(InteractionGroups::new(0b0010, 0b0001));

    collider_builder.is_sensor = true;

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
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
        .insert(collider_builder);
}
