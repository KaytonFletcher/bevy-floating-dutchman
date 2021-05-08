use bevy::prelude::*;

use bevy_rapier2d::{
    physics::{RapierConfiguration, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
};

use crate::{
    components::Motion,
    components::{Follow, Player, Track},
};

// Runs when the Track component is modified, applying angular velocity to the entity in the direction of the
// entity being tracked
pub fn update_tracking(
    query: Query<(&Track, &RigidBodyHandleComponent)>,
    rapier_parameters: Res<RapierConfiguration>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let elapsed = time.delta_seconds();
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

            // applies the scale factor between rapier physics and bevy physics
            // also applies a time delta to ensure the force is time agnostic
            let scaled_torque = torque * rapier_parameters.scale * elapsed;

            rb.apply_torque_impulse(scaled_torque, true);
        }
    }
}

// Runs when the Motion component is modified, applying linear force to the entity in the direction
// specified by the Motion component
pub fn update_movement(
    query: Query<(&Motion, &RigidBodyHandleComponent), Changed<Motion>>,
    rapier_parameters: Res<RapierConfiguration>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let elapsed = time.delta_seconds();
    for (motion, rigid_body) in query.iter() {
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            let force = motion.acceleration * rb.mass();
            let directed_force = motion.direction * force;

            // applies the scale factor between rapier physics and bevy physics
            // also applies a time delta to ensure the force is time agnostic
            let scaled_force = directed_force * rapier_parameters.scale * elapsed;

            rb.apply_force(scaled_force.into(), true);

            let scaled_max_velocity = motion.max_vel * rapier_parameters.scale;

            // ensures the velocity of rigid body does not exceed the specified entities max velocity
            // specifically when applying the force above ^^
            // (specified on the Motion component)
            rb.set_linvel(rb.linvel().cap_magnitude(scaled_max_velocity), false);
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
                    motion.direction = Vec2::new(x, y).normalize();
                }
            }
        }
    }
}
