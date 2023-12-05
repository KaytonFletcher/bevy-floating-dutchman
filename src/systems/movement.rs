use bevy::prelude::*;

use bevy_rapier2d::prelude::{ExternalForce, ReadMassProperties, RigidBody, Velocity};

use crate::components::{Follow, Motion, Player, Track};

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
        external_force.force = if motion.is_moving {
            motion.get_direction_as_vec2() * motion.acceleration * mass_properties.get().mass
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
            motion.direction =
                follow.get_new_angle_to_follow(follower_transform, followed_transform);
        }
    }
}
