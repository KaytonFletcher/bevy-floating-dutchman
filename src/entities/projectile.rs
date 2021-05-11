use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::{RigidBody, RigidBodyBuilder},
    geometry::ColliderBuilder,
};
use nalgebra::ComplexField;

use crate::components::{Motion, ProjectileBundle};

pub fn spawn_projectile(
    commands: &mut Commands,
    spawn_body: &RigidBody,
    projectile_bundle: &ProjectileBundle,
) {
    let start_pos = spawn_body.position().translation;
    let mut new_pb = (*projectile_bundle).clone();

    let rot = spawn_body.position().rotation.angle();
    new_pb.motion.direction = Vec2::new(rot.sin(), rot.cos());

    commands
        .spawn_bundle(new_pb)
        .insert(RigidBodyBuilder::new_dynamic().translation(start_pos.x, start_pos.y))
        .insert(ColliderBuilder::ball(2.0));
}
