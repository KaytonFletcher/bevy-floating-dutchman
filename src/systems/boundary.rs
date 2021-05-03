use bevy::prelude::*;

use bevy_rapier2d::{
    physics::RigidBodyHandleComponent,
    rapier::dynamics::RigidBodySet,
};

pub fn position_system(mut bodies: ResMut<RigidBodySet>, query: Query<&RigidBodyHandleComponent>) {
    for body_handle in &mut query.iter() {
        let body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position().translation.vector.x;
        let mut y = body.position().translation.vector.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = 1000.0 / 2.0;
        let half_height = 1000.0 / 2.0;
        if x < -half_width && body.linvel().x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && body.linvel().x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && body.linvel().y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && body.linvel().y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            println!("UPDATING");
            let mut new_position = body.position().clone();
            new_position.translation.vector.x = x;
            new_position.translation.vector.y = y;
            body.set_position(new_position, false);
        }
    }
}