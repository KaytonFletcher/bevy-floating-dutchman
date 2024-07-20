use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Damage, Health, Motion, Player, ProjectileBundle, Scorer, Track, Weapon},
    damage::on_damage_taken,
    labels::CursorCoordinates,
    player::{
        score::add_scores_from_killed,
        ship::{determine_player_death, on_player_death},
    },
    resources::SpriteAssets,
};

pub fn spawn_player(
    mut commands: Commands,
    cursor_query: Query<Entity, With<CursorCoordinates>>,
    sprites: Res<SpriteAssets>,
) {
    const SCALE: f32 = 0.33;
    const SPRITE_DIM: f32 = 549.;
    const WIDTH: f32 = SPRITE_DIM - 350.;
    const HEIGHT: f32 = SPRITE_DIM - 80.;
    const SPEED: f32 = 900.0;
    const ACCEL: f32 = 600.0;
    const ROTATE_ACCEL: f32 = 10.0;
    const SPRITE_OFFSET: f32 = std::f32::consts::PI / 2.0;

    let player_weapon = Weapon {
        projectile: ProjectileBundle {
            sprite: SpriteBundle {
                texture: sprites.cannonball.clone(),
                ..Default::default()
            },
            ..Default::default()
        },
        pos_offset: 100.0,
        ..Default::default()
    };

    let cursor_coord_entity = cursor_query.single();

    let mut player_builder = commands.spawn_empty();

    let player_entity = player_builder.id();

    info!("Player Entity: {:?}", player_entity);

    player_builder
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(10.0, 10.0, 1.0),
                scale: Vec3::new(SCALE, SCALE, 1.),
                ..Default::default()
            },
            texture: sprites.player.clone(),
            ..Default::default()
        })
        .insert(Player { score: 0 })
        .insert(Health::new(4.0))
        .insert(Track::new(ROTATE_ACCEL, SPRITE_OFFSET).with_entity(cursor_coord_entity))
        .insert(Damage { amount: 4.0 })
        .insert(Scorer {
            player: player_entity,
        })
        .insert(Motion {
            acceleration: ACCEL,
            max_vel: SPEED,
            ..Default::default()
        })
        .insert(player_weapon)
        .insert(Collider::cuboid(WIDTH / 2.0, HEIGHT / 2.0))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_2))
        .insert(ColliderMassProperties::default())
        .insert(ReadMassProperties::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce::default())
        .insert(Velocity::zero())
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.0,
        })
        .observe(add_scores_from_killed)
        .observe(on_damage_taken)
        .observe(determine_player_death)
        .observe(on_player_death);
}
