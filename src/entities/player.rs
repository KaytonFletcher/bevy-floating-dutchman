use bevy::prelude::*;

use bevy_rapier2d::{
    physics::RapierConfiguration,
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

use crate::{
    components::Motion,
    components::{Damage, Health, Player, ProjectileBundle, Track, Weapon},
    resources::Game,
};

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    rapier_config: ResMut<RapierConfiguration>,
) {
    const PLAYER_SCALE: f32 = 0.33;
    const PLAYER_SPRITE_DIM: f32 = 549.;
    const PLAYER_WIDTH: f32 = PLAYER_SPRITE_DIM - 350.;
    const PLAYER_HEIGHT: f32 = PLAYER_SPRITE_DIM - 80.;
    const PLAYER_SPEED: f32 = 400.0;
    const PLAYER_ACCEL: f32 = 50.0;
    const PLAYER_ROTATE_ACCEL: f32 = 5.0;
    const PLAYER_SPRITE_OFFSET: f32 = std::f32::consts::PI / 2.0;

    let texture_handle = asset_server.load("sprites/pirate_ship.png");
    let bullet_texture = asset_server.load("sprites/cannonball.png");

    let collider_size_x = PLAYER_WIDTH * PLAYER_SCALE / rapier_config.scale;
    let collider_size_y = PLAYER_HEIGHT * PLAYER_SCALE / rapier_config.scale;

    let mut player_builder = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
            ..Default::default()
        },
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let player_entity = player_builder.id();

    game.player = Some(player_entity);

    player_builder
        .insert(Player::new())
        .insert(Health::new(4.0))
        .insert(Damage { amount: 1.0 })
        .insert(Motion::new(PLAYER_SPEED, PLAYER_ACCEL))
        .insert(Track::new(PLAYER_ROTATE_ACCEL, PLAYER_SPRITE_OFFSET))
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(1.0)
                .angular_damping(2.0)
                .user_data(player_entity.to_bits() as u128),
        )
        .insert(ColliderBuilder::cuboid(
            collider_size_x / 2.0,
            collider_size_y / 2.0,
        ))
        .with_children(|parent| {
            parent.spawn().insert(Weapon {
                fire_rate: Timer::from_seconds(0.2, true),
                spread: 0.1,
                projectile: ProjectileBundle::new(materials.add(bullet_texture.into())),
            });
        });
}
