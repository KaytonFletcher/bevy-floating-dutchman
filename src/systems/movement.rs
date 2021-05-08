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

pub fn physics_dampening(
    query: Query<(&RigidBodyHandleComponent, &Motion)>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (rb_handle, motion) in query.iter() {
        let elapsed = time.delta_seconds();
        let rb = rigid_bodies.get_mut(rb_handle.handle()).unwrap();
        rb.set_angvel(rb.angvel() * 0.005f32.powf(elapsed), false);
        rb.set_linvel(rb.linvel().cap_magnitude(motion.max_vel * elapsed), false);
    }
}

pub fn update_movement(
    mut query: Query<(&mut Motion, &RigidBodyHandleComponent)>,
    rapier_parameters: Res<RapierConfiguration>,
    mut rigid_bodies: ResMut<RigidBodySet>,
) {
    for (mut motion, rigid_body) in query.iter_mut() {
        // if motion.acceleration.x == 0. {
        //     motion.velocity.x = 0.;
        // }

        // if motion.acceleration.y == 0. {
        //     motion.velocity.y = 0.;
        // }

        // motion.velocity.x += motion.acceleration.x;
        // motion.velocity.y += motion.acceleration.y;

        // let max_vel = if motion.acceleration.y != 0. && motion.acceleration.x != 0. {
        //     motion.max_vel * 0.707
        // } else {
        //     motion.max_vel
        // };

        // if motion.acceleration.x < 0. {
        //     motion.velocity.x = motion.velocity.x.max(-max_vel);
        // } else if motion.acceleration.x > 0. {
        //     motion.velocity.x = motion.velocity.x.min(max_vel);
        // }

        // if motion.acceleration.y < 0. {
        //     motion.velocity.y = motion.velocity.y.max(-max_vel);
        // } else if motion.acceleration.y > 0. {
        //     motion.velocity.y = motion.velocity.y.min(max_vel);
        // }

        // let force = Vec2::new(motion.velocity.x, motion.velocity.y) * rapier_parameters.scale;

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        if let Some(rb) = rigid_bodies.get_mut(rigid_body.handle()) {
            // println!("Applying force: {:?}", motion.direction * motion.force);
            rb.apply_force(
                (motion.direction * motion.force * rapier_parameters.scale).into(),
                true,
            );
        }
    }
}
