use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::WindowId,
};

use crate::components::Player;

const PLAYER_SCALE: f32 = 0.33;
const PLAYER_SPRITE_DIM: f32 = 549.;
const PLAYER_WIDTH: f32 = PLAYER_SPRITE_DIM - 350.;
const PLAYER_HEIGHT: f32 = PLAYER_SPRITE_DIM - 80.;
const PLAYER_SPEED: f32 = 16.;
const PLAYER_ACCEL: f32 = 2.;

pub fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
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
        .insert(Player::new());

    // world
    //     .create_entity()
    //     .with(prefab)
    //     .with(Boundary::new(
    //         0.,
    //         dimensions.width(),
    //         dimensions.height(),
    //         0.,
    //     ))
    //     .with(collider)
    //     .with(Player::new())
    //     .with(transform)
    //     // .with(Motion::new(PLAYER_SPEED, PLAYER_ACCEL))
    //     .build();
}
