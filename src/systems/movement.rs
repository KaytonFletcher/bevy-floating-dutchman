use bevy::prelude::*;

use bevy_rapier2d::physics::{RapierConfiguration, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::na::Vector2;

use crate::entities::Motion;

pub fn movement(
    mut query: Query<(&mut Motion, &RigidBodyHandleComponent)>,
    rapier_parameters: Res<RapierConfiguration>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (mut motion, rigid_body) in query.iter_mut() {
        if motion.acceleration.x == 0. {
            motion.velocity.x = 0.;
        }

        if motion.acceleration.y == 0. {
            motion.velocity.y = 0.;
        }

        motion.velocity.x += motion.acceleration.x;
        motion.velocity.y += motion.acceleration.y;

        let max_vel = if motion.acceleration.y != 0. && motion.acceleration.x != 0. {
            motion.max_vel * 0.707
        } else {
            motion.max_vel
        };

        if motion.acceleration.x < 0. {
            motion.velocity.x = motion.velocity.x.max(-max_vel);
        } else if motion.acceleration.x > 0. {
            motion.velocity.x = motion.velocity.x.min(max_vel);
        }

        if motion.acceleration.y < 0. {
            motion.velocity.y = motion.velocity.y.max(-max_vel);
        } else if motion.acceleration.y > 0. {
            motion.velocity.y = motion.velocity.y.min(max_vel);
        }

        let mut force = Vec2::new(motion.velocity.x, motion.velocity.y);

        force.normalize();
        force *= rapier_parameters.scale;

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            rb.apply_force(force.into(), true);
        }
    }
}
