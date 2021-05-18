use bevy::prelude::*;

use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::{ColliderBuilder, InteractionGroups},
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
) {
    println!("spawn player");

    const SCALE: f32 = 0.33;
    const SPRITE_DIM: f32 = 549.;
    const WIDTH: f32 = (SPRITE_DIM - 350.) * SCALE;
    const HEIGHT: f32 = (SPRITE_DIM - 80.) * SCALE;
    const SPEED: f32 = 400.0;
    const ACCEL: f32 = 200.0;
    const ROTATE_ACCEL: f32 = 400.0;
    const SPRITE_OFFSET: f32 = std::f32::consts::PI / 2.0;

    let player_sprite = asset_server.load("sprites/pirate_ship.png");
    let bullet_sprite = asset_server.load("sprites/cannonball.png");

    let player_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                material: materials.add(bullet_sprite.into()),
                ..Default::default()
            },
            motion: Motion {
                acceleration: 10000.0,
                max_vel: 300.0,
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let mut player_builder = commands.spawn();

    let player_entity = player_builder.id();

    game.player = Some(player_entity);

    player_builder
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            material: materials.add(player_sprite.into()),
            ..Default::default()
        })
        .insert(Player {})
        .insert(Health::new(4.0))
        .insert(Track::new(ROTATE_ACCEL, SPRITE_OFFSET))
        .insert(Damage { amount: 4.0 })
        .insert(Motion {
            acceleration: ACCEL,
            max_vel: SPEED,
            ..Default::default()
        })
        .insert(player_weapon)
        .insert(
            RigidBodyBuilder::new_dynamic()
                .linear_damping(0.5)
                .angular_damping(5.0)
                .user_data(player_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::cuboid(WIDTH / 2.0, HEIGHT / 2.0)
                .collision_groups(InteractionGroups::new(0x00100, 0x00001)),
        );

    println!("player spawned");
}
