use std::f32::consts::PI;

use bevy::prelude::*;

use bevy_rapier2d::prelude::{ExternalForce, ReadMassProperties, RigidBody, Velocity};

use crate::{
    components::Motion,
    components::{Follow, Player, Track},
};

/**
 * Runs when the Track component is modified. Currently this just changes the entities transform to point
 * the entity marked on the Track component using Trigonometry. In the future we may want to use &mut ExternalForce
 * to apply a more "weighty" rotational feel rather than snapping to where the entity should look.
 * Move this to "Physics" GamePlay set if we decide to use forces here.
 */
pub fn update_rotation_based_on_tracking(
    mut query: Query<(&Track, &mut Transform), Changed<Track>>,
) {
    for (track, mut transform) in query.iter_mut() {
        // angle between entity (rigid body) being tracked and the entity given the Track component
        let new_angle = (track.pos.y - transform.translation.y)
            .atan2(track.pos.x - transform.translation.x)
            + track.get_offset();

        transform.rotation = Quat::from_rotation_z(new_angle);
    }
}

pub fn update_position_of_entity_tracked(
    mut trackers: Query<&mut Track, Without<Player>>,
    rb_query: Query<&Transform, (With<RigidBody>, Changed<Transform>)>,
) {
    for mut track in trackers.iter_mut() {
        if let Some(entity) = &track.entity_tracked {
            if let Ok(transform) = rb_query.get(*entity) {
                track.pos.x = transform.translation.x;
                track.pos.y = transform.translation.y;
            }
        }
    }
}

// Runs when the Motion component is modified, applying linear force to the entity in the direction
// specified by the Motion component
pub fn apply_forces(
    mut query: Query<
        (
            &Motion,
            &ReadMassProperties,
            &mut ExternalForce,
            &mut Velocity,
        ),
        Changed<Motion>,
    >,
) {
    for (motion, mass_properties, mut external_force, mut velocity) in query.iter_mut() {
        let rotation_matrix = Mat3::from_quat(motion.direction);

        let direction = rotation_matrix.x_axis.truncate().normalize();

        external_force.force = if motion.is_moving {
            direction * motion.acceleration * mass_properties.get().mass
        } else {
            Vec2::ZERO
        };

        // ensures the velocity of rigid body does not exceed the specified entities max velocity
        // specifically when applying the force above ^^
        // (specified on the Motion component)
        velocity.linvel = velocity.linvel.clamp_length_max(motion.max_vel);
    }
}

pub fn follow(
    mut followers: Query<(&mut Motion, &Follow, &Transform), With<RigidBody>>,
    rb_query: Query<&Transform, With<RigidBody>>,
) {
    for (mut motion, follow, follower_transform) in followers.iter_mut() {
        if let Ok(followed_transform) = rb_query.get(follow.entity) {
            let follower_pos = follower_transform.translation;
            let being_followed_pos = followed_transform.translation;

            // we calculate the x and y translation between the entity being followed
            // and the entity following. This ensures the follower will move in the direction
            // of the entity (Rigid body component) being followed
            let x = being_followed_pos.x - follower_pos.x;
            let y = being_followed_pos.y - follower_pos.y;

            let new_follow_pos = Vec2::new(x, y);

            // angle between entity (rigid body) being tracked and the entity given the Track component
            let mut new_angle = (new_follow_pos.y).atan2(new_follow_pos.x);

            if let Some(space) = follow.space {
                if new_follow_pos.length() < space {
                    new_angle = new_angle + PI;
                }
            }

            motion.direction = Quat::from_rotation_z(new_angle);
        }
    }
}
