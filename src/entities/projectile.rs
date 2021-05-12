use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{physics::RapierConfiguration, rapier::{
    dynamics::{MassProperties, RigidBody, RigidBodyBuilder},
    geometry::ColliderBuilder,
}};

use crate::components::{Motion, ProjectileBundle};

pub fn spawn_projectile(
    commands: &mut Commands,
    spawn_body: &RigidBody,
    projectile_bundle: &ProjectileBundle,
    rapier_config: &Res<RapierConfiguration>,
) {
    let start_pos = spawn_body.position().translation;
    let mut new_pb = (*projectile_bundle).clone();

    let rot = spawn_body.position().rotation.angle() - (PI / 2.0);
    new_pb.motion.direction = Vec2::new(rot.cos(), rot.sin());
    new_pb.sprite.transform.translation.x = start_pos.x * rapier_config.scale;
    new_pb.sprite.transform.translation.y = start_pos.y * rapier_config.scale;

    commands
        .spawn_bundle(new_pb)
        .insert(RigidBodyBuilder::new_dynamic().translation(start_pos.x, start_pos.y))
        .insert(
            ColliderBuilder::ball(2.0)
                .sensor(true)
                .mass_properties(MassProperties::from_ball(5.0, 2.0)),
        );
}
