use bevy::prelude::*;

use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::{
    components::Motion,
    components::{Follow, Player, Track},
};

// Runs when the Track component is modified, applying angular velocity to the entity in the direction of the
// entity being tracked
pub fn update_tracking(
    query: Query<(&Track, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (track, rigid_body) in query.iter() {
        // always true
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            // angle between entity (rigid body) being tracked and the entity given the Track component
            let mut new_angle = (track.pos.y - rb.position().translation.vector.y)
                .atan2(track.pos.x - rb.position().translation.vector.x)
                + track.get_offset();

            // subtracts tracked angle to get the difference in angles
            new_angle -= rb.position().rotation.angle();

            let torque = new_angle.sin() * track.rotate_acceleration * rb.mass();

            rb.apply_torque_impulse(torque, true);
        }
    }
}

pub fn tracking(
    mut trackers: Query<&mut Track, Without<Player>>,
    rb_query: Query<&RigidBodyHandleComponent>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for mut track in trackers.iter_mut() {
        if let Some(entity) = &track.entity_tracked {
            if let Ok(rb_comp) = rb_query.get_component::<RigidBodyHandleComponent>(*entity) {
                if let Some(rb) = rigid_bodies.get(rb_comp.handle()) {
                    track.pos.x = rb.position().translation.x;
                    track.pos.y = rb.position().translation.y;
                }
            }
        }
    }
}

// Runs when the Motion component is modified, applying linear force to the entity in the direction
// specified by the Motion component
pub fn update_movement(
    query: Query<(&Motion, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (motion, rigid_body) in query.iter() {
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            let force = motion.acceleration * rb.mass();
            let directed_force = motion.direction * force;

            rb.apply_force(directed_force.into(), true);

            // ensures the velocity of rigid body does not exceed the specified entities max velocity
            // specifically when applying the force above ^^
            // (specified on the Motion component)
            rb.set_linvel(rb.linvel().cap_magnitude(motion.max_vel), false);
        }
    }
}

pub fn follow(
    mut followers: Query<(&mut Motion, &Follow, &RigidBodyHandleComponent)>,
    rb_query: Query<&RigidBodyHandleComponent>,
    rigid_bodies: Res<RigidBodySet>,
) {
    for (mut motion, follow, rb_handle) in followers.iter_mut() {
        if let Some(follower_rb) = rigid_bodies.get(rb_handle.handle()) {
            if let Ok(rb_comp) = rb_query.get_component::<RigidBodyHandleComponent>(follow.entity) {
                if let Some(being_followed_rb) = rigid_bodies.get(rb_comp.handle()) {
                    let follower_pos = follower_rb.position().translation;
                    let being_followed_pos = being_followed_rb.position().translation;

                    // we calculate the x and y translation between the entity being followed
                    // and the entity following. This ensures the follower will move in the direction
                    // of the entity (Rigid body component) being followed
                    let x = being_followed_pos.x - follower_pos.x;
                    let y = being_followed_pos.y - follower_pos.y;

                    let new_follow_pos = Vec2::new(x, y);

                    let mut mult = 1.0;

                    if let Some(space) = follow.space {
                        if new_follow_pos.length() < space {
                            mult = -1.0;
                        }
                    }
                    motion.direction = Vec2::new(x, y).normalize() * mult;
                }
            }
        }
    }
}
