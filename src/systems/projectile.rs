

pub fn player_shooting(
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