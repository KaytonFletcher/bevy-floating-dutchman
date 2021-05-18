use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::{MassProperties, RigidBody, RigidBodyBuilder},
    geometry::{ColliderBuilder, InteractionGroups},
};

use crate::components::Weapon;

pub fn spawn_projectile(
    commands: &mut Commands,
    spawn_body: &RigidBody,
    weapon: &Weapon,
    ang_offset: f32,
) {
    let mut new_pb = weapon.projectile.clone();

    let rot = spawn_body.position().rotation.angle() - ang_offset;
    new_pb.motion.direction = Vec2::new(rot.cos(), rot.sin());

    let bruh = new_pb.motion.direction * weapon.pos_offset;

    let start_pos: Vec2 = Vec2::new(
        spawn_body.position().translation.x + bruh.x,
        spawn_body.position().translation.y + bruh.y,
    );

    new_pb.sprite.transform.translation.x = start_pos.x;
    new_pb.sprite.transform.translation.y = start_pos.y;

    let mut entity_builder = commands.spawn_bundle(new_pb);

    let proj_entity = entity_builder.id();

    entity_builder
        .insert(
            RigidBodyBuilder::new_dynamic()
                .lock_rotations()
                .translation(start_pos.x, start_pos.y)
                .user_data(proj_entity.to_bits() as u128),
        )
        .insert(
            ColliderBuilder::ball(2.0)
                .sensor(true)
                .mass_properties(MassProperties::from_ball(5.0, 2.0))
                .collision_groups(InteractionGroups::new(0x00010, 0x00001)),
        );
}
