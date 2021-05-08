use bevy::prelude::*;

use bevy_rapier2d::{
    physics::{RapierConfiguration, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
};

use crate::{components::Motion, components::Track};

pub fn update_tracking(
    mut query: Query<(&Track, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (track, rigid_body) in query.iter_mut() {
        // always true
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            // angle between player position and last known mouse position
            let mut new_angle = (track.pos.y - rb.position().translation.vector.y)
                .atan2(track.pos.x - rb.position().translation.vector.x)
                + track.get_offset();

            // subtracts player angle to get the difference in angles
            new_angle -= rb.position().rotation.angle();

            rb.apply_torque_impulse(new_angle.sin() * track.rotate_speed, true);
        }
    }
}

pub fn update_movement(
    query: Query<(&Motion, &RigidBodyHandleComponent)>,
    rapier_parameters: Res<RapierConfiguration>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    let elapsed = time.delta_seconds();
    for (motion, rigid_body) in query.iter() {
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            // println!("Applying force: {:?}", motion.direction * motion.acceleration);
            rb.apply_force(
                (motion.direction * motion.acceleration * rb.mass() * rapier_parameters.scale * elapsed).into(),
                true,
            );

            rb.set_linvel(
                rb.linvel()
                    .cap_magnitude(motion.max_vel * rapier_parameters.scale),
                false,
            );
        }
    }
}
