use bevy::prelude::*;

use bevy_rapier2d::physics::{RapierConfiguration, RigidBodyHandleComponent};
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
// use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::na::Vector2;

use crate::entities::Motion;

pub fn movement(
    mut query: Query<(&mut Transform, &mut Motion, &RigidBodyHandleComponent)>,
    rapier_parameters: Res<RapierConfiguration>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (mut transform, mut motion, rigid_body) in query.iter_mut() {
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

        let mut move_delta = Vector2::new(motion.velocity.x, motion.velocity.y);
        if move_delta != Vector2::zeros() {
            // Note that the RapierConfiguration::Scale factor is also used here to transform
            // the move_delta from: 'pixels/second' to 'physics_units/second'
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        // transform.translation.x += motion.velocity.x;
        // transform.translation.y += motion.velocity.y;

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            rb.set_linvel(move_delta * motion.max_vel, true);
        }
    }
}
