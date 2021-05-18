use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::components::{Motion, Projectile};

pub fn debug_projectiles(
    projectile_query: Query<(&Motion, &Projectile, &RigidBodyHandleComponent, Entity)>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for (motion, _, rb_handle, entity) in projectile_query.iter() {
        if let Some(rb) = rigid_bodies.get(rb_handle.handle()) {
            println!(
                "Projectile with ID: {:?} Position: {:?} Direction: {:?}",
                entity.id(),
                rb.position().translation.vector,
                motion.direction
            );
        }
    }
}
