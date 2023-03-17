use bevy::prelude::*;
use bevy_rapier2d::prelude::RigidBody;

use crate::components::{Motion, Projectile};

pub fn debug_projectiles(
    projectile_query: Query<(&Motion, &Transform, Entity), (With<RigidBody>, With<Projectile>)>,
) {
    for (motion, transform, entity) in projectile_query.iter() {
        println!(
            "Projectile with Entity: {:?} Position: {:?} Direction: {:?}",
            entity,
            transform.translation,
            motion.direction
        );
    }
}
