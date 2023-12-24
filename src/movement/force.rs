use bevy::prelude::*;
use bevy_rapier2d::dynamics::{ExternalForce, ReadMassProperties, Velocity};

use crate::components::Motion;

// Runs when the Motion component is modified, applying linear force to the entity in the direction
// specified by the Motion component
pub fn apply_forces_from_acceleration(
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
